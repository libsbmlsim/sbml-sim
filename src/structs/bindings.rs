use mathml_rs::{evaluate_node, MathNode};
use sbml_rs::Model;
use sbml_rs::{self, SpeciesStatus};

use crate::{
    AssignmentRule, Compartment, InitialAssignment, ODETerm, Parameter, RateRule, Reaction,
    Species, UnboundCompartment, UnboundParameter, UnboundSpecies, ODE,
};
use std::collections::HashMap;

#[derive(Debug, Clone)]
#[allow(non_snake_case)]
pub struct Bindings {
    //units: HashMap<String, Unit>,
    pub compartments: HashMap<String, Compartment>,
    pub unbound_compartments: HashMap<String, UnboundCompartment>,
    pub parameters: HashMap<String, Parameter>,
    pub unbound_parameters: HashMap<String, UnboundParameter>,
    pub species: HashMap<String, Species>,
    pub unbound_species: HashMap<String, UnboundSpecies>,
    pub functions: HashMap<String, Vec<MathNode>>,
    pub reactions: HashMap<String, Reaction>,
    pub initial_assignments: HashMap<String, InitialAssignment>,
    pub assignment_rules: HashMap<String, AssignmentRule>,
    pub rate_rules: HashMap<String, RateRule>,
    pub ODEs: Vec<ODE>,
}

impl Bindings {
    pub fn new() -> Self {
        Bindings {
            compartments: HashMap::new(),
            unbound_compartments: HashMap::new(),
            parameters: HashMap::new(),
            unbound_parameters: HashMap::new(),
            species: HashMap::new(),
            unbound_species: HashMap::new(),
            functions: HashMap::new(),
            initial_assignments: HashMap::new(),
            reactions: HashMap::new(),
            assignment_rules: HashMap::new(),
            rate_rules: HashMap::new(),
            ODEs: Vec::new(),
        }
    }

    pub fn from(model: &Model) -> Self {
        let mut bindings = Bindings::new();

        bindings.parse_compartments(model);
        bindings.parse_parameters(model);
        bindings.parse_species(model);
        bindings.functions = model.function_definition_math();
        bindings.parse_initial_assignments(model);
        bindings.evaluate_initial_assignments();
        bindings.parse_reactions(model);
        bindings.parse_assignment_rules(model);
        bindings.evaluate_assignment_rules();
        bindings.parse_rate_rules(model);
        bindings.parse_ODEs(model);

        bindings
    }

    pub fn values(&self) -> HashMap<String, f64> {
        let mut hm: HashMap<String, f64> = HashMap::new();

        for (id, compartment) in &self.compartments {
            hm.insert(id.clone(), compartment.size());
        }
        for (id, parameter) in &self.parameters {
            hm.insert(id.clone(), parameter.value);
        }
        for (id, species) in &self.species {
            hm.insert(id.clone(), species.amount());
        }

        hm
    }

    pub fn results(&self, print_amounts: bool) -> HashMap<String, f64> {
        let mut hm: HashMap<String, f64> = HashMap::new();

        for (id, compartment) in &self.compartments {
            hm.insert(id.clone(), compartment.size());
        }
        for (id, parameter) in &self.parameters {
            hm.insert(id.clone(), parameter.value);
        }
        for (id, species) in &self.species {
            if print_amounts {
                hm.insert(id.clone(), species.amount());
            } else {
                hm.insert(id.clone(), species.concentration());
            }
        }

        hm
    }

    #[allow(non_snake_case)]
    pub fn parse_compartments(&mut self, model: &Model) {
        // Compartments
        for compartment in model.compartments() {
            if let Some(id) = &compartment.id {
                match Compartment::from(&compartment) {
                    Ok(bound_compartment) => {
                        self.compartments.insert(id.to_string(), bound_compartment);
                    }
                    Err(unbound_compartment) => {
                        self.unbound_compartments
                            .insert(id.to_string(), unbound_compartment);
                    }
                }
            }
        }
    }

    pub fn parse_parameters(&mut self, model: &Model) {
        // Parameters
        for parameter in model.parameters() {
            if let Some(id) = &parameter.id {
                match Parameter::from(&parameter) {
                    Ok(bound_parameter) => {
                        self.parameters.insert(id.to_string(), bound_parameter);
                    }
                    Err(unbound_parameter) => {
                        self.unbound_parameters
                            .insert(id.to_string(), unbound_parameter);
                    }
                }
            }
        }
    }

    pub fn parse_initial_assignments(&mut self, model: &Model) {
        for initial_assignment in model.initial_assignments() {
            if let Some(symbol) = &initial_assignment.symbol {
                self.initial_assignments.insert(
                    symbol.clone(),
                    InitialAssignment::from(&initial_assignment, model),
                );
            } else {
                panic!("InitialAssignments must be associated with a symbol.");
            }
        }
    }

    pub fn parse_species(&mut self, model: &Model) {
        // Species
        for species in model.species() {
            // SpeciesID and Compartment are mandatory attributes
            let id = &species
                .id
                .as_ref()
                .expect("Species ID not found.")
                .to_owned();
            let compartment_id = &species
                .compartment
                .as_ref()
                .expect("Species compartment not found.")
                .to_owned();

            // But compartment may not have a size yet
            if let Some(compartment) = self.compartments.get(compartment_id) {
                // InitialAmount is optional
                if let Some(amount) = &species.initial_amount {
                    let bound_species = Species::from_amount(&species, *amount, compartment.size());
                    self.species.insert(id.clone(), bound_species);
                // InitialConc is also optional
                } else if let Some(concentration) = &species.initial_concentration {
                    if let Some(compartment) = self.compartments.get(compartment_id) {
                        let bound_species = Species::from_concentration(
                            &species,
                            *concentration,
                            compartment.size(),
                        );
                        self.species.insert(id.clone(), bound_species);
                    }
                } else {
                    let unbound_species = UnboundSpecies::from(&species);
                    self.unbound_species.insert(id.clone(), unbound_species);
                }
            } else {
                let unbound_species = UnboundSpecies::from(&species);
                self.unbound_species.insert(id.clone(), unbound_species);
            }
        }
    }

    pub fn update_compartment_size(&mut self, compartment_id: &String, size: f64) {
        if let Some(compartment) = self.compartments.get_mut(compartment_id) {
            compartment.update_size(size);
        }
        for (_, species) in &mut self.species {
            if species.compartment == compartment_id.to_owned() {
                species.update_compartment_size(size);
            }
        }
    }

    pub fn update_compartment_size_by(&mut self, compartment_id: &String, delta: f64) {
        if let Some(compartment) = self.compartments.get_mut(compartment_id) {
            compartment.update_size_by(delta);
            let size = compartment.size();
            for (_, species) in &mut self.species {
                if species.compartment == compartment_id.to_owned() {
                    //let conc = species.concentration();
                    species.update_compartment_size(size);
                    //println!(
                    //"changed conc of {} from {} to {}",
                    //species.id,
                    //conc,
                    //species.concentration()
                    //);
                }
            }
        }
    }

    pub fn evaluate_initial_assignments(&mut self) {
        let initial_assignments = self.initial_assignments.clone();
        for (symbol, initial_assignment) in initial_assignments {
            let mut values = self.values();
            if let Ok(value) = evaluate_node(&initial_assignment.math, 0, &values, &self.functions)
            {
                // Update value
                values.insert(symbol.clone(), value);

                // Bound compartment being reassigned
                if self.compartments.get(&symbol).is_some() {
                    self.update_compartment_size(&symbol, value);
                }

                // Unbound compartment
                if let Some(unbound_compartment) = self.unbound_compartments.get(&symbol) {
                    let bound_compartment = unbound_compartment.to_bound(value);
                    self.compartments.insert(symbol.clone(), bound_compartment);
                    self.unbound_compartments.remove(&symbol);
                    // It might be possible to assign values to some unbound species now
                    let mut bound_species_ids = Vec::new();
                    for (id, species) in &self.unbound_species {
                        if &species.compartment == id {
                            if let Ok(bound_species) = species.to_bound_with_size(value) {
                                values.insert(id.clone(), bound_species.concentration());
                                self.species.insert(id.clone(), bound_species);
                                bound_species_ids.push(id.clone());
                            }
                        }
                    }
                    for id in bound_species_ids {
                        self.unbound_species.remove(&id);
                    }
                }

                // Parameter
                if self.parameters.get(&symbol).is_some() {
                    self.parameters
                        .entry(symbol.clone())
                        .and_modify(|c| c.value = value);
                }

                // Unbound parameter
                if let Some(unbound_parameter) = self.unbound_parameters.get(&symbol) {
                    let bound_parameter = unbound_parameter.to_bound(value);
                    self.parameters.insert(symbol.clone(), bound_parameter);
                    self.unbound_parameters.remove(&symbol);
                }

                // Species being reassigned
                if let Some(species) = self.species.get_mut(&symbol) {
                    let compartment = &species.compartment;
                    let compartment_size = self.compartments.get(compartment).unwrap().size();
                    if !species.has_only_substance_units {
                        species.update_concentration(value, compartment_size);
                    } else {
                        species.update_amount(value, compartment_size);
                        values.insert(symbol.clone(), species.concentration());
                    }
                }

                // For now, assuming that the compartment size is known by now
                // This will have to change later because compartments can also be
                // assigned by algebraic rules etc. which are not supported right now
                if let Some(unbound_species) = self.unbound_species.get(&symbol) {
                    let size = self
                        .compartments
                        .get(&unbound_species.compartment)
                        .expect("Compartment size not found.")
                        .size();
                    let species;
                    if !unbound_species.has_only_substance_units {
                        let concentration = value;
                        let amount = concentration * size;
                        species = unbound_species.to_bound(amount, concentration);
                    } else {
                        let amount = value;
                        let concentration = amount / size;
                        species = unbound_species.to_bound(amount, concentration);
                        values.insert(symbol.clone(), species.concentration());
                    }
                    self.species.insert(symbol.clone(), species);
                }
            }
        }
    }

    pub fn parse_assignment_rules(&mut self, model: &Model) {
        for assignment_rule in model.assignment_rules() {
            if let Some(variable) = &assignment_rule.variable {
                self.assignment_rules.insert(
                    variable.clone(),
                    AssignmentRule::from(&assignment_rule, model),
                );
            } else {
                panic!("AssignmentRule must be associated with a variable.");
            }
        }
    }

    pub fn parse_rate_rules(&mut self, model: &Model) {
        for rate_rule in model.rate_rules() {
            if let Some(variable) = &rate_rule.variable {
                self.rate_rules
                    .insert(variable.clone(), RateRule::from(&rate_rule, model));
            } else {
                panic!("RateRule must be associated with a variable.");
            }
        }
    }

    pub fn parse_reactions(&mut self, model: &Model) {
        // Reaction
        for reaction in model.reactions() {
            if let Some(id) = &reaction.id {
                let reaction = Reaction::from(&reaction, model);
                self.reactions.insert(id.to_string(), reaction);
            }
        }
    }

    pub fn local_parameters(&self) -> HashMap<String, HashMap<String, f64>> {
        let mut hm: HashMap<String, HashMap<String, f64>> = HashMap::new();
        for (reaction_id, reaction) in &self.reactions {
            hm.insert(reaction_id.clone(), reaction.local_parameter_values());
        }
        hm
    }

    pub fn assignment_rules_math(&self) -> HashMap<String, Vec<MathNode>> {
        let mut hm: HashMap<String, Vec<MathNode>> = HashMap::new();
        for (variable, assignment_rule) in &self.assignment_rules {
            hm.insert(variable.clone(), assignment_rule.math.clone());
        }
        hm
    }

    pub fn evaluate_assignment_rules(&mut self) {
        let mut values = self.values();
        let assignment_rules = self.assignment_rules.clone();
        for (variable, rule) in assignment_rules {
            if let Ok(value) = evaluate_node(&rule.math, 0, &values, &self.functions) {
                // Update value
                values.insert(variable.clone(), value);

                // Bound compartment being reassigned
                if self.compartments.get(&variable).is_some() {
                    // Update value
                    self.update_compartment_size(&variable, value);
                }

                // Parameter
                if self.parameters.get(&variable).is_some() {
                    self.parameters
                        .entry(variable.clone())
                        .and_modify(|c| c.value = value);
                }

                // Species being reassigned
                if let Some(species) = self.species.get_mut(&variable) {
                    let compartment = &species.compartment;
                    let compartment_size = self.compartments.get(compartment).unwrap().size();
                    if !species.has_only_substance_units {
                        species.update_concentration(value, compartment_size);
                        values.insert(species.id.clone(), species.concentration());
                    } else {
                        species.update_amount(value, compartment_size);
                        values.insert(species.id.clone(), species.concentration());
                    }
                }

                // For now, assuming that the compartment size is known by now
                // This will have to change later because compartments can also be
                // assigned by algebraic rules etc. which are not supported right now
                if let Some(unbound_species) = self.unbound_species.get(&variable) {
                    let size = self
                        .compartments
                        .get(&unbound_species.compartment)
                        .expect("Compartment size not found.")
                        .size();
                    let species;
                    if !unbound_species.has_only_substance_units {
                        let concentration = value;
                        let amount = concentration * size;
                        species = unbound_species.to_bound(amount, concentration);
                    } else {
                        let amount = value;
                        let concentration = amount / size;
                        species = unbound_species.to_bound(amount, concentration);
                        values.insert(variable.clone(), species.concentration());
                    }
                    self.species.insert(variable.clone(), species);
                }

                // SpeciesReferences
                for (_, reaction) in &mut self.reactions {
                    // Bound reactants being reassigned
                    reaction
                        .reactants
                        .entry(variable.clone())
                        .and_modify(|reactant| reactant.stoichiometry = value);
                    reaction
                        .products
                        .entry(variable.clone())
                        .and_modify(|product| product.stoichiometry = value);
                }
            }
        }
    }

    #[allow(non_snake_case)]
    pub fn parse_ODEs(&mut self, model: &Model) {
        // stores a matrix where key is (SpeciesID, ReactionID)
        // and value is a SpeciesStates::(Reactant, Product, None)
        // Calculated before running the simulation so that
        // reactants and products don't have to be checked at each iteration
        let rxn_matrix = model.reaction_matrix();

        for (species_id, species) in &self.species {
            if species.boundary_condition {
                continue;
            }

            let compartment = &species.compartment;
            //let mut ode = ODE::new(species_id.clone(), Some(compartment.clone()));
            let mut ode = ODE::new(species_id.clone(), None);

            let mut term_count = 0;
            for (rxn_id, reaction) in &self.reactions {
                let mut coefficient = None;
                // simulation step
                match rxn_matrix.get(&(species_id.to_string(), rxn_id.to_string())) {
                    Some(SpeciesStatus::Reactant(stoich)) => {
                        coefficient = Some(-stoich);
                    }
                    Some(SpeciesStatus::Product(stoich)) => {
                        coefficient = Some(*stoich);
                    }
                    _ => {}
                }

                if let Some(value) = coefficient {
                    let ode_term =
                        ODETerm::new(value, reaction.kinetic_law.clone(), rxn_id.to_string());
                    ode.add_term(ode_term);
                    term_count += 1;
                }
            }

            if term_count > 0 {
                self.ODEs.push(ode);
            }
        }
        // Rate rules
        for (variable, rule) in &self.rate_rules {
            let ode_term = ODETerm::new(1.0, rule.math.clone(), "None".to_string());
            let mut ode = ODE::new(variable.clone(), None);
            ode.add_term(ode_term);
            self.ODEs.push(ode);
        }
    }

    pub fn update_delta(&mut self, key: &String, delta: f64) {
        if let Some(species) = self.species.get_mut(key) {
            let compartment = &species.compartment;
            let compartment_size = self.compartments.get(compartment).unwrap().size();
            //if species.has_only_substance_units {
            let amount = species.amount();
            species.update_amount(amount + delta, compartment_size);
            //println!(
            //"changed amount of {} from {} to {}",
            //key,
            //amount,
            //species.amount()
            //);
            //} else {
            //let conc = species.concentration();
            //species.update_concentration(conc + delta, compartment_size);
            ////println!(
            ////"changed conc of {} from {} to {}",
            ////key,
            ////conc,
            ////species.concentration()
            ////);
            //}
        } else if let Some(parameter) = self.parameters.get_mut(key) {
            parameter.value += delta;
        } else if self.compartments.get(key).is_some() {
            // this function also updates species concentrations
            self.update_compartment_size_by(key, delta);
        } else {
            panic!("Invalid key {}", key);
        }
        // TODO for other types
    }
}

pub enum BindingType {
    Species,
    Compartment,
    Parameter,
    LocalParameter,
    Stoichiometry,
}
