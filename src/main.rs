mod integrators;
use integrators::main::*;
pub mod structs;
pub use structs::assignment_rule::*;
pub use structs::compartment::*;
pub use structs::derivative::*;
pub use structs::initial_assignment::*;
pub use structs::local_parameter::*;
pub use structs::parameter::*;
pub use structs::rate_rule::*;
pub use structs::reaction::*;
pub use structs::species::*;
extern crate clap;
use clap::{App, Arg};

fn main() {
    let matches = App::new("SBML Simulator in Rust")
        .version("1.0")
        .author("Pranav Ballaney <ballaneypranav@gmail.com>")
        .about("A simulator for SBML models.")
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("TIME")
                .help("Simulation duration in seconds")
                .required(true),
        )
        .arg(
            Arg::with_name("STEPS")
                .help("Number of steps for numerical integration")
                .required(true),
        )
        .arg(
            Arg::with_name("RELATIVE_TOLERANCE")
                .help("Relative error tolerance")
                .required(false)
                .default_value("1e-10"),
        )
        .arg(
            Arg::with_name("ABSOLUTE_TOLERANCE")
                .help("Absolute error tolerance")
                .required(false)
                .default_value("1e-16"),
        )
        .arg(
            Arg::with_name("amounts")
                .short("a")
                .help("Print amounts instead of concentrations"),
        )
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .arg(
            Arg::with_name("debug")
                .short("d")
                .help("Print debug information"),
        )
        .get_matches();

    // Vary the output based on how many times the user used the "verbose" flag
    // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
    match matches.occurrences_of("v") {
        0 => {}
        1 => println!("Some verbose info"),
        2 => println!("Tons of verbose info"),
        _ => println!("Don't be crazy"),
    }

    // Calling .unwrap() is safe here because "INPUT" is required (if "INPUT" wasn't
    // required we could have used an 'if let' to conditionally get the value)
    let filename = matches.value_of("INPUT").unwrap();
    let time = matches.value_of("TIME").unwrap().parse::<f64>().unwrap();
    let steps = matches.value_of("STEPS").unwrap().parse::<i32>().unwrap();
    let rtol = matches
        .value_of("RELATIVE_TOLERANCE")
        .unwrap()
        .parse::<f64>()
        .unwrap();
    let atol = matches
        .value_of("ABSOLUTE_TOLERANCE")
        .unwrap()
        .parse::<f64>()
        .unwrap();
    println!("Using input file: {}", filename);
    println!("{} seconds with {} steps.", time, steps);
    //println!("atol: {}, rtol: {}", atol, rtol);

    let DEBUG = matches.is_present("debug");
    let print_amounts = matches.is_present("amounts");

    //let step_size = time / (steps as f64) / 4096.0;
    let step_size = time / (steps as f64);
    let model = sbml_rs::parse(&filename).expect("Couldn't parse model.");
    let result = integrate(
        &model,
        time,
        steps,
        step_size,
        rtol,
        atol,
        print_amounts,
        DEBUG,
    )
    .unwrap();

    print!("t       \t");
    let mut headings = Vec::<String>::new();
    for heading in result.iter().nth(1).unwrap().keys() {
        if heading != "t" {
            headings.push(heading.clone());
        }
    }
    headings.sort();
    for heading in &headings {
        print!("{:24}", heading);
    }
    println!();
    for iteration in result.iter().step_by(1) {
        let t = iteration.get("t").unwrap();
        print!("{:.6}\t", t);
        for heading in &headings {
            print!("{:.20}\t", iteration.get(heading).unwrap());
        }
        println!();
    }
}
