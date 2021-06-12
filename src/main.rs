use std::env;
use std::fs::File;
use std::io::BufReader;

use xml;
use xml::reader::{EventReader, XmlEvent};
#[allow(unused_variables)]
mod structs;
use sbml_simulator::{attach, close, push};
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
                    "listOfSpecies" => attach!(ListOfSpecies to Model),
                    "listOfReactions" => attach!(ListOfReactions to Model),
                    "species" => push!(Species with name, compartment into ListOfSpecies),
                    "reaction" => push!(Reaction into ListOfReactions),
                    "kineticLaw" => attach!(KineticLaw to Reaction),
                    _ => {}
                }
                //"math" => match container[current] {
                //Tag::KineticLaw(ref mut kinetic_law) => {
                //let math = MathTag::new_root();
                //new_tag = Some(Tag::MathTag(math));
                //current = container_len;
                //kinetic_law.math = Some(current.clone());
                //stack.push(current.clone());
                //}
                //_ => {}
                //},
                //"apply" => match container[current] {
                //Tag::MathTag(ref mut math_tag) => {
                //let apply = MathTag::new_apply();
                //new_tag = Some(Tag::MathTag(apply));
                //current = container_len;
                //match math_tag.node {
                //MathNode::Root(ref mut root) => {
                //root.push(current.clone());
                //}
                //_ => {}
                //}
                //stack.push(current.clone());
                //}
                //_ => {}
                //},
                //}
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
                "listOfSpecies" => close![ListOfSpecies],
                "listOfReactions" => close![ListOfReactions],
                "species" => close![Species],
                "reaction" => close![Reaction],
                "kineticLaw" => close![KineticLaw],
                //"math" => match container[current] {
                //Tag::MathTag(ref mut math_tag) => {
                //stack.pop();
                //current = stack.last().unwrap().to_owned();
                //math_tag.parent = Some(current.clone());
                //}
                //_ => {}
                //},
                //"apply" => match container[current] {
                //Tag::MathTag(ref mut math_tag) => {
                //stack.pop();
                //current = stack.last().unwrap().to_owned();
                //math_tag.parent = Some(current.clone());
                //}
                //_ => {}
                //},
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
