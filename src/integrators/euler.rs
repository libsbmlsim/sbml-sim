use mathml_rs::MathNode;
use std::collections::HashMap;

use crate::ODE;

#[allow(dead_code)]
pub fn euler(
    derivatives: &HashMap<String, ODE>,
    assignments: &HashMap<String, f64>,
    local_parameters: &HashMap<String, HashMap<String, f64>>,
    functions: &HashMap<String, Vec<MathNode>>,
    step_size: f64,
) -> Result<HashMap<String, f64>, String> {
    //
    let mut deltas: HashMap<String, f64> = HashMap::new();
    for species_id in derivatives.keys() {
        let derivative = derivatives.get(species_id).unwrap();
        //let derivative_value = derivative.evaluate(assignments, local_parameters, functions)?;
        //let delta = step_size * derivative_value;
        //deltas.insert(species_id.to_owned(), delta);
    }
    Ok(deltas)
}
