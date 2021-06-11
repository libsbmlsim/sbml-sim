///
//pub type TagIndex = usize;
use super::model::TagIndex;

#[derive(Debug)]
pub enum Op {
    Plus,
    Minus,
    Times,
}

#[derive(Debug)]
pub struct Math {
    pub operation: Option<Op>,
    pub parent: Option<TagIndex>,
}

impl Math {
    pub fn new() -> Self {
        return Math {
            operation: None,
            parent: None,
        };
    }
}
//// A genetic mathematical operator
//#[derive(Debug, PartialEq)]
//pub enum Operator {
//Add,
//Sub,
//Mul,
//Div,
//None,
//}

//// Dictates how to print mathematical operators
//impl fmt::Display for Operator {
//fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//match self {
//Operator::Add => {
//return write!(f, "+");
//}
//Operator::Sub => {
//return write!(f, "-");
//}
//Operator::Mul => {
//return write!(f, "*");
//}
//Operator::Div => {
//return write!(f, "/");
//}
//Operator::None => {
//return write!(f, "");
//}
//}
//}
//}
