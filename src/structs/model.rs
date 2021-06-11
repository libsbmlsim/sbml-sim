//use std::collections::HashMap;

// An SBML Model container
#[derive(Debug)]
pub struct Model<'a> {
    pub name: Option<String>,
    pub list_of_species: Option<ListOfSpecies<'a>>,
    //pub tags: Vec<Tag>,
    //pub root: TagIndex,
}

#[derive(Debug)]
pub struct ListOfSpecies<'a> {
    pub species: Vec<Species<'a>>,
    pub parent: Option<ListOfSpeciesParent<'a>>,
}

#[derive(Debug)]
pub enum ListOfSpeciesParent<'a> {
    Model(&'a Model<'a>),
}

#[derive(Debug)]
pub struct Species<'a> {
    pub id: String,
    pub name: Option<String>,
    pub parent: Option<SpeciesParent<'a>>,
}

#[derive(Debug)]
pub enum SpeciesParent<'a> {
    ListOfSpecies(&'a ListOfSpecies<'a>),
}
#[derive(Debug)]
pub enum Refs<'a> {
    Model(&'a mut Model<'a>),
    ListOfSpecies(&'a ListOfSpecies<'a>),
    Species(&'a Species<'a>),
}

//pub type TagIndex = usize;

//#[derive(Debug)]
//pub struct Tag {
//pub tag: String,
//pub text: String,
//pub attributes: HashMap<String, String>,
//pub children: Vec<TagIndex>,
//}

impl Model<'_> {
    // returns a new SBML model
    pub fn new() -> Model<'static> {
        return Model {
            name: None,
            list_of_species: None,
            //tags: Vec::new(),
            //root: 0,
        };
    }
}

impl<'a> ListOfSpecies<'a> {
    pub fn new() -> ListOfSpecies<'static> {
        return ListOfSpecies {
            species: Vec::new(),
            parent: None,
        };
    }

    pub fn with_parent(mut self, parent: Refs<'a>) -> Self {
        match parent {
            Refs::Model(p) => {
                self.parent = Some(ListOfSpeciesParent::Model(p));
            }
            _ => {}
        }
        self
    }
}

// adds a node to the model and returns its index
//pub fn add_node(&mut self, tag: String) -> TagIndex {
//let index = self.tags.len();
//self.tags.push(Tag {
//tag: tag,
//text: String::from(""),
//attributes: HashMap::new(),
//children: Vec::new(),
//});
//return index;
//}

//pub fn add_text(&mut self, tag: TagIndex, text: String) {
//self.tags[tag].text = text;
//}

//pub fn add_attr(&mut self, tag: TagIndex, attr: String, value: String) {
//self.tags[tag].attributes.insert(attr, value);
//}

//pub fn add_child(&mut self, source: TagIndex, target: TagIndex) {
//self.tags[source].children.push(target);
//}

//pub fn get_tag_name(&self, index: TagIndex) -> &String {
//return &self.tags[index].tag;
//}

//pub fn get_text(&self, index: TagIndex) -> &String {
//return &self.tags[index].text;
//}
//}
