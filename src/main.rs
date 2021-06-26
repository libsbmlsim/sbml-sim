use mathml_rs::methods::evaluate::*;
//use mathml_rs::Op;
use sbml_rs;
//use sbml_rs::MathNode;
use sbml_rs::Tag;
//use sbml_rs::TagIndex;
use std::collections::HashMap;
use std::env;

fn main() {
    let mut args = env::args();
    let filename = args
        .nth(1)
        .expect("Please provide the filename of an SBML model as a command line argument.")
        .to_owned();
    let model = sbml_rs::parse(&filename).expect("Couldn't parse model.");
    for tag in &model {
        print!("{}", tag);
    }

    let mut hm: HashMap<String, f64> = HashMap::new();
    hm.insert("hello".into(), 55.0);

    // Evaluate math nodes
    for tag in &model {
        match tag {
            Tag::MathTag(math_tag) => {
                println!("{}", evaluate_node(&math_tag.nodes, 0, &hm).unwrap())
            }
            _ => {}
        }
    }
}
