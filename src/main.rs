use std::env;
use std::str;

use quick_xml::events::Event;
use quick_xml::Reader;

mod structs;
use sbml_simulator::{attach, close, push};
use structs::math::*;
use structs::model::*;

#[allow(unused_variables, unused_assignments)]
fn main() {
    // read cmd line args
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    // read file
    //let file = File::open().unwrap();
    let mut reader = Reader::from_file(filename).expect("File error.");
    reader.trim_text(true);
    reader.expand_empty_elements(true);
    let mut buf = Vec::new();
    let mut txt = Vec::new();

    let mut stack: Vec<TagIndex> = Vec::new();
    let mut container = Vec::new();
    let mut container_len = 0;

    let model = Model::new();
    container.push(Tag::Model(model));
    container_len += 1;
    let mut current = 0;
    stack.push(current);
    println!("{:?}", current);

    loop {
        match reader.read_event(&mut buf) {
            // for each starting tag
            Ok(Event::Start(ref e)) => {
                let mut new_tag = None;
                match e.name() {
                    b"listOfSpecies" => attach!(ListOfSpecies to Model),
                    b"listOfReactions" => attach!(ListOfReactions to Model),
                    b"species" => {
                        push!(Species with 
                                name as String, 
                                compartment as String 
                            into ListOfSpecies)
                    }
                    b"reaction" => push!(Reaction into ListOfReactions),
                    b"kineticLaw" => attach!(KineticLaw to Reaction),
                    b"math" => match container[current] {
                        Tag::KineticLaw(ref mut kinetic_law) => {
                            let math = MathTag::new_root();
                            new_tag = Some(Tag::MathTag(math));
                            current = container_len;
                            kinetic_law.math = Some(current.clone());
                            stack.push(current.clone());
                        }
                        _ => {}
                    },
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
                    //}
                    _ => {
                        println!("Tag not parsed: {}", str::from_utf8(e.name()).unwrap());
                    }
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
            Ok(Event::End(ref e)) => match e.name() {
                b"listOfSpecies" => close![ListOfSpecies],
                b"listOfReactions" => close![ListOfReactions],
                b"species" => close![Species],
                b"reaction" => close![Reaction],
                b"kineticLaw" => close![KineticLaw],
                b"math" => close![MathTag],
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
            // unescape and decode the text event using the reader encoding
            Ok(Event::Text(e)) => txt.push(e.unescape_and_decode(&reader).unwrap()),
            Ok(Event::Eof) => break, // exits the loop when reaching end of file
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (), // There are several other `Event`s we do not consider here
        }
    }
    for item in container {
        println!("{:?}", item);
    }
    println!("{:?}", stack);
    println!("{:?}", current);
}
