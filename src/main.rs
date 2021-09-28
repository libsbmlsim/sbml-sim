use csv;
use sbml_sim::simulate;
use sbml_sim::structs::methods::Methods;
use std::error::Error;
extern crate clap;
use clap::{App, Arg};
use std::collections::HashMap;

fn main() {
    let matches = App::new("SBML Simulator in Rust")
        .version("1.0")
        .author("Pranav Ballaney <ballaneypranav@gmail.com>")
        .about("A simulator for SBML models.")
        .arg(
            Arg::with_name("INPUT")
                .short("i")
                .takes_value(true)
                .help("Sets the input file to use")
                .required(true),
        )
        .arg(
            Arg::with_name("OUTPUT")
                .short("o")
                .takes_value(true)
                .help("Sets the output file")
                .required(true),
        )
        .arg(
            Arg::with_name("TIME")
                .short("t")
                .takes_value(true)
                .help("Simulation duration in seconds")
                .required(true),
        )
        .arg(
            Arg::with_name("STEPS")
                .short("s")
                .takes_value(true)
                .help("Number of steps for numerical integration")
                .required(true),
        )
        .arg(
            Arg::with_name("METHOD")
                .short("m")
                .takes_value(true)
                .help("Numerical integration algorithm")
                .required(false)
                .possible_values(&Methods::variants())
                .default_value("RKF45"),
        )
        .arg(
            Arg::with_name("RELATIVE_TOLERANCE")
                .long("rtol")
                .takes_value(true)
                .help("Relative error tolerance")
                .required(false)
                .default_value("1e-10"),
        )
        .arg(
            Arg::with_name("ABSOLUTE_TOLERANCE")
                .long("atol")
                .takes_value(true)
                .help("Absolute error tolerance")
                .required(false)
                .default_value("1e-16"),
        )
        .arg(
            Arg::with_name("amounts")
                .short("a")
                .multiple(false)
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
    //
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
    let input_filename = matches.value_of("INPUT").unwrap().to_string();
    let output_filename = matches.value_of("OUTPUT").unwrap().to_string();
    let time = matches.value_of("TIME").unwrap().parse::<f64>().unwrap();
    let steps = matches.value_of("STEPS").unwrap().parse::<i32>().unwrap();
    let method = matches
        .value_of("METHOD")
        .unwrap()
        .parse::<Methods>()
        .unwrap();
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
    println!("Using {} on input file: {}", method, input_filename);
    println!("{} seconds with {} steps.", time, steps);

    let debug = matches.is_present("debug");
    let print_amounts = matches.is_present("amounts");

    let result = simulate(
        input_filename,
        time,
        steps,
        method,
        rtol,
        atol,
        print_amounts,
        debug,
    );

    match write_csv(result, output_filename) {
        Ok(()) => {}
        Err(error) => {
            dbg!(error);
        }
    }
}

fn write_csv(
    result: HashMap<String, Vec<f64>>,
    output_filename: String,
) -> Result<(), Box<dyn Error>> {
    let mut headings: Vec<String> = Vec::new();
    for key in result.keys() {
        if key != "t" {
            headings.push(key.to_string());
        }
    }
    headings.sort();
    headings.insert(0, "time".to_string());
    let length = result.get("time").unwrap().len();

    let mut wtr = csv::Writer::from_path(output_filename).unwrap();
    wtr.write_record(&headings)?;
    for i in 0..length {
        let mut record = Vec::new();
        for heading in &headings {
            record.push(result.get(heading).unwrap()[i].to_string());
        }
        wtr.write_record(&record)?;
    }

    wtr.flush()?;
    Ok(())
}
