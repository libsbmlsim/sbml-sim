use mathml_rs::{evaluate_node, MathNode};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use toposort_scc::IndexGraph;

use sbml_rs;
use sbml_rs::Model;

use crate::{
    AssignmentRule, Compartment, InitialAssignment, ODETerm, Parameter, RateRule, Reaction,
    Species, SpeciesStatus, UnboundCompartment, UnboundParameter, UnboundSpecies, ODE,
};
use std::collections::{HashMap, HashSet};

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
    pub initial_assignments: Vec<InitialAssignment>,
    pub assignment_rules: Vec<AssignmentRule>,
    pub rate_rules: Vec<RateRule>,
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
            initial_assignments: Vec::new(),
            reactions: HashMap::new(),
            assignment_rules: Vec::new(),
            rate_rules: Vec::new(),
            ODEs: Vec::new(),
        }
    }

    pub fn from(model: &Model) -> Self {
        let mut bindings = Bindings::new();

        bindings.parse_compartments(model);
        bindings.parse_parameters(model);
        bindings.parse_species(model);
        bindings.functions = model.function_definition_math();
        bindings.parse_reactions(model);
        bindings.parse_initial_assignments(model);
        bindings.parse_assignment_rules(model);
        dbg!(&bindings);
        dbg!(&bindings.values());
        bindings.sort_initial_assignments();
        bindings.evaluate_initial_assignments();
        bindings.evaluate_assignment_rules();
        bindings.recheck_species();
        bindings.parse_rate_rules(model);
        bindings.parse_ODEs();

        //dbg!(&bindings);
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
        for (_reaction_id, reaction) in &self.reactions {
            for (_reactant_species_id, reactant) in &reaction.reactants {
                hm.insert(reactant.id.clone(), reactant.stoichiometry);
            }

            for (_product_species_id, product) in &reaction.products {
                hm.insert(product.id.clone(), product.stoichiometry);
            }
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
            if initial_assignment.symbol.is_some() && initial_assignment.math.is_some() {
                self.initial_assignments
                    .push(InitialAssignment::from(&initial_assignment, model));
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
                    species.update_compartment_size(size);
                }
            }
        }
    }

    pub fn sort_initial_assignments(&mut self) {
        let mut dependencies = HashMap::<String, Vec<String>>::new();
        let mut symbols_set = HashSet::<String>::new();
        //decide an order for initial assignments by performing a topological sort
        for assignment in &self.initial_assignments {
            dependencies.insert(assignment.symbol.clone(), Vec::<String>::new());
            symbols_set.insert(assignment.symbol.clone());

            for dep in &assignment.dependent_variables {
                dependencies
                    .entry(assignment.symbol.clone())
                    .or_insert(Vec::<String>::new())
                    .push(dep.clone());
                symbols_set.insert(dep.clone());
            }
        }
        let mut symbols: Vec<String> = symbols_set.into_iter().collect();
        symbols.sort();
        dbg!(&dependencies);

        let mut g = IndexGraph::with_vertices(symbols.len());
        for (symbol, symbol_deps) in &dependencies {
            let symbol_idx = symbols.iter().position(|r| r == symbol).unwrap();
            for dep in symbol_deps {
                let dep_idx = symbols.iter().position(|r| r == dep).unwrap();
                g.add_edge(symbol_idx, dep_idx);
            }
        }

        let mut assignment_order: Vec<String> = g
            .toposort()
            .unwrap()
            .into_iter()
            .map(|x| symbols[x].clone())
            .collect();
        assignment_order.reverse();

        for initial_assignment in &mut self.initial_assignments {
            initial_assignment.topological_order = assignment_order
                .iter()
                .position(|x| x == &initial_assignment.symbol)
                .unwrap();
        }

        self.initial_assignments.sort();
    }

    pub fn evaluate_initial_assignments(&mut self) {
        let initial_assignments = self.initial_assignments.clone();
        for initial_assignment in initial_assignments {
            let symbol = initial_assignment.symbol;
            let mut values = self.values();
            match evaluate_node(&initial_assignment.math, 0, &values, &self.functions) {
                Ok(value) => {
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
                            if species.compartment == symbol {
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

                    // Stoichiometry of a reactant
                    for (_, reaction) in &mut self.reactions {
                        for (_, reactant) in &mut reaction.reactants {
                            if reactant.id == symbol {
                                reactant.stoichiometry = value;
                            }
                        }
                        for (_, product) in &mut reaction.products {
                            if product.id == symbol {
                                product.stoichiometry = value;
                            }
                        }
                    }
                }
                Err(_error) => {
                    //panic!("Evaluation of {} failed: {}", symbol, _error);
                }
            }
        }
    }

    pub fn recheck_species(&mut self) {
        // It might be possible to assign values to some unbound species now
        let mut bound_species_ids = Vec::new();
        for (species_id, species) in &self.unbound_species {
            if let Some(compartment) = self.compartments.get(&species.compartment) {
                if let Ok(bound_species) = species.to_bound_with_size(compartment.size()) {
                    self.species.insert(species_id.clone(), bound_species);
                    bound_species_ids.push(species_id.clone());
                }
            }
        }
        for id in bound_species_ids {
            self.unbound_species.remove(&id);
        }
    }

    pub fn parse_assignment_rules(&mut self, model: &Model) {
        for assignment_rule in model.assignment_rules() {
            if assignment_rule.variable.is_some() && assignment_rule.math.is_some() {
                self.assignment_rules
                    .push(AssignmentRule::from(&assignment_rule, model));
                // also create an initial assignment from assignment rule
                self.initial_assignments
                    .push(InitialAssignment::from_assignment_rule(
                        &assignment_rule,
                        model,
                    ));
            }
        }
    }

    pub fn parse_rate_rules(&mut self, model: &Model) {
        for rate_rule in model.rate_rules() {
            if rate_rule.variable.is_some() && rate_rule.math.is_some() {
                self.rate_rules.push(RateRule::from(&rate_rule, model));
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
        for assignment_rule in &self.assignment_rules {
            hm.insert(
                assignment_rule.variable.clone(),
                assignment_rule.math.clone(),
            );
        }
        hm
    }

    // Calculates values of dependent variables by evaluating assignment rules
    // and updates the provided Bindings object
    pub fn evaluate_assignment_rules(&mut self) {
        let mut values = self.values();
        let assignment_rules = self.assignment_rules.clone();
        for rule in assignment_rules {
            let variable = rule.variable;
            match evaluate_node(&rule.math, 0, &values, &self.functions) {
                Ok(value) => {
                    // Update value
                    values.insert(variable.clone(), value);

                    // Bound compartment being reassigned
                    if self.compartments.get(&variable).is_some() {
                        // Update value
                        self.update_compartment_size(&variable, value);
                        continue;
                    }

                    // Unbound compartment
                    if let Some(unbound_compartment) = self.unbound_compartments.get(&variable) {
                        let bound_compartment = unbound_compartment.to_bound(value);
                        self.compartments
                            .insert(variable.clone(), bound_compartment);
                        self.unbound_compartments.remove(&variable);
                        continue;
                    }

                    // Parameter
                    if self.parameters.get(&variable).is_some() {
                        self.parameters
                            .entry(variable.clone())
                            .and_modify(|c| c.value = value);
                        continue;
                    }

                    // Unbound parameter
                    if let Some(unbound_parameter) = self.unbound_parameters.get(&variable) {
                        let bound_parameter = unbound_parameter.to_bound(value);
                        self.parameters.insert(variable.clone(), bound_parameter);
                        self.unbound_parameters.remove(&variable);
                        continue;
                    }

                    // Species being reassigned
                    if let Some(species) = self.species.get_mut(&variable) {
                        let compartment = &species.compartment;
                        let compartment_size = self.compartments.get(compartment).unwrap().size();
                        // TODO: THIS IS PROBABLY WRONG
                        if !species.has_only_substance_units {
                            species.update_concentration(value, compartment_size);
                            values.insert(species.id.clone(), species.concentration());
                            continue;
                        } else {
                            species.update_amount(value, compartment_size);
                            values.insert(species.id.clone(), species.concentration());
                            continue;
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
                        //println!("Set {} to {}", &species.id, species.amount());
                        self.species.insert(variable.clone(), species);
                    }

                    // SpeciesReferences
                    for (_rxn_id, reaction) in &mut self.reactions {
                        // Stoichiometry of a reactant being reassigned
                        for (_reactant_species_id, reactant) in &mut reaction.reactants {
                            if reactant.id == variable {
                                reactant.stoichiometry = value;
                                dbg!("{} assigned {}", &reactant.id, value);
                                continue;
                            }
                        }
                        // Unbound stoichiometry of a reactant
                        for (reactant_species_id, reactant) in &mut reaction.unbound_reactants {
                            if reactant.id == variable {
                                let bound_reactant = reactant.to_bound(value);
                                reaction
                                    .reactants
                                    .insert(reactant_species_id.clone(), bound_reactant);
                                continue;
                            }
                        }
                        // Stoichiometry of a product
                        for (_product_species_id, product) in &mut reaction.products {
                            if product.id == variable {
                                product.stoichiometry = value;
                                continue;
                            }
                        }
                        // Unbound stoichiometry of a product
                        for (product_species_id, product) in &mut reaction.unbound_products {
                            if product.id == variable {
                                let bound_product = product.to_bound(value);
                                reaction
                                    .products
                                    .insert(product_species_id.clone(), bound_product);
                                continue;
                            }
                        }
                    }
                }
                Err(error) => panic!("{}", error),
            }
        }
    }

    // Calculates values of dependent variables by evaluating their assignment rules.
    // Prioritizes supplied assignments over assignments in the Bindings object.
    // Does not change the original Bindings object,
    // just returns a new assignment hashmap.
    pub fn emulate_assignment_rules(
        &self,
        assignments: &HashMap<String, f64>,
    ) -> HashMap<String, f64> {
        // priorities provided assignments over Bindings
        let mut values = self.values();
        for (key, value) in assignments {
            values.insert(key.clone(), *value);
        }

        let assignment_rules = self.assignment_rules.clone();
        for rule in assignment_rules {
            let variable = rule.variable;
            match evaluate_node(&rule.math, 0, &values, &self.functions) {
                Ok(value) => {
                    // Update value
                    values.insert(variable.clone(), value);
                }
                Err(error) => panic!("{}", error),
            }
        }
        values
    }

    pub fn reaction_matrix(&self) -> HashMap<(String, String), Vec<SpeciesStatus>> {
        let mut rxn_matrix: HashMap<(String, String), Vec<SpeciesStatus>> = HashMap::new();

        for (sp_id, _) in &self.species {
            for (rxn_id, reaction) in &self.reactions {
                rxn_matrix.insert((sp_id.clone(), rxn_id.clone()), Vec::new());

                for (reactant_species, reactant) in &reaction.reactants {
                    if sp_id == reactant_species {
                        rxn_matrix
                            .entry((sp_id.clone(), rxn_id.clone()))
                            .and_modify(|v| v.push(SpeciesStatus::Reactant(reactant.id.clone())));
                    }
                }
                for (product_species, product) in &reaction.products {
                    if sp_id == product_species {
                        rxn_matrix
                            .entry((sp_id.clone(), rxn_id.clone()))
                            .and_modify(|v| v.push(SpeciesStatus::Product(product.id.clone())));
                    }
                }
            }
        }

        rxn_matrix
    }

    #[allow(non_snake_case)]
    pub fn parse_ODEs(&mut self) {
        // stores a matrix where key is (SpeciesID, ReactionID)
        // and value is a SpeciesStates::(Reactant, Product, None)
        // Calculated before running the simulation so that
        // reactants and products don't have to be checked at each iteration
        let rxn_matrix = self.reaction_matrix();

        for (species_id, species) in &self.species {
            if species.boundary_condition {
                continue;
            }

            let mut ode = ODE::new(species_id.clone());

            let mut term_count = 0;
            for (rxn_id, reaction) in &self.reactions {
                // simulation step
                let sp_statuses = rxn_matrix
                    .get(&(species_id.to_string(), rxn_id.to_string()))
                    .expect("Rxn matrix");
                for status in sp_statuses {
                    let mut coefficient_id = None;
                    let mut coefficient_factor = 1.0;
                    match status {
                        SpeciesStatus::Reactant(reactant_id) => {
                            coefficient_id = Some(reactant_id);
                            coefficient_factor = -1.0;
                        }
                        SpeciesStatus::Product(product_id) => {
                            coefficient_id = Some(product_id);
                            coefficient_factor = 1.0;
                        }
                        _ => {}
                    }

                    if let Some(x) = coefficient_id {
                        let ode_term = ODETerm::new(
                            coefficient_factor,
                            Some(x.to_string()),
                            reaction.kinetic_law.clone(),
                            Some(rxn_id.to_string()),
                        );
                        ode.add_term(ode_term);
                        term_count += 1;
                    }
                }
            }

            if term_count > 0 {
                self.ODEs.push(ode);
            }
        }
        // Rate rules
        for rule in &self.rate_rules {
            let ode_term = ODETerm::new(1.0, None, rule.math.clone(), None);
            let mut ode = ODE::new(rule.variable.clone());
            ode.add_term(ode_term);
            self.ODEs.push(ode);
        }
    }

    pub fn update_delta(&mut self, key: &String, delta: f64) {
        let mut updated = false;
        if !updated {
            if let Some(species) = self.species.get_mut(key) {
                let compartment = &species.compartment;
                let compartment_size = self.compartments.get(compartment).unwrap().size();
                let amount = species.amount();
                species.update_amount(amount + delta, compartment_size);
                updated = true;
            }
        }
        if !updated {
            if let Some(parameter) = self.parameters.get_mut(key) {
                parameter.value += delta;
                updated = true;
            }
        }
        if !updated {
            if self.compartments.get(key).is_some() {
                // this function also updates species concentrations
                self.update_compartment_size_by(key, delta);
                updated = true;
            }
        }
        if !updated {
            for (_reaction_id, reaction) in &mut self.reactions {
                for (_reactant_species_id, reactant) in &mut reaction.reactants {
                    if key.to_string() == reactant.id {
                        reactant.stoichiometry += delta;
                        updated = true;
                        break;
                    }
                }
            }
        }
        if !updated {
            for (_reaction_id, reaction) in &mut self.reactions {
                for (_product_species_id, product) in &mut reaction.products {
                    if key.to_string() == product.id {
                        product.stoichiometry += delta;
                        updated = true;
                        break;
                    }
                }
            }
        }

        if !updated {
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

pub fn random_string() -> String {
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();
    return rand_string;
}
