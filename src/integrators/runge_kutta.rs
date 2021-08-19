use crate::structs::bindings::Bindings;
use std::collections::HashMap;

#[allow(non_snake_case)]
pub fn runge_kutta_4(bindings: &Bindings, step_size: f64) -> Result<HashMap<String, f64>, String> {
    //
    let original_bindings = bindings.values();
    let mut k: HashMap<String, Vec<f64>> = HashMap::new();

    // K1
    let k1_assignments = HashMap::<String, f64>::new();
    for ODE in &bindings.ODEs {
        let dependent_variable = &ODE.id;
        k.insert(dependent_variable.clone(), Vec::new());
        let ode_value = ODE.evaluate(&k1_assignments, bindings)?;
        let k1 = step_size * ode_value;
        k.entry(dependent_variable.clone())
            .and_modify(|v| v.push(k1));
    }
    drop(k1_assignments);

    let mut k2_assignments: HashMap<String, f64> = HashMap::new();
    for (dependent_variable, k_current) in &k {
        let k1 = k_current[0];
        if let Some(original_value) = original_bindings.get(dependent_variable) {
            k2_assignments.insert(dependent_variable.to_owned(), original_value + k1 / 2.0);
        }
    }

    for (key, val) in bindings.emulate_assignment_rules(&k2_assignments) {
        k2_assignments.insert(key, val);
    }

    for ODE in &bindings.ODEs {
        let dependent_variable = &ODE.id;
        let ode_value = ODE.evaluate(&k2_assignments, bindings)?;
        let k2 = step_size * ode_value;
        k.entry(dependent_variable.clone())
            .and_modify(|v| v.push(k2));
    }
    drop(k2_assignments);

    let mut k3_assignments = HashMap::<String, f64>::new();
    for (dependent_variable, k_current) in &k {
        let k2 = k_current[1];
        if let Some(original_value) = original_bindings.get(dependent_variable) {
            k3_assignments.insert(dependent_variable.to_owned(), original_value + k2 / 2.0);
        }
    }

    for (key, val) in bindings.emulate_assignment_rules(&k3_assignments) {
        k3_assignments.insert(key, val);
    }

    // Calculate k3 values
    for ODE in &bindings.ODEs {
        let dependent_variable = &ODE.id;
        let ode_value = ODE.evaluate(&k3_assignments, bindings)?;
        let k3 = step_size * ode_value;
        k.entry(dependent_variable.clone())
            .and_modify(|v| v.push(k3));
    }
    drop(k3_assignments);

    let mut k4_assignments = HashMap::<String, f64>::new();
    for (dependent_variable, k_current) in &k {
        let k3 = k_current[2];
        if let Some(original_value) = original_bindings.get(dependent_variable) {
            k4_assignments.insert(dependent_variable.to_owned(), original_value + k3);
        }
    }

    for (key, val) in bindings.emulate_assignment_rules(&k4_assignments) {
        k4_assignments.insert(key, val);
    }

    // Calculate k4 values
    for ODE in &bindings.ODEs {
        let dependent_variable = &ODE.id;
        let ode_value = ODE.evaluate(&k4_assignments, bindings)?;
        let k4 = step_size * ode_value;
        k.entry(dependent_variable.clone())
            .and_modify(|v| v.push(k4));
    }
    drop(k4_assignments);

    // Calculate final changes to derivatives
    let mut deltas: HashMap<String, f64> = HashMap::new();
    for (dependent_variable, k_current) in &k {
        let k1 = k_current[0];
        let k2 = k_current[1];
        let k3 = k_current[2];
        let k4 = k_current[3];

        let delta = (k1 + k4) / 6.0 + (k2 + k3) / 3.0;
        deltas.insert(dependent_variable.to_owned(), delta);
    }

    Ok(deltas)
}
