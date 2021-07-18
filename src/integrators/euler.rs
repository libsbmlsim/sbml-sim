use super::super::structs::derivative::Derivative;
use std::collections::HashMap;

pub fn euler(
    derivatives: &HashMap<String, Derivative>,
    assignments: &HashMap<String, f64>,
    step_size: f64,
) -> Result<HashMap<String, f64>, String> {
    //
    let mut deltas: HashMap<String, f64> = HashMap::new();
    for species_id in derivatives.keys() {
        //dbg!(species_id);
        let derivative = derivatives.get(species_id).unwrap();
        //println!("{}", derivative);
        let derivative_value = derivative.evaluate(assignments)?;
        //dbg!(derivative_value);
        let delta = step_size * derivative_value;
        //dbg!(delta);
        deltas.insert(species_id.to_owned(), delta);
    }
    Ok(deltas)
}
