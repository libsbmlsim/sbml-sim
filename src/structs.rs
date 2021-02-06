use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
pub struct SBMLTag {
  pub tag: String,
  pub attributes: HashMap<String, String>,
  pub children: Vec<Rc<RefCell<SBMLTag>>>,
}

impl SBMLTag {
  pub fn new() -> SBMLTag {
    SBMLTag {
      tag: String::from(""),
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