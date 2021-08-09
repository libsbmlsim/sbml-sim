mod integrators;
use integrators::main::*;
pub mod structs;
pub use structs::assignment_rule::*;
pub use structs::compartment::*;
pub use structs::derivative::*;
pub use structs::function::*;
pub use structs::initial_assignment::*;
pub use structs::local_parameter::*;
pub use structs::parameter::*;
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
                .required(true),
        )
        .arg(
            Arg::with_name("ABSOLUTE_TOLERANCE")
                .help("Absolute error tolerance")
                .required(true),
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

    let DEBUG = matches.is_present("debug");

    //let step_size = time / (steps as f64) / 4096.0;
    let step_size = time / (steps as f64);
    let model = sbml_rs::parse(&filename).expect("Couldn't parse model.");
    let result = integrate(&model, time, steps, step_size, rtol, atol, DEBUG).unwrap();

    print!("t       \t");
    for sp in &model.species() {
        print!("{}\t\t\t", sp.id.as_ref().unwrap());
    }
    println!();
    for iteration in result.iter().step_by(1) {
        let t = iteration.get("t").unwrap();
        print!("{:.6}\t", t);
        for sp in &model.species() {
            print!(
                "{:.20}\t",
                iteration
                    .get(sp.id.as_ref().expect("Species ID not found"))
                    .expect("No data for species")
            );
        }
        println!();
    }
}
