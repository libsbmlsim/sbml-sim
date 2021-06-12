use std::env;
use std::fs::File;
use std::io::BufReader;

use xml;
use xml::reader::{EventReader, XmlEvent};

mod structs;
use sbml_simulator::proc_opening;
use structs::math::*;
use structs::model::*;

fn main() {
    // read cmd line args
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    // read file
    let file = File::open(filename).unwrap();
    let file = BufReader::new(file);
    let parser = EventReader::new(file);

    let mut stack: Vec<TagIndex> = Vec::new();
    let mut container = Vec::new();
    let mut container_len = 0;
    let model = Model::new();
    container.push(Tag::Model(model));
    container_len += 1;
    let mut current = 0;
    stack.push(current);
    println!("{:?}", current);

    for e in parser {
        match e {
            // for each starting tag
            Ok(XmlEvent::StartElement {
                name, attributes, ..
            }) => {
                let mut new_tag = None;
                match name.local_name.as_str() {
                    "listOfSpecies" => match container[current] {
                        Tag::Model(ref mut model) => {
                            let list_of_species = ListOfSpecies::new();
                            new_tag = Some(Tag::ListOfSpecies(list_of_species));
                            current = container_len;
                            model.list_of_species = Some(current.clone());
                            stack.push(current.clone());
                        }
                        _ => {}
                    },
                    "listOfReactions" => match container[current] {
                        Tag::Model(ref mut model) => {
                            let list_of_reactions = ListOfReactions::new();
                            new_tag = Some(Tag::ListOfReactions(list_of_reactions));
                            current = container_len;
                            model.list_of_reactions = Some(current.clone());
                            stack.push(current.clone());
                        }
                        _ => {}
                    },
                    "species" => match container[current] {
                        Tag::ListOfSpecies(ref mut list_of_species) => {
                            let mut species = Species::new();
                            for attribute in attributes {
                                if attribute.name.local_name == "name" {
                                    species.name = Some(attribute.value);
                                }
                            }
                            new_tag = Some(Tag::Species(species));
                            current = container_len;
                            list_of_species.species.push(current.clone());
                            stack.push(current.clone());
                        }
                        _ => {}
                    },
                    "reaction" => match container[current] {
                        Tag::ListOfReactions(ref mut list_of_reactions) => {
                            let reaction = Reaction::new();
                            new_tag = Some(Tag::Reaction(reaction));
                            current = container_len;
                            list_of_reactions.reactions.push(current.clone());
                            stack.push(current.clone());
                        }
                        _ => {}
                    },
                    "kineticLaw" => match container[current] {
                        Tag::Reaction(ref mut reaction) => {
                            let kinetic_law = KineticLaw::new();
                            new_tag = Some(Tag::KineticLaw(kinetic_law));
                            current = container_len;
                            reaction.kinetic_law = Some(current.clone());
                            stack.push(current.clone());
                        }
                        _ => {}
                    },
                    "math" => match container[current] {
                        Tag::KineticLaw(ref mut kinetic_law) => {
                            let math = MathTag::new_root();
                            new_tag = Some(Tag::MathTag(math));
                            current = container_len;
                            kinetic_law.math = Some(current.clone());
                            stack.push(current.clone());
                        }
                        _ => {}
                    },
                    "apply" => match container[current] {
                        Tag::MathTag(ref mut math_tag) => {
                            let apply = MathTag::new_apply();
                            new_tag = Some(Tag::MathTag(apply));
                            current = container_len;
                            match math_tag.node {
                                MathNode::Root(ref mut root) => {
                                    root.push(current.clone());
                                }
                                _ => {}
                            }
                            stack.push(current.clone());
                        }
                        _ => {}
                    },
                    _ => {}
                }
                match new_tag {
                    Some(t) => {
                        container.push(t);
                        container_len += 1;
                    }
                    None => {}
                }
            }
            // for each closing tag
            Ok(XmlEvent::EndElement { name }) => match name.local_name.as_str() {
                "listOfSpecies" => match container[current] {
                    Tag::ListOfSpecies(ref mut list_of_species) => {
                        stack.pop();
                        current = stack.last().unwrap().to_owned();
                        list_of_species.parent = Some(current.clone());
                    }
                    _ => {}
                },
                "listOfReactions" => match container[current] {
                    Tag::ListOfReactions(ref mut list_of_reactions) => {
                        stack.pop();
                        current = stack.last().unwrap().to_owned();
                        list_of_reactions.parent = Some(current.clone());
                    }
                    _ => {}
                },
                "species" => match container[current] {
                    Tag::Species(ref mut species) => {
                        stack.pop();
                        current = stack.last().unwrap().to_owned();
                        species.parent = Some(current.clone());
                    }
                    _ => {}
                },
                "reaction" => match container[current] {
                    Tag::Reaction(ref mut reaction) => {
                        stack.pop();
                        current = stack.last().unwrap().to_owned();
                        reaction.parent = Some(current.clone());
                    }
                    _ => {}
                },
                "kineticLaw" => match container[current] {
                    Tag::KineticLaw(ref mut kinetic_law) => {
                        stack.pop();
                        current = stack.last().unwrap().to_owned();
                        kinetic_law.parent = Some(current.clone());
                    }
                    _ => {}
                },
                "math" => match container[current] {
                    Tag::MathTag(ref mut math_tag) => {
                        stack.pop();
                        current = stack.last().unwrap().to_owned();
                        math_tag.parent = Some(current.clone());
                    }
                    _ => {}
                },
                "apply" => match container[current] {
                    Tag::MathTag(ref mut math_tag) => {
                        stack.pop();
                        current = stack.last().unwrap().to_owned();
                        math_tag.parent = Some(current.clone());
                    }
                    _ => {}
                },
                _ => {}
            },
            // read text within tags
            Ok(XmlEvent::Characters(s)) => {
                //model.add_text(current, String::from(s.trim()));
            }
            Err(e) => {
                println!("Error: {}", e);
            }
            _ => {}
        }
    }

    for item in container {
        println!("{:?}", item);
    }
    println!("{:?}", stack);
    println!("{:?}", current);
}
