//use super::euler::euler;
use super::super::structs::derivative::Derivative;
use super::euler::euler;
use super::runge_kutta::runge_kutta_4;
use sbml_rs::{MathTag, Model, SpeciesStatus};
use std::collections::HashMap;

pub fn integrate(
    model: &Model,
    time: f64,
    step_size: f64,
) -> Result<Vec<HashMap<String, f64>>, String> {
    // number of steps
    let steps = (time / step_size).ceil() as i32;
    // vector to store results
    let mut results = Vec::new();

    // get initial assignments from the model
    let mut assignments = model.assignments();
    let functions = model.function_definition_tags();
    //dbg!(&assignments);

    // get list of species IDs
    let species = model.species();

    // store first result as initial values
    let mut initial_results = assignments.clone();
    initial_results.insert("t".to_string(), 0.0);
    results.push(initial_results);

    let derivatives = get_derivatives(model);

    for t in 1..(steps + 1) {
        let mut iteration_result = results.last().unwrap().clone();
        iteration_result.insert("t".to_string(), (t as f64) * step_size);

        let deltas = runge_kutta_4(&derivatives, &assignments, &functions, step_size)?;
        //let deltas = euler(&derivatives, &assignments, step_size)?;

        for (key, value) in deltas.iter() {
            iteration_result
                .entry(key.to_string())
                .and_modify(|e| *e += value);
        }

        for (key, val) in iteration_result.iter() {
            if key == "t" {
                continue;
            } else {
                assignments.insert(key.to_string(), val.to_owned());
            }
        }
        //dbg!(&iteration_result);
        results.push(iteration_result);
    }

    let mut result_amounts: Vec<HashMap<String, f64>> = Vec::new();
    for timestep in &results {
        let mut result_amounts_current = timestep.clone();
        for sp in &species {
            let compartment = sp.compartment.as_ref().unwrap();
            let species_id = sp.id.as_ref().unwrap();
            let sp_amount = timestep.get(species_id).unwrap();
            let compartment_size = timestep.get(compartment).unwrap();
            let concentration = sp_amount * compartment_size;
            result_amounts_current.insert(species_id.to_owned(), concentration);
        }
        result_amounts.push(result_amounts_current);
    }

    Ok(result_amounts)
}

fn get_derivatives(model: &Model) -> HashMap<String, Derivative> {
    // get list of species IDs
    let species = model.species();
    let reaction_ids = model
        .reactions()
        .iter()
        .map(|r| r.id.as_ref().unwrap().to_owned())
        .collect::<Vec<String>>();

    let all_kinetic_laws = model.all_kinetic_laws();

    // stores a matrix where key is (SpeciesID, ReactionID)
    // and value is a SpeciesStates::(Reactant, Product, None)
    // Calculated before running the simulation so that
    // reactants and products don't have to be checked at each iteration
    let rxn_matrix = model.reaction_matrix();

    let mut derivatives: HashMap<String, Derivative> = HashMap::new();
    for sp in species {
        if let Some(true) = sp.boundary_condition {
            continue;
        }

        let species_id = sp.id.as_ref().unwrap().to_owned();
        let compartment_size = sp.compartment_size(model).unwrap();

        for rxn_id in &reaction_ids {
            let kinetic_law = all_kinetic_laws.get(rxn_id).unwrap().to_owned();
            // simulation step
            match rxn_matrix.get(&(species_id.to_string(), rxn_id.to_string())) {
                Some(SpeciesStatus::Reactant(stoich)) => {
                    derivatives
                        .entry(species_id.clone())
                        .or_insert(Derivative::new(compartment_size))
                        .add_term(-stoich, kinetic_law.clone());
                }
                Some(SpeciesStatus::Product(stoich)) => {
                    derivatives
                        .entry(species_id.clone())
                        .or_insert(Derivative::new(compartment_size))
                        .add_term(*stoich, kinetic_law.clone());
                }
                _ => {}
            }
        }
    }
    derivatives
}
