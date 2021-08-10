use crate::structs::bindings::Bindings;

use super::runge_kutta_fehlberg::runge_kutta_fehlberg_45;
use sbml_rs::Model;
use std::collections::HashMap;
use std::io::stdin;

pub fn integrate(
    model: &Model,
    duration: f64,
    steps: i32,
    init_step_size: f64,
    rtol: f64,
    atol: f64,
    print_amounts: bool,
    DEBUG: bool,
) -> Result<Vec<HashMap<String, f64>>, String> {
    // number of steps let steps = (time / step_size).ceil() as i32;
    // vector to store results
    let mut results = Vec::new();

    let mut bindings = Bindings::from(&model);
    //dbg!(&bindings);

    // store first result as initial values
    let mut initial_results = bindings.results(print_amounts);
    initial_results.insert("t".to_string(), 0.0);
    results.push(initial_results);

    let mut t = 0.0;
    // the interval at which results are required
    let result_interval = duration / (steps as f64);
    let mut t_next_result = ((t + result_interval) * 1000000.0).round() / 1000000.0;

    let mut current_step_size = init_step_size;
    // this is used if the step size was adjusted in the previous step to hit a result point
    // used only if the associated boolean value is true
    let mut cached_step_size = None;

    while duration - t > f64::EPSILON {
        if DEBUG {
            println!();
            println!("Integrating from {} to {}", t, t + current_step_size);
            println!("Calling rkf45 with dt = {}", current_step_size);
        }
        let (deltas, used_step_size, mut next_step_size) =
            runge_kutta_fehlberg_45(&bindings, current_step_size, rtol, atol, DEBUG, false)?;
        current_step_size = used_step_size;
        if DEBUG {
            if current_step_size != used_step_size {
                println!("Tried {}, used {}", current_step_size, used_step_size);
            }
            println!("Integrated from t = {} to {}", t, t + &current_step_size);
        }
        // if the step size wasn't reduced and there's a valid step_size_cache,
        // try to use that in the next step
        if next_step_size > current_step_size {
            if let Some(cached_step_size_value) = cached_step_size {
                // use cache value only if it is better
                if next_step_size < cached_step_size_value {
                    next_step_size = cached_step_size_value;
                    if DEBUG {
                        println!(
                            "Will use cached step size of {} for next step",
                            next_step_size
                        );
                    }
                }
                // but reset cache regardless
                cached_step_size = None;
            }
        }

        for (key, val) in deltas.iter() {
            bindings.update_delta(key, *val);
        }

        // evaluate assignment rules
        bindings.evaluate_assignment_rules();

        // see if we reached a result_point in this iteration
        // if we did, increment t_next_result and store results
        if t_next_result - (t + current_step_size) < f64::EPSILON {
            if DEBUG {
                println!("Reached t = {}, storing results", t + current_step_size);
            }
            t_next_result += result_interval;
            if t_next_result > duration {
                t_next_result = duration;
            }
            t_next_result = (t_next_result * 1000000.0).round() / 1000000.0;
            // create result object for this iteration
            let mut iteration_result = bindings.results(print_amounts);
            iteration_result.insert("t".to_string(), t + current_step_size);

            //dbg!(iteration_result);
            //panic!();
            results.push(iteration_result);
        }

        // update t
        t += current_step_size;
        // ensure next step doesn't overtake result points
        if (t + next_step_size) - t_next_result >= f64::EPSILON {
            // save this value to use for the step after the result point
            cached_step_size = Some(next_step_size);
            next_step_size = t_next_result - t;
        }
        current_step_size = next_step_size;
        if DEBUG {
            println!(
                "Next step will be from t = {} to {} with step size {}",
                t,
                t + current_step_size,
                current_step_size
            );
        }

        if DEBUG {
            println!("Press return to continue.");
            let mut input_string = String::new();
            stdin()
                .read_line(&mut input_string)
                .ok()
                .expect("Failed to read line");
        }
    }

    Ok(results)
}
