use super::super::structs::derivative::Derivative;
use mathml_rs::MathNode;
use sbml_rs::structs::species;
use std::collections::HashMap;

pub fn runge_kutta_fehlberg_45(
    derivatives: &HashMap<String, Derivative>,
    assignments: &HashMap<String, f64>,
    local_parameters: &HashMap<String, HashMap<String, f64>>,
    functions: &HashMap<String, Vec<MathNode>>,
    step_size: f64,
    rtol: f64,
    atol: f64,
    debug: bool,
    prev_step_failed: bool,
) -> Result<(HashMap<String, f64>, f64, f64), String> {
    // COEFFICIENTS
    // Coefficients related to the independent variable of the evaluations
    //let a2 = 2.500000000000000e-01; //  1/4
    //let a3 = 3.750000000000000e-01; //  3/8
    //let a4 = 9.230769230769231e-01; //  12/13
    //let a5 = 1.000000000000000e+00; //  1
    //let a6 = 5.000000000000000e-01; //  1/2

    // Coefficients related to the dependent variable of the evaluations
    let b21 = 2.500000000000000e-01; //  1/4
    let b31 = 9.375000000000000e-02; //  3/32
    let b32 = 2.812500000000000e-01; //  9/32
    let b41 = 8.793809740555303e-01; //  1932/2197
    let b42 = -3.277196176604461e+00; // -7200/2197
    let b43 = 3.320892125625853e+00; //  7296/2197
    let b51 = 2.032407407407407e+00; //  439/216
    let b52 = -8.000000000000000e+00; // -8
    let b53 = 7.173489278752436e+00; //  3680/513
    let b54 = -2.058966861598441e-01; // -845/4104
    let b61 = -2.962962962962963e-01; // -8/27
    let b62 = 2.000000000000000e+00; //  2
    let b63 = -1.381676413255361e+00; // -3544/2565
    let b64 = 4.529727095516569e-01; //  1859/4104
    let b65 = -2.750000000000000e-01; // -11/40

    // Coefficients related to the truncation error
    // Obtained through the difference of the 5th and 4th order RK methods:
    //     R = (1/h)|y5_i+1 - y4_i+1|
    let r1 = 2.777777777777778e-03; //  1/360
    let r3 = -2.994152046783626e-02; // -128/4275
    let r4 = -2.919989367357789e-02; // -2197/75240
    let r5 = 2.000000000000000e-02; //  1/50
    let r6 = 3.636363636363636e-02; //  2/55

    // Coefficients related to RK 4th order method
    let c1 = 1.157407407407407e-01; //  25/216
    let c3 = 5.489278752436647e-01; //  1408/2565
    let c4 = 5.353313840155945e-01; //  2197/4104
    let c5 = -2.000000000000000e-01; // -1/5

    //println!("rkf45 called with dt = {}", step_size);
    //if prev_step_failed {
    //println!("{:?}", assignments);
    //}

    let mut k: HashMap<String, Vec<f64>> = HashMap::new();

    // K1
    for (species_id, derivative) in derivatives {
        k.insert(species_id.clone(), Vec::new());
        let derivative_value = derivative.evaluate(assignments, local_parameters, functions)?;
        let k1 = step_size * derivative_value;
        k.entry(species_id.clone()).and_modify(|v| v.push(k1));
    }

    // Prepare assignments for k2 according to
    // k2 = h * f( x + a2 * h, y + b21 * k1 )
    // k2_assignment = y + b21 * k1
    let mut k2_assignments = assignments.clone();
    for (species_id, k_current) in &k {
        let k1 = k_current[0];
        k2_assignments
            .entry(species_id.to_owned())
            .and_modify(|v| *v += k1 * b21);
    }

    // Calculate k2 values
    for (species_id, derivative) in derivatives {
        let derivative_value = derivative.evaluate(&k2_assignments, local_parameters, functions)?;
        let k2 = step_size * derivative_value;
        // k2 = h * f( x + a2 * h, y + b21 * k1 )
        k.entry(species_id.clone()).and_modify(|v| v.push(k2));
    }

    // Prepare assignments for k3 according to
    // k3 = h * f( x + a3 * h, y + b31 * k1 + b32 * k2 )
    // k3_assignment = y + b31 * k1 + b32 * k2
    let mut k3_assignments = assignments.clone();
    for (species_id, k_current) in &k {
        let k1 = k_current[0];
        let k2 = k_current[1];
        k3_assignments
            .entry(species_id.to_owned())
            .and_modify(|v| *v += k1 * b31 + k2 * b32);
    }

    // Calculate k3 values
    for (species_id, derivative) in derivatives {
        let derivative_value = derivative.evaluate(&k3_assignments, local_parameters, functions)?;
        let k3 = step_size * derivative_value;
        k.entry(species_id.clone()).and_modify(|v| v.push(k3));
    }

    // Prepare assignments for k4 according to
    // k4 = h * f( x + a4 * h, y + b41 * k1 + b42 * k2 + b43 * k3 )
    // k4_assignment = y + b41 * k1 + b42 * k2 + b43 * k3
    let mut k4_assignments = assignments.clone();
    for (species_id, k_current) in &k {
        let k1 = k_current[0];
        let k2 = k_current[1];
        let k3 = k_current[2];
        k4_assignments
            .entry(species_id.to_owned())
            .and_modify(|v| *v += k1 * b41 + k2 * b42 + k3 * b43);
    }

    // Calculate k4 values
    for (species_id, derivative) in derivatives {
        let derivative_value = derivative.evaluate(&k4_assignments, local_parameters, functions)?;
        let k4 = step_size * derivative_value;
        k.entry(species_id.clone()).and_modify(|v| v.push(k4));
    }

    // Prepare assignments for k5 according to
    // k5 = h * f( x + a5 * h, y + b51 * k1 + b52 * k2 + b53 * k3 + b54 * k4 )
    // k5_assignment = y + b51 * k1 + b52 * k2 + b53 * k3 + b54 * k4
    let mut k5_assignments = assignments.clone();
    for (species_id, k_current) in &k {
        let k1 = k_current[0];
        let k2 = k_current[1];
        let k3 = k_current[2];
        let k4 = k_current[3];
        k5_assignments
            .entry(species_id.to_owned())
            .and_modify(|v| *v += k1 * b51 + k2 * b52 + k3 * b53 + k4 * b54);
    }

    // Calculate k5 values
    for (species_id, derivative) in derivatives {
        let derivative_value = derivative.evaluate(&k4_assignments, local_parameters, functions)?;
        let k5 = step_size * derivative_value;
        k.entry(species_id.clone()).and_modify(|v| v.push(k5));
    }

    // Prepare assignments for k6 according to
    // k6 = h * f( x + a6 * h, y + b61 * k1 + b62 * k2 + b63 * k3 + b64 * k4 + b65 * k5 )
    // k6_assignment = y + b61 * k1 + b62 * k2 + b63 * k3 + b64 * k4 + b65 * k5
    let mut k6_assignments = assignments.clone();
    for (species_id, k_current) in &k {
        let k1 = k_current[0];
        let k2 = k_current[1];
        let k3 = k_current[2];
        let k4 = k_current[3];
        let k5 = k_current[4];
        k6_assignments
            .entry(species_id.to_owned())
            .and_modify(|v| *v += k1 * b61 + k2 * b62 + k3 * b63 + k4 * b64 + k5 * b65);
    }

    // Calculate k6 values
    for (species_id, derivative) in derivatives {
        let derivative_value = derivative.evaluate(&k4_assignments, local_parameters, functions)?;
        let k6 = step_size * derivative_value;
        k.entry(species_id.clone()).and_modify(|v| v.push(k6));
    }

    // Calculate final changes to derivatives
    let mut deltas: HashMap<String, f64> = HashMap::new();
    for (species_id, k_current) in &k {
        let k1 = k_current[0];
        let k3 = k_current[2];
        let k4 = k_current[3];
        let k5 = k_current[4];

        let delta = c1 * k1 + c3 * k3 + c4 * k4 + c5 * k5;
        deltas.insert(species_id.to_owned(), delta);
    }

    // Calculate local error for each equation
    let mut local_errors: HashMap<String, f64> = HashMap::new();
    for (species_id, k_current) in &k {
        let k1 = k_current[0];
        let k3 = k_current[2];
        let k4 = k_current[3];
        let k5 = k_current[4];
        let k6 = k_current[5];

        let local_error = (r1 * k1 + r3 * k3 + r4 * k4 + r5 * k5 + r6 * k6).abs();
        local_errors.insert(species_id.clone(), local_error);
    }

    // Estimate error
    // Error factor is the local error over for an equation
    // over the error tolerance for that equation
    let mut max_error_factor = 0.0;
    for (species_id, delta) in &deltas {
        if let Some(original_value) = assignments.get(species_id) {
            if let Some(local_error) = local_errors.get(species_id) {
                let final_value = original_value + delta;
                let error_tolerance = rtol * f64::max(*original_value, final_value) + atol;
                let error_factor = step_size * local_error / error_tolerance;
                if debug {
                    println!("species_id = {:?}", species_id);
                    println!("original_value = {:?}", original_value);
                    println!("final_value = {:?}", final_value);
                    //println!("max_value = {:?}", f64::max(*original_value, final_value));
                    //println!("rtol = {:?}", rtol);
                    //println!("atol = {:?}", atol);
                    println!("error_tolerance = {:?}", error_tolerance);
                    println!("local_error = {:?}", local_error);
                    println!("error_factor = {:?}", error_factor);
                }

                max_error_factor = f64::max(max_error_factor, error_factor);
            }
        }
    }

    let mut step_change_factor = 0.9 * 0.84 * (max_error_factor).powf(-0.25);

    if debug {
        println!("max_err_factor = {:?}", max_error_factor);
        println!("step_change_factor = {:?}", step_change_factor);
    }
    if max_error_factor < 1.0 {
        if debug {
            println!("Error acceptable");
        }
        // Increase step size if possible
        // If the previous step failed, increase is not allowed
        if prev_step_failed {
            if debug {
                println!("Not changing step size");
            }
            Ok((deltas, step_size, step_size))
        } else {
            // Calculate next step size
            // limit increase to a factor of 5
            if step_change_factor > 5.0 {
                step_change_factor = 5.0;
            }
            // don't decrease step size
            if step_change_factor < 1.0 {
                step_change_factor = 1.0;
            }

            if debug {
                println!(
                    "Increasing step size by a factor of {} to {}",
                    step_change_factor,
                    step_size * step_change_factor
                );
            }
            Ok((deltas, step_size, step_size * step_change_factor))
        }
    } else {
        if debug {
            println!("Current step failed, reducing step size");
        }
        // Calculate next step size
        // Limit decrease to a factor of 0.1
        if step_change_factor < 0.1 {
            step_change_factor = 0.1;
        }
        // don't increase step size
        if step_change_factor > 1.0 {
            step_change_factor = 1.0;
        }
        if debug {
            println!(
                "Calling rkf45 again with step_size = {}",
                step_size * step_change_factor
            );
        }
        //println!("{:?}", assignments);
        runge_kutta_fehlberg_45(
            derivatives,
            assignments,
            local_parameters,
            functions,
            step_size * step_change_factor,
            rtol,
            atol,
            debug,
            true,
        )
    }
}
