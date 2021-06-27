A simulator for SBML models.

Currently only parses a model using [sbml-rs](https://crates.io/crates/sbml-rs) and [mathml-rs](https://crates.io/crates/mathml-rs) with limited support.

To try out, run the following: 
```
git clone https://github.com/ballaneypranav/sbml-sim
git clone https://github.com/ballaneypranav/sbml-rs
git clone https://github.com/ballaneypranav/mathml-rs
cd sbml-sim
cargo run
```

As of now, this can only evaluate an AST with the +, -, * and / operations.
To try it out, run 
```
cargo run [path to model]
```

If there are variables in your AST, their values must be specified through a hashmap in the main function.
