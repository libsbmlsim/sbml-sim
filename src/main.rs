mod integrators;
use integrators::main::*;
mod structs;
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
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
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
    println!("Using input file: {}", filename);
    println!("{} seconds with {} steps.", time, steps);

    let step_size = time / (steps as f64) / 4096.0;
    //let step_size = time / (steps as f64);

    let model = sbml_rs::parse(&filename).expect("Couldn't parse model.");
    let result = integrate(&model, time, step_size).unwrap();

    print!("t\t");
    for sp in &model.species() {
        print!("{}\t\t\t", sp.id.as_ref().unwrap());
    }
    println!();
    for iteration in result.iter().step_by(4096) {
        let t = iteration.get("t").unwrap();
        print!("{:.2}\t", t);
        for sp in &model.species() {
            print!("{:.15}\t", iteration.get(sp.id.as_ref().unwrap()).unwrap());
        }
        println!();
    }
}
