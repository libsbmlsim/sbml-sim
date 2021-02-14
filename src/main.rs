use std::env;
use std::fs::File;
use std::io::BufReader;

use xml;
use xml::reader::{EventReader, XmlEvent};

mod structs;
use structs::model::Model;
use structs::model::TagIndex;
use structs::math::MathExp;

mod helpers;
use helpers::parse_expression;
use helpers::print_postfix;

fn main() {
  // read cmd line args
  let args: Vec<String> = env::args().collect();
  let filename = &args[1];

  // read file
  let file = File::open(filename).unwrap();
  let file = BufReader::new(file);
  let parser = EventReader::new(file);

  let mut stack: Vec<TagIndex> = Vec::new();
  let mut model = Model::new();
  let mut current: TagIndex = model.add_node(String::from("root"));

  for e in parser {
    match e {
      // for each starting tag
      Ok(XmlEvent::StartElement {
        name, attributes, ..
      }) => {
        // read tag
        let tag = model.add_node(String::from(name.local_name));
        // read attributes
        for attribute in attributes {
          model.add_attr(tag, attribute.name.local_name, attribute.value);
        }
        // append to current tag and advance
        model.add_child(current, tag);
        current = tag;
        // push to stack
        stack.push(tag);
      }
      // for each closing tag
      Ok(XmlEvent::EndElement { name }) => {
        // read tag name
        let tag = name.local_name;
        // if this is the last tag in the stack
        if *model.get_tag_name(current) == tag {
          // pop out and advance
          if stack.len() > 1 {
            stack.pop();
            current = stack[stack.len() - 1];
          } else if stack.len() > 0 {
            stack.pop();
          }
        }
      }
      // read text within tags
      Ok(XmlEvent::Characters(s)) => {
        model.add_text(current, String::from(s.trim()));
      }
      Err(e) => {
        println!("Error: {}", e);
      }
      _ => {}
    }
  }

  model.root = current;
  let root = model.root;

  // print species IDs
  println!("Species IDs: ");
  let results = helpers::find(&model, Some(root), String::from("species"));
  for result in results {
    print!("{}  ", model.tags[result].attributes.get("id").unwrap());
  }
  println!();

  // find kinetic laws
  let kinetic_laws = helpers::find(&model, Some(root), String::from("kineticLaw"));
  // find math nodes
  let mut expressions: Vec<MathExp> = Vec::new();
  for law in kinetic_laws {
    let math_nodes = helpers::find(&model, Some(law), String::from("math"));
    // parse expressions and store
    for expression in math_nodes {
      let mut parsed_expr = MathExp::new();
      parse_expression(&model, expression, &mut parsed_expr);
      expressions.push(parsed_expr);
    }
  }

  println!();
  // print expressions
  println!("Math nodes in Reverse Polish notation:");
  for expression in expressions {
    print_postfix(&expression, expression.root);
    println!();
  }
}
