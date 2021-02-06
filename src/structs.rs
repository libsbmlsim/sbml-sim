use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

#[derive(Debug)]
pub struct SBMLTag {
  pub tag: String,
  pub text: String,
  pub attributes: HashMap<String, String>,
  pub children: Vec<Rc<RefCell<SBMLTag>>>,
}

impl SBMLTag {
  pub fn new() -> SBMLTag {
    SBMLTag {
      tag: String::from(""),
      text: String::from(""),
      attributes: HashMap::new(),
      children: Vec::new()
    }
  }

  pub fn add_attr(&mut self, attr: String, value: String) -> &mut SBMLTag {
    self.attributes.insert(attr, value);
    self
  }

  pub fn add_child(&mut self, child: Rc<RefCell<SBMLTag>>) -> &mut SBMLTag {
    self.children.push(child);
    self
  }
}

#[derive(Debug)]
pub enum MathNode {
  Var(String),
  Branch {
    operator: Operator,
    operands: Vec<Rc<RefCell<MathNode>>>,
  },
}

impl MathNode {
  pub fn new_var(s: String) -> MathNode {
    return MathNode::Var(s);
  }
}

#[derive(Debug, PartialEq)]
pub enum Operator {
  Add,
  Sub,
  Mul,
  Div,
  None,
}

impl fmt::Display for Operator {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Operator::Add => {
        return write!(f, "+");
      }
      Operator::Sub => {
        return write!(f, "-");
      }
      Operator::Mul => {
        return write!(f, "*");
      }
      Operator::Div => {
        return write!(f, "/");
      }
      Operator::None => {
        return write!(f, "");
      }
      
    }
  }
}
