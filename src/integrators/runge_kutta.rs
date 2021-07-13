use super::super::structs::derivative::Derivative;
use std::collections::HashMap;

pub fn runge_kutta_4(
    derivatives: &HashMap<String, Derivative>,
    assignments: &HashMap<String, f64>,
    step_size: f64,
) -> Result<HashMap<String, f64>, String> {
    //

    let mut k1: HashMap<String, f64> = HashMap::new();
    let mut k2_assignments = assignments.clone();
    for species_id in derivatives.keys() {
        let derivative = derivatives.get(species_id).unwrap();
        let derivative_value = derivative.evaluate(assignments)?;
        let k1_current = step_size * derivative_value;
        k1.insert(species_id.to_string(), k1_current);

        k2_assignments
            .entry(species_id.to_owned())
            .and_modify(|v| *v += k1_current / 2.0);
    }

    let mut k2: HashMap<String, f64> = HashMap::new();
    let mut k3_assignments = assignments.clone();
    for species_id in derivatives.keys() {
        let derivative = derivatives.get(species_id).unwrap();
        let derivative_value = derivative.evaluate(&k2_assignments)?;
        let k2_current = step_size * derivative_value;
        k2.insert(species_id.to_string(), k2_current);

        k3_assignments
            .entry(species_id.to_owned())
            .and_modify(|v| *v += k2_current / 2.0);
    }

    let mut k3: HashMap<String, f64> = HashMap::new();
    let mut k4_assignments = assignments.clone();
    for species_id in derivatives.keys() {
        let derivative = derivatives.get(species_id).unwrap();
        let derivative_value = derivative.evaluate(&k3_assignments)?;
        let k3_current = step_size * derivative_value;
        k3.insert(species_id.to_string(), k3_current);

        k4_assignments
            .entry(species_id.to_owned())
            .and_modify(|v| *v += k3_current);
    }

    let mut k4: HashMap<String, f64> = HashMap::new();
    for species_id in derivatives.keys() {
        let derivative = derivatives.get(species_id).unwrap();
        let derivative_value = derivative.evaluate(&k4_assignments)?;
        let k4_current = step_size * derivative_value;
        k4.insert(species_id.to_string(), k4_current);
    }

    let mut deltas: HashMap<String, f64> = HashMap::new();
    for species_id in derivatives.keys() {
        let k1_current = k1.get(species_id).unwrap().to_owned();
        let k2_current = k2.get(species_id).unwrap().to_owned();
        let k3_current = k3.get(species_id).unwrap().to_owned();
        let k4_current = k4.get(species_id).unwrap().to_owned();

        let delta = (k1_current + 2.0 * k2_current + 2.0 * k3_current + k4_current) / 6.0;
        deltas.insert(species_id.to_owned(), delta);
    }

    Ok(deltas)
}
