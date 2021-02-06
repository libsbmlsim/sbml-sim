use std::cell::RefCell;
use std::rc::Rc;

use crate::structs;
use structs::MathNode;
use structs::Operator;
use structs::SBMLTag;

pub fn new_tag() -> Rc<RefCell<SBMLTag>> {
  return Rc::new(RefCell::new(SBMLTag::new()));
}

pub fn find(root: Rc<RefCell<SBMLTag>>, tag: String) -> Vec<Rc<RefCell<SBMLTag>>> {
  let mut stack: Vec<Rc<RefCell<SBMLTag>>> = Vec::new();
  let mut results: Vec<Rc<RefCell<SBMLTag>>> = Vec::new();
  stack.push(Rc::clone(&root));

  while !stack.is_empty() {
    let current = stack.pop().unwrap();
    if current.borrow().tag == tag {
      results.push(Rc::clone(&current));
    }
    for child in &current.borrow().children {
      stack.push(Rc::clone(&child));
    }
  }
  return results;
}

pub fn new_math_var(s: String) -> Rc<RefCell<MathNode>> {
  return Rc::new(RefCell::new(MathNode::new_var(s)));
}

pub fn parse_expression(expr: Rc<RefCell<SBMLTag>>) -> Rc<RefCell<MathNode>> {
  let mut operator = Operator::None;
  let mut operands: Vec<Rc<RefCell<MathNode>>> = Vec::new();
  for child in &expr.borrow().children {
    match &child.borrow().tag as &str {
      "times" => {
        operator = Operator::Mul;
      }
      "minus" => {
        operator = Operator::Sub;
      }
      "divide" => {
        operator = Operator::Div;
      }
      "plus" => {
        operator = Operator::Add;
      }
      "apply" => {
        operands.push(parse_expression(Rc::clone(child)));
      }
      "ci" => {
        operands.push(new_math_var(child.borrow().text.clone()));
      }
      _ => {}
    }
  }

  // if there is no operand and only one operator
  // just return that operator
  if operator == Operator::None && operands.len() == 1 {
    return Rc::clone(&operands[0]);
  } else {
    // else return the whole thing
    let node = MathNode::Branch { operator, operands };
    return Rc::new(RefCell::new(node));
  }
}

pub fn print_postfix(expression: Rc<RefCell<MathNode>>) {
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
