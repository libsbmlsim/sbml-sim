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

    // get list of species IDs
    let species = model.species();

    // store first result as initial values
    let mut initial_results = HashMap::new();
    initial_results.insert("t".to_string(), 0.0);
    for sp in &species {
        if let Some(id) = &sp.id {
            initial_results.insert(id.to_string(), assignments.get(id).unwrap().to_owned());
        }
    }
    results.push(initial_results);

    let derivatives = get_derivatives(model);

    for t in 1..(steps + 1) {
        let mut iteration_result: HashMap<String, f64> = HashMap::new();
        iteration_result.insert("t".to_string(), (t as f64) * step_size);
        //dbg!(t);
        //
        let deltas = runge_kutta_4(&derivatives, &assignments, step_size)?;

        for sp in &species {
            let species_id = sp.id.as_ref().unwrap().to_owned();
            let current_value = assignments.get(&species_id).unwrap().to_owned();
            // add current value to iteration_results
            iteration_result.insert(species_id.to_string(), current_value);

            // don't update results if boundary_condition is true for species
            if sp.boundary_condition.unwrap() {
                continue;
            }
            let delta = deltas.get(&species_id).unwrap().to_owned();
            //let delta = euler(derivative, &assignments, step_size)?;
            iteration_result
                .entry(species_id.to_string())
                .and_modify(|e| *e += delta);
        }

        for (key, val) in iteration_result.iter() {
            if key == "t" {
                continue;
            } else {
                assignments.insert(key.to_string(), val.to_owned());
            }
        }
        results.push(iteration_result);
    }
    Ok(results)
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
    for sp in &species {
        let species_id = sp.id.as_ref().unwrap().to_owned();

        for rxn_id in &reaction_ids {
            let kinetic_law = all_kinetic_laws.get(rxn_id).unwrap().to_owned();
            // simulation step
            match rxn_matrix.get(&(species_id.to_string(), rxn_id.to_string())) {
                Some(SpeciesStatus::Reactant(stoich)) => {
                    derivatives
                        .entry(species_id.clone())
                        .or_insert(Derivative::default())
                        .push(-stoich, kinetic_law.clone());
                }
                Some(SpeciesStatus::Product(stoich)) => {
                    derivatives
                        .entry(species_id.clone())
                        .or_insert(Derivative::default())
                        .push(*stoich, kinetic_law.clone());
                }
                _ => {}
            }
        }
    }
    derivatives
}
