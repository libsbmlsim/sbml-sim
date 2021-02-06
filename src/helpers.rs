use std::cell::RefCell;
use std::rc::Rc;

use crate::structs;
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
