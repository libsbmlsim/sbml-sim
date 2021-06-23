use sbml_rs;

fn main() {
    let filename = "models/small.xml";
    let model = sbml_rs::parse(filename);
    println!("{:?}", model);
}
