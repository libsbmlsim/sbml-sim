//use mathml_rs::Op;
use sbml_rs;
//use sbml_rs::MathNode;
//use sbml_rs::TagIndex;
use std::env;
mod integrators;
use integrators::euler::*;

fn main() {
    let mut args = env::args();
    let filename = args
        .nth(1)
        .expect("Please provide the filename of an SBML model as a command line argument.")
        .to_owned();
    let model = sbml_rs::parse(&filename).expect("Couldn't parse model.");
    let time = 5.0;
    let step_size = 0.05;

    let result = euler_integrator(&model, time, step_size).unwrap();

    print!("t\t");
    for sp in &model.species() {
        print!("{}\t\t", sp.id.as_ref().unwrap());
    }
    println!();
    for iteration in result.iter().step_by(2) {
        let mut t = iteration.get("t").unwrap();
        print!("{:.2}\t", t);
        for sp in &model.species() {
            print!("{:.7}\t", iteration.get(sp.id.as_ref().unwrap()).unwrap());
        }
        println!();
    }
}
