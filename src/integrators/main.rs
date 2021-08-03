use super::super::structs::derivative::{Derivative, DerivativeTerm};
use super::runge_kutta_fehlberg::runge_kutta_fehlberg_45;
use mathml_rs::{evaluate_node, MathNode};
use sbml_rs::{Model, SpeciesStatus};
use std::collections::HashMap;
use std::io::stdin;

pub fn integrate(
    model: &Model,
    duration: f64,
    steps: i32,
    init_step_size: f64,
    rtol: f64,
    atol: f64,
) -> Result<Vec<HashMap<String, f64>>, String> {
    // number of steps let steps = (time / step_size).ceil() as i32;
    // vector to store results
    let mut results = Vec::new();

    // get initial assignments from the model
    let mut assignments = model.assignments();
    let local_parameters = model.local_parameters();
    let functions = model.function_definition_math();
    let assignment_rules = model.assignment_rule_math();
    assignments = evaluate_rules(&assignment_rules, &mut assignments, &functions);
    // get list of species IDs
    let species = model.species();

    // store first result as initial values
    let mut initial_results = assignments.clone();
    initial_results.insert("t".to_string(), 0.0);
    results.push(initial_results);

    let derivatives = get_derivatives(model);

    let mut t = 0.0;
    // the interval at which results are required
    let result_interval = duration / (steps as f64);
    let mut t_next_result = ((t + result_interval) * 1000000.0).round() / 1000000.0;

    let mut current_step_size = init_step_size;
    // this is used if the step size was adjusted in the previous step to hit a result point
    // used only if the associated boolean value is true
    let mut cached_step_size = None;
    //println!("Starting with step size {}", current_step_size);

    // step_size is not permitted to become smaller than
    // 25 times the smallest float value supported
    //let min_step_size = f64::EPSILON * 25.0;

    while duration - t > f64::EPSILON {
        //println!();
        //println!("Integrating from {} to {}", t, t + current_step_size);
        //println!("Calling rkf45 with dt = {}", current_step_size);
        let (deltas, used_step_size, mut next_step_size) = runge_kutta_fehlberg_45(
            &derivatives,
            &assignments,
            &local_parameters,
            &functions,
            current_step_size,
            rtol,
            atol,
            false,
        )?;
        if current_step_size != used_step_size {
            //println!("Tried {}, used {}", current_step_size, used_step_size);
        }
        current_step_size = used_step_size;
        //println!("Integrated from t = {} to {}", t, t + &current_step_size);
        //dbg!(t_next_result);
        // if the step size wasn't reduced and there's a valid step_size_cache,
        // try to use that in the next step
        if next_step_size > current_step_size {
            if let Some(cached_step_size_value) = cached_step_size {
                // use cache value only if it is better
                if next_step_size < cached_step_size_value {
                    //dbg!(cached_step_size_value);
                    next_step_size = cached_step_size_value;
                    //println!(
                    //"Will use cached step size of {} for next step",
                    //next_step_size
                    //);
                }
                // but reset cache regardless
                cached_step_size = None;
            }
        }

        for (key, val) in deltas.iter() {
            assignments.entry(key.to_string()).and_modify(|e| *e += val);
        }

        // evaluate assignment rules
        assignments = evaluate_rules(&assignment_rules, &mut assignments, &functions);

        // see if we reached a result_point in this iteration
        // if we did, increment t_next_result and store results
        if t_next_result - (t + current_step_size) < f64::EPSILON {
            //println!("Reached t = {}, storing results", t + current_step_size);
            t_next_result += result_interval;
            if t_next_result > duration {
                t_next_result = duration;
            }
            t_next_result = (t_next_result * 1000000.0).round() / 1000000.0;
            // create result object for this iteration
            let mut iteration_result = assignments.clone();
            iteration_result.insert("t".to_string(), t + current_step_size);

            results.push(iteration_result);
        }

        // update t
        t += current_step_size;
        // ensure next step doesn't overtake result points
        if (t + next_step_size) - t_next_result >= f64::EPSILON {
            //println!("t + next_step_size = {}", t + next_step_size);
            // save this value to use for the step after the result point
            cached_step_size = Some(next_step_size);
            next_step_size = t_next_result - t;
            //println!("changed next_step_size to {} and", next_step_size);
            //println!("saved {} in the cache", cached_step_size.unwrap());
        }
        current_step_size = next_step_size;
        //println!(
        //"Next step will be from t = {} to {} with step size {}",
        //t,
        //t + current_step_size,
        //current_step_size
        //);

        //let mut input_string = String::new();
        //stdin()
        //.read_line(&mut input_string)
        //.ok()
        //.expect("Failed to read line");
    }

    let mut result_amounts: Vec<HashMap<String, f64>> = Vec::new();
    for timestep in &results {
        let mut result_amounts_current = timestep.clone();
        for sp in &species {
            let compartment = sp.compartment.as_ref().unwrap();
            let species_id = sp.id.as_ref().unwrap();
            if let Some(concentration) = timestep.get(species_id) {
                let compartment_size = timestep.get(compartment).unwrap();
                let sp_amount = concentration * compartment_size;
                result_amounts_current.insert(species_id.to_owned(), sp_amount);
            }
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
        // TODO: Do not crash if compartment size doesn't exist

        for rxn_id in &reaction_ids {
            let kinetic_law = all_kinetic_laws.get(rxn_id).unwrap().to_owned();
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
                let derivative_term = DerivativeTerm::new(value, kinetic_law, rxn_id.to_string());
                derivatives
                    .entry(species_id.clone())
                    .or_insert_with(|| Derivative::new(compartment_size))
                    .add_term(derivative_term);
            }
        }
    }
    derivatives
}

pub fn evaluate_rules(
    rules: &HashMap<String, Vec<MathNode>>,
    assignments: &mut HashMap<String, f64>,
    functions: &HashMap<String, Vec<MathNode>>,
) -> HashMap<String, f64> {
    for (variable, math) in rules {
        if let Ok(value) = evaluate_node(math, 0, assignments, functions) {
            assignments.insert(variable.to_string(), value);
        }
    }

    assignments.clone()
}
