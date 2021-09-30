mod integrators;
use integrators::main::*;
pub mod structs;
pub use structs::assignment_rule::*;
pub use structs::compartment::*;
pub use structs::derivative::*;
pub use structs::initial_assignment::*;
pub use structs::local_parameter::*;
pub use structs::methods::*;
pub use structs::parameter::*;
pub use structs::rate_rule::*;
pub use structs::reaction::*;
pub use structs::species::*;

use std::collections::HashMap;

pub fn simulate(
    model_filename: String,
    time: f64,
    steps: i32,
    method: Methods,
    rtol: f64,
    atol: f64,
    print_amounts: bool,
    debug: bool,
) -> HashMap<String, Vec<f64>> {
    let step_size = time / (steps as f64);

    let model = sbml_rs::parse_and_transform(&model_filename).expect("Couldn't parse model.");

    let result = integrate(
        &model,
        time,
        steps,
        step_size,
        method,
        rtol,
        atol,
        print_amounts,
        debug,
    )
    .unwrap();
    flatten(result)
}

// Flattens a vector of hashmaps to a hashmap of vectors
fn flatten(input: Vec<HashMap<String, f64>>) -> HashMap<String, Vec<f64>> {
    if input.len() == 0 {
        return HashMap::new();
    }

    let mut result = HashMap::<String, Vec<f64>>::new();
    for row in input {
        for (mut key, value) in row {
            if key == "t" {
                key = "time".to_string();
            }
            result.entry(key).or_insert(Vec::<f64>::new()).push(value);
        }
    }

    result
}
