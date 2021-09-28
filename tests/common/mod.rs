use sbml_sim::simulate;
use sbml_sim::Methods;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Parameters {
    pub duration: f64,
    pub steps: i32,
    pub rtol: f64,
    pub atol: f64,
    pub model_filename: String,
    pub result_filename: String,
}

pub fn run_test(test_n: i32) -> bool {
    let testsuite_root = "../testsuites/core-semantic/".to_string();
    let parameters = get_parameters(testsuite_root, test_n).expect("Couldn't read parameters.");
    let method = Methods::RKF45;
    let print_amounts = true;
    let debug = false;
    let simulation_rtol = 1e-10;
    let simulation_atol = 1e-16;
    let simulation_result = simulate(
        parameters.model_filename,
        parameters.duration,
        parameters.steps,
        method,
        simulation_rtol,
        simulation_atol,
        print_amounts,
        debug,
    );
    let standard_results = get_standard_results(parameters.result_filename).unwrap();
    let comparison_result = compare(
        simulation_result,
        standard_results,
        parameters.rtol,
        parameters.atol,
    );
    comparison_result
}

pub fn get_parameters(mut testsuite_root: String, n: i32) -> Result<Parameters, Box<dyn Error>> {
    if !testsuite_root.ends_with("/") {
        testsuite_root = testsuite_root + "/";
    }
    let model_root = std::format!("{}/{:05}/{:05}-", testsuite_root, n, n);

    let parameter_filename = model_root.clone() + "settings.txt";
    let model_filename = model_root.clone() + "sbml-l3v2.xml";
    let result_filename = model_root.clone() + "results.csv";

    let reader = BufReader::new(File::open(parameter_filename)?);

    let mut parameters = HashMap::<String, String>::new();
    for line in reader.lines() {
        let words: Vec<String> = line
            .unwrap()
            .split(":")
            .map(|s| s.trim().to_string())
            .collect();
        if words.len() != 2 {
            continue;
        }
        parameters.insert(words[0].to_string(), words[1].to_string());
    }

    Ok(Parameters {
        duration: parameters.get("duration").unwrap().parse::<f64>().unwrap(),
        steps: parameters.get("steps").unwrap().parse::<i32>().unwrap(),
        rtol: parameters.get("relative").unwrap().parse::<f64>().unwrap(),
        atol: parameters.get("absolute").unwrap().parse::<f64>().unwrap(),
        model_filename,
        result_filename,
    })
}

pub fn get_standard_results(filename: String) -> Result<HashMap<String, Vec<f64>>, Box<dyn Error>> {
    let file = File::open(filename)?;
    let mut rdr = csv::Reader::from_reader(file);
    let headers: Vec<String> = rdr.headers()?.iter().map(|s| s.to_string()).collect();
    let mut result = HashMap::<String, Vec<f64>>::new();

    for record_result in rdr.records() {
        let record = record_result?;
        for i in 0..record.len() {
            let field = record[i].parse::<f64>().unwrap();
            result
                .entry(headers[i].clone())
                .or_insert(Vec::<f64>::new())
                .push(field);
        }
    }
    Ok(result)
}

pub fn compare(
    a_hm: HashMap<String, Vec<f64>>,
    b_hm: HashMap<String, Vec<f64>>,
    rtol: f64,
    atol: f64,
) -> bool {
    let mut result = true;
    for col in b_hm.keys() {
        if let Some(a) = a_hm.get(col) {
            if let Some(b) = b_hm.get(col) {
                result = result && all_close(a, b, rtol, atol);
            } else {
                println!("{} not found in a_hm", col);
                result = false;
            }
        } else {
            println!("{} not found in b_hm", col);
            result = false;
        }
        if !result {
            println!("{} doesn't match", col);
            return false;
        }
    }
    result
}

fn all_close(a: &Vec<f64>, b: &Vec<f64>, rtol: f64, atol: f64) -> bool {
    let mut result = true;
    assert_eq!(a.len(), b.len());
    for i in 0..b.len() {
        let tolerance = (rtol * b[i].abs()) + atol;
        let abs_diff = (a[i] - b[i]).abs();
        let current_result = abs_diff < tolerance;
        result = result && current_result;
        if !result {
            dbg!(a[i], b[i], abs_diff, tolerance);
            return false;
        }
    }
    result
}
