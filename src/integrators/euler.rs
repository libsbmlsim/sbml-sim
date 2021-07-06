use mathml_rs::evaluate_node;
use sbml_rs::{KineticLaw, MathTag, Model, SpeciesStatus};
use std::collections::HashMap;

pub fn euler_integrator(
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

    let reaction_ids = model
        .reactions()
        .iter()
        .map(|r| r.id.as_ref().unwrap().to_owned())
        .collect::<Vec<String>>();

    let all_kinetic_laws = model.all_kinetic_laws();

    let rxn_matrix = model.reaction_matrix();

    for t in 1..(steps + 1) {
        let mut iteration_result: HashMap<String, f64> = HashMap::new();
        iteration_result.insert("t".to_string(), (t as f64) * step_size);

        for rxn_id in &reaction_ids {
            let kinetic_law = all_kinetic_laws.get(rxn_id).unwrap().to_owned();
            let derivative = evaluate_node(&kinetic_law.nodes, 0, &assignments)?;

            for species in &species {
                // get species ID
                let species_id = species.id.as_ref().unwrap().to_owned();

                let current_value = assignments.get(&species_id).unwrap().to_owned();
                iteration_result
                    .entry(species_id.to_string())
                    .or_insert(current_value);

                if species.boundary_condition.unwrap() {
                    continue;
                }

                match rxn_matrix.get(&(species_id.to_string(), rxn_id.to_string())) {
                    Some(SpeciesStatus::Reactant(stoich)) => {
                        iteration_result
                            .entry(species_id.to_string())
                            .and_modify(|e| *e -= step_size * derivative * stoich);
                    }
                    Some(SpeciesStatus::Product(stoich)) => {
                        iteration_result
                            .entry(species_id.to_string())
                            .and_modify(|e| *e += step_size * derivative * stoich);
                    }
                    _ => {}
                }
            }
        }
        // TODO: Assert sum = constant

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
