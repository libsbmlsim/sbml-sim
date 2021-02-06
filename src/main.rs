use std::cell::RefCell;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::rc::Rc;

use xml;
use xml::reader::{EventReader, XmlEvent};

mod structs;
use structs::MathNode;
use structs::Operator;
use structs::SBMLTag;

mod helpers;
use helpers::new_tag;
use helpers::parse_expression;

fn main() {
  let args: Vec<String> = env::args().collect();
  let filename = &args[1];
  let file = File::open(filename).unwrap();
  let file = BufReader::new(file);

  let parser = EventReader::new(file);

  let mut stack: Vec<Rc<RefCell<SBMLTag>>> = Vec::new();
  let mut current = new_tag();

  for e in parser {
    match e {
      // for each starting tag
      Ok(XmlEvent::StartElement {
        name, attributes, ..
      }) => {
        // println!("{:?}", name);
        // read tag
        let tag = new_tag();
        tag.borrow_mut().tag = name.local_name;
        // read attributes
        for attribute in attributes {
          tag
            .borrow_mut()
            .add_attr(attribute.name.local_name, attribute.value);
        }

        // append to current tag and advance
        current.borrow_mut().add_child(Rc::clone(&tag));
        current = tag;
        // push to stack
        stack.push(Rc::clone(&current));
      }
      // for each closing tag
      Ok(XmlEvent::EndElement { name }) => {
        // println!("{:?}", name);
        // read tag name
        let tag = name.local_name;
        // if this is the last tag in the stack
        if current.borrow().tag == tag {
          // pop out and advance
          if stack.len() > 1 {
            stack.pop();
            current = Rc::clone(&stack[stack.len() - 1]);
          } else if stack.len() > 0 {
            stack.pop();
          }
        }
      }
      Ok(XmlEvent::Characters(s)) => {
        current.borrow_mut().text = String::from(s.trim());
      }
      Err(e) => {
        println!("Error: {}", e);
      }
      _ => {}
    }
  }

  let root = current;

  // print species IDs
  println!("Species IDs: ");
  let results = helpers::find(Rc::clone(&root), String::from("species"));
  for result in results {
    print!("{}  ", result.borrow().attributes.get("id").unwrap());
  }

  // find math nodes
  let mut expressions: Vec<Rc<RefCell<MathNode>>> = Vec::new();
  let math_nodes = helpers::find(Rc::clone(&root), String::from("math"));
  for expression in math_nodes {
    expressions.push(parse_expression(expression));
  }

  println!();
  println!();
  println!("Math nodes in Reverse Polish notation:");
  for expression in expressions {
    print_postfix(expression);
    println!();
  }
}

fn print_postfix(expression: Rc<RefCell<MathNode>>) {
  match &*expression.borrow() {
    MathNode::Branch { operator, operands } => {
      let mut count = 0;
      // print operator after every two operands
      for operand in operands {
        match &*operand.borrow() {
          MathNode::Var(s) => {
            print!("{} ", s);
          }
          MathNode::Branch { .. } => {
            print_postfix(Rc::clone(operand));
          }
        }
        count += 1;
        if count == 2 {
          match operator {
            Operator::None => {}
            _ => {
              print!("{} ", operator)
            }
          }
          count = 0;
        }
      }
      if count == 1 {
        match operator {
          Operator::None => {}
          _ => {
            print!("{} ", operator)
          }
        }
      }
    }
    _ => {}
  }
}
