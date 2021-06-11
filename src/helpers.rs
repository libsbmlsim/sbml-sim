//use crate::structs;
//use structs::math::MathExp;
//use structs::math::MathIndex;
//use structs::math::MathNode;
//use structs::math::Operator;
//use structs::model::Model;
//use structs::model::TagIndex;

//// pub fn print_model(model: &Model, root: Option<TagIndex>) {
////   match root {
////     Some(index) => {
////       println!("{}: {:?}", index, model.tags[index]);
////       for child in &model.tags[index].children {
////         print_model(&model, Some(*child));
////       }
////     }
////     None => {}
////   }
//// }

//// Input: Model, TagIndex
//// Searches the given tag and its children
//// Returns matches as a vector
//pub fn find(model: &Model, root: Option<TagIndex>, tag: String) -> Vec<TagIndex> {
//if root.is_none() {
//return Vec::new();
//}

//// children to be processed
//let mut stack: Vec<TagIndex> = Vec::new();
//// matching tags to be returned
//let mut results: Vec<TagIndex> = Vec::new();
//stack.push(root.unwrap());

//while !stack.is_empty() {
//let current = stack.pop().unwrap();
//if *model.get_tag_name(current) == tag {
//results.push(current);
//}
//for child in &model.tags[current].children {
//stack.push(*child);
//}
//}
//return results;
//}

//// Parses an SBMLTag into a MathExp
//pub fn parse_expression(model: &Model, root: TagIndex, exp: &mut MathExp) -> MathIndex {
//// store operand and operators
//let mut operator = Operator::None;
//let mut operands: Vec<MathIndex> = Vec::new();
//for child in &model.tags[root].children {
//match model.get_tag_name(*child) as &str {
//"times" => {
//operator = Operator::Mul;
//}
//"minus" => {
//operator = Operator::Sub;
//}
//"divide" => {
//operator = Operator::Div;
//}
//"plus" => {
//operator = Operator::Add;
//}
//"apply" => {
//// store operators in the expression and get their indices
//let child_exp = parse_expression(model, *child, exp);
//operands.push(child_exp);
//}
//"ci" => {
//let var_index = exp.get_var(model.get_text(*child));
//operands.push(var_index);
//}
//_ => {}
//}
//}

//// if there is no operand and only one operator
//// just return that operator
//if operator == Operator::None && operands.len() == 1 {
//// set root
//exp.root = operands[0];
//return operands[0];
//} else {
//// else create a branch and return it
//let index = exp.get_branch(operator);
//for operand in operands {
//exp.add_operand(index, operand);
//}
//// set root of expression
//// (this is the last step of the function,
////  so the first function call with set the
////  root when all other recursive calls are over)
//exp.root = index;
//return index;
//}
//}

//// Input: MathNode
//// Prints the contents in Reverse Polish notation
//pub fn print_postfix(expression: &MathExp, root: MathIndex) {
//match &expression.nodes[root] {
//MathNode::Branch { operator, operands } => {
//// keep track of operands printed
//// print operator after every two operands
//let mut count = 0;
//for operand in operands {
//print_postfix(expression, *operand);
//count += 1;
//if count == 2 {
//print!("{} ", operator);
//count = 0;
//}
//}
//// print operator again if required
//if count == 1 {
//print!("{} ", operator);
//}
//}
//MathNode::Var(var) => {
//print!("{} ", var);
//}
//}
//}
