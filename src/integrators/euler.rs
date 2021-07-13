use super::super::structs::derivative::Derivative;
use mathml_rs::evaluate_node;
use std::collections::HashMap;

pub fn euler(
    derivative: &Derivative,
    assignments: &HashMap<String, f64>,
    step_size: f64,
) -> Result<f64, String> {
    //
    let derivative_value = derivative.evaluate(assignments)?;
    let change = step_size * derivative_value;
    Ok(change)
}
