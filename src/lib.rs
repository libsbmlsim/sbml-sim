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

pub fn simulate(
    model_filename: String,
    time: f64,
    steps: i32,
    method: Methods,
    rtol: f64,
    atol: f64,
    print_amounts: bool,
    debug: bool,
) {
    let step_size = time / (steps as f64);
    let model =
        sbml_rs::parse_with_converted_species(&model_filename).expect("Couldn't parse model.");
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

    print!("t           \t");
    // print!("t      ");
    let mut headings = Vec::<String>::new();
    for heading in result.iter().nth(1).unwrap().keys() {
        if heading != "t" {
            headings.push(heading.clone());
        }
    }
    headings.sort();
    for heading in &headings {
        print!("{:24}", heading);
        //print!("{:16}", heading);
    }
    println!();
    for iteration in result.iter().step_by(1) {
        let t = iteration.get("t").unwrap();
        print!("{:.10}\t", t);
        //print!("{:.2}  ", t);
        for heading in &headings {
            print!("{:.20}\t", iteration.get(heading).unwrap());
            //print!("{:.12}\t", iteration.get(heading).unwrap());
        }
        println!();
    }
}
