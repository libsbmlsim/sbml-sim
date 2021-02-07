use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

// A genetic SBML Tag
#[derive(Debug)]
pub struct SBMLTag {
  pub tag: String,
  pub text: String,
  pub attributes: HashMap<String, String>,
  pub children: Vec<Rc<RefCell<SBMLTag>>>,
}

impl SBMLTag {
  // returns an empty SBML tag
  pub fn new() -> SBMLTag {
    SBMLTag {
      tag: String::from(""),
      text: String::from(""),
      attributes: HashMap::new(),
      children: Vec::new()
    }
  }

  // Adds an attribute to an SBML tag
  pub fn add_attr(&mut self, attr: String, value: String) -> &mut SBMLTag {
    self.attributes.insert(attr, value);
    self
  }

  // Adds a child to an SBML tag
  pub fn add_child(&mut self, child: Rc<RefCell<SBMLTag>>) -> &mut SBMLTag {
    self.children.push(child);
    self
  }
}

// A genetic node in mathematical expressions
// Types: Variable and Branch
// Variable: String
// Branch consists of an operator and a list of 
// operands, which themselves are MathNodes.
#[derive(Debug)]
pub enum MathNode {
  Var(String),
  Branch {
    operator: Operator,
    operands: Vec<Rc<RefCell<MathNode>>>,
  },
}

impl MathNode {
  // returns a new MathNode of Variable type
  // with the given string
  pub fn new_var(s: String) -> MathNode {
    return MathNode::Var(s);
  }
}

// A genetic mathematical operator
#[derive(Debug, PartialEq)]
pub enum Operator {
  Add,
  Sub,
  Mul,
  Div,
  None,
}

// Dictates how to print mathematical operators
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
