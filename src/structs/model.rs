//use std::collections::HashMap;

pub type TagIndex = usize;
use super::math::*;

#[derive(Debug)]
pub enum Tag {
    Model(Model),
    ListOfSpecies(ListOfSpecies),
    ListOfReactions(ListOfReactions),
    Species(Species),
    Reaction(Reaction),
    KineticLaw(KineticLaw),
    MathTag(MathTag),
}

// An SBML Model container
#[derive(Debug)]
pub struct Model {
    pub name: Option<String>,
    pub list_of_species: Option<TagIndex>,
    pub list_of_reactions: Option<TagIndex>,
}
impl Model {
    // returns a new SBML model
    pub fn new() -> Model {
        return Model {
            name: None,
            list_of_species: None,
            list_of_reactions: None,
        };
    }
}

#[derive(Debug)]
pub struct ListOfSpecies {
    pub species: Vec<TagIndex>,
    pub parent: Option<TagIndex>,
}

impl ListOfSpecies {
    pub fn new() -> Self {
        return ListOfSpecies {
            species: Vec::new(),
            parent: None,
        };
    }
}

#[derive(Debug)]
pub struct ListOfReactions {
    pub reactions: Vec<TagIndex>,
    pub parent: Option<TagIndex>,
}

impl ListOfReactions {
    pub fn new() -> Self {
        return ListOfReactions {
            reactions: Vec::new(),
            parent: None,
        };
    }
}

#[derive(Debug)]
pub struct Species {
    pub name: Option<String>,
    pub compartment: Option<String>,
    pub parent: Option<TagIndex>,
}

impl Species {
    pub fn new() -> Self {
        return Species {
            name: None,
            compartment: None,
            parent: None,
        };
    }
}

#[derive(Debug)]
pub struct Reaction {
    pub kinetic_law: Option<TagIndex>,
    pub parent: Option<TagIndex>,
}

impl Reaction {
    pub fn new() -> Self {
        return Reaction {
            kinetic_law: None,
            parent: None,
        };
    }
}

#[derive(Debug)]
pub struct KineticLaw {
    pub math: Option<TagIndex>,
    pub parent: Option<TagIndex>,
}

impl KineticLaw {
    pub fn new() -> Self {
        return KineticLaw {
            math: None,
            parent: None,
        };
    }
}