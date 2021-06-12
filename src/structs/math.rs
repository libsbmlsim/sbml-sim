///
//pub type TagIndex = usize;
use super::model::TagIndex;

#[derive(Debug)]
pub struct MathTag {
    pub node: MathNode,
    pub parent: Option<TagIndex>,
}

impl MathTag {
    pub fn new_root() -> Self {
        return MathTag {
            node: MathNode::Root(Vec::new()),
            parent: None,
        };
    }
    pub fn new_apply() -> Self {
        return MathTag {
            node: MathNode::Apply(Vec::new()),
            parent: None,
        };
    }
}

#[derive(Debug)]
pub enum MathNode {
    Apply(Vec<TagIndex>),
    Op(Op),
    Text(String),
    Root(Vec<TagIndex>),
    Ci(Vec<TagIndex>),
    //Csymbol {
    //definition_url: String,
    //encoding: Option<String>,
    //children: Vec<MathNode>,
    //},
    Cn { num_type: NumType, value: String },
    Comment(String),
    PI(String, Option<String>),
}

#[derive(Debug)]
pub enum NumType {
    Real(f64),
    Integer(i64),
    Rational(i64, i64),
    ComplexCartesian(f64, f64),
    ComplexPolar(f64, f64),
    Constant(String),
    ENotation(f64, i64),
}
#[derive(Debug)]
pub enum Op {
    Plus,
    Minus,
    Times,
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
