use mathml_rs::MathNode;
use sbml_rs::{self, Model};
use std::collections::HashMap;

use crate::LocalParameter;

use super::bindings::random_string;

#[derive(Debug, Clone)]
pub struct Reaction {
    pub id: String,
    pub reversible: bool,
    pub compartment: Option<String>,
    pub sbo_term: Option<String>,
    pub reactants: HashMap<String, SpeciesReference>,
    pub unbound_reactants: HashMap<String, UnboundSpeciesReference>,
    pub products: HashMap<String, SpeciesReference>,
    pub unbound_products: HashMap<String, UnboundSpeciesReference>,
    pub modifiers: Vec<String>,
    pub kinetic_law: Vec<MathNode>,
    pub local_parameters: HashMap<String, LocalParameter>,
}

impl Reaction {
    pub fn from(sbml_reaction: &sbml_rs::Reaction, model: &Model) -> Self {
        let id = sbml_reaction
            .id
            .as_ref()
            .expect("No ID found for Reaction.")
            .to_owned();
        let reversible = sbml_reaction
            .reversible
            .expect("Reversible attribute is mandatory on Reactions.");
        let mut compartment = None;
        if let Some(value) = &sbml_reaction.compartment {
            compartment = Some(value.to_owned());
        }
        let mut sbo_term = None;
        if let Some(value) = &sbml_reaction.sbo_term {
            sbo_term = Some(value.to_owned());
        }

        let mut reaction = Reaction {
            id,
            reversible,
            compartment,
            sbo_term,
            reactants: HashMap::new(),
            unbound_reactants: HashMap::new(),
            products: HashMap::new(),
            unbound_products: HashMap::new(),
            modifiers: Vec::new(),
            kinetic_law: Vec::new(),
            local_parameters: HashMap::new(),
        };

        reaction.parse_reactants(sbml_reaction, model);
        reaction.parse_products(sbml_reaction, model);
        reaction.parse_modifiers(sbml_reaction, model);
        reaction.parse_kinetic_law(sbml_reaction, model);
        reaction.parse_local_parameters(sbml_reaction, model);

        reaction
    }

    pub fn parse_reactants(&mut self, reaction: &sbml_rs::Reaction, model: &Model) {
        for reactant in &reaction.reactants(model) {
            if let Some(id) = &reactant.species {
                match SpeciesReference::from(&reactant) {
                    Ok(bound_reactant) => {
                        self.reactants.insert(id.clone(), bound_reactant);
                    }
                    Err(unbound_reactant) => {
                        self.unbound_reactants.insert(id.clone(), unbound_reactant);
                    }
                }
            }
        }
    }

    pub fn parse_products(&mut self, reaction: &sbml_rs::Reaction, model: &Model) {
        for product in &reaction.products(model) {
            if let Some(id) = &product.species {
                match SpeciesReference::from(&product) {
                    Ok(product) => {
                        self.products.insert(id.clone(), product);
                    }
                    Err(unbound_product) => {
                        self.unbound_products.insert(id.clone(), unbound_product);
                    }
                }
            }
        }
    }

    pub fn parse_modifiers(&mut self, reaction: &sbml_rs::Reaction, model: &Model) {
        for modifier in reaction.modifiers(model) {
            self.modifiers.push(modifier);
        }
    }

    pub fn parse_kinetic_law(&mut self, reaction: &sbml_rs::Reaction, model: &Model) {
        if let Some(kinetic_law) = reaction.kinetic_law(model) {
            self.kinetic_law = kinetic_law.nodes;
        } else {
            panic!("Can't simulate a reaction without a kinetic law.")
        }
    }

    pub fn parse_local_parameters(&mut self, reaction: &sbml_rs::Reaction, model: &Model) {
        for local_parameter in reaction.local_parameters(model) {
            if let Some(id) = &local_parameter.id {
                self.local_parameters
                    .insert(id.clone(), LocalParameter::from(&local_parameter));
            }
        }
    }

    pub fn local_parameter_values(&self) -> HashMap<String, f64> {
        let mut hm: HashMap<String, f64> = HashMap::new();
        for (id, local_parameter) in &self.local_parameters {
            hm.insert(id.clone(), local_parameter.value);
        }
        hm
    }
}

#[derive(Debug, Clone)]
pub struct UnboundSpeciesReference {
    pub id: String,
    pub constant: bool,
    pub species: String,
    pub sbo_term: Option<String>,
}

impl UnboundSpeciesReference {
    pub fn from(sp_ref: &sbml_rs::SpeciesReference) -> Self {
        let species = sp_ref
            .species
            .clone()
            .expect("Species attribute is mandatory on SpeciesReferences.");
        let id;
        if let Some(value) = &sp_ref.id {
            id = value.to_owned();
        } else {
            id = species.clone() + "_stoich_" + &random_string();
            //dbg!(&id);
        }
        let constant = sp_ref
            .constant
            .expect("Constant attribute is mandatory on SpeciesReferences.");
        let mut sbo_term = None;
        if let Some(value) = &sp_ref.sbo_term {
            sbo_term = Some(value.to_owned());
        }

        UnboundSpeciesReference {
            id,
            constant,
            species,
            sbo_term,
        }
    }

    pub fn to_bound(&self, stoichiometry: f64) -> SpeciesReference {
        SpeciesReference {
            id: self.id.clone(),
            constant: self.constant,
            species: self.species.clone(),
            sbo_term: self.sbo_term.clone(),
            stoichiometry,
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub struct SpeciesReference {
    pub id: String,
    pub stoichiometry: f64,
    pub constant: bool,
    pub species: String,
    pub sbo_term: Option<String>,
}

impl SpeciesReference {
    pub fn from(sp_ref: &sbml_rs::SpeciesReference) -> Result<Self, UnboundSpeciesReference> {
        let unbound = UnboundSpeciesReference::from(sp_ref);
        if let Some(stoich) = sp_ref.stoichiometry {
            Ok(unbound.to_bound(stoich))
        } else {
            Err(unbound)
        }
    }
}

#[derive(Debug)]
// used in a reaction matrix
// specifies whether a particular species
// is a reactant or a product in a particular reaction
// along with its stoichiometry
pub enum SpeciesStatus {
    Reactant(String),
    Product(String),
    None,
}

impl Default for SpeciesStatus {
    fn default() -> Self {
        SpeciesStatus::None
    }
}
