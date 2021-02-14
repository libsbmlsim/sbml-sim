use std::collections::HashMap;

// An SBML Model container
#[derive(Debug)]
pub struct Model {
  pub tags: Vec<Tag>,
  pub root: TagIndex,
}

pub type TagIndex = usize;

#[derive(Debug)]
pub struct Tag {
  pub tag: String,
  pub text: String,
  pub attributes: HashMap<String, String>,
  pub children: Vec<TagIndex>,
}

impl Model {
  // returns a new SBML model
  pub fn new() -> Model {
    return Model {
      tags: Vec::new(),
      root: 0,
    };
  }

  // adds a node to the model and returns its index
  pub fn add_node(&mut self, tag: String) -> TagIndex {
    let index = self.tags.len();
    self.tags.push(Tag {
      tag: tag,
      text: String::from(""),
      attributes: HashMap::new(),
      children: Vec::new(),
    });
    return index;
  }

  pub fn add_text(&mut self, tag: TagIndex, text: String) {
    self.tags[tag].text = text;
  }

  pub fn add_attr(&mut self, tag: TagIndex, attr: String, value: String) {
    self.tags[tag].attributes.insert(attr, value);
  }

  pub fn add_child(&mut self, source: TagIndex, target: TagIndex) {
    self.tags[source].children.push(target);
  }
  
  pub fn get_tag_name(&self, index: TagIndex) -> &String {
    return &self.tags[index].tag;
  }

  pub fn get_text(&self, index: TagIndex) -> &String {
    return &self.tags[index].text;
  }
}
