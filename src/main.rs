use sbml_rs;
use std::env;

fn main() {
    let mut args = env::args();
    let filename = args
        .nth(1)
        .expect("Please provide the filename of an SBML model as a command line argument.")
        .to_owned();
    let model = sbml_rs::parse(&filename);
    println!("{:?}", model);
}
