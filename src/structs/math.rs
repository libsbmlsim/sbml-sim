use std::fmt;

pub type MathIndex = usize;

// A mathematical expression
#[derive(Debug)]
pub struct MathExp {
  pub root: MathIndex,
  pub nodes: Vec<MathNode>,
}

// A generic node in mathematical expressions
// Types: Variable and Branch
// Variable: String
// Branch consists of an operator and a list of
// operands, which themselves are MathNodes.
#[derive(Debug)]
pub enum MathNode {
  Var(String),
  Branch {
    operator: Operator,
    operands: Vec<MathIndex>,
  },
}

impl MathExp {
  // returns a new empty math expression
  pub fn new() -> MathExp {
    return MathExp {
      nodes: Vec::new(),
      root: 0,
    };
  }

  // adds a branch to mathexp and returns its index
  pub fn get_branch(&mut self, op: Operator) -> MathIndex {
    self.nodes.push(MathNode::Branch {
      operator: op,
      operands: Vec::new(),
    });
    return self.nodes.len() - 1;
  }

  // adds a variable to mathexp and returns its index
  // TODO: Check for duplicates here
  pub fn get_var(&mut self, var: &String) -> MathIndex {
    self.nodes.push(MathNode::Var(var.to_string()));
    return self.nodes.len() - 1;
  }

  // adds an operand to a branch
  pub fn add_operand(&mut self, branch_idx: MathIndex, op_idx: MathIndex) {
    match &mut self.nodes[branch_idx] {
      MathNode::Var(..) => {}
      MathNode::Branch { operands, .. } => {
        operands.push(op_idx);
      }
    }
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
