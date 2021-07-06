A simulator for SBML models.

Currently only parses a model using [sbml-rs](https://crates.io/crates/sbml-rs) and [mathml-rs](https://crates.io/crates/mathml-rs) with limited support.

To try out, run the following: 
```
git clone https://github.com/ballaneypranav/sbml-sim
git clone https://github.com/ballaneypranav/sbml-rs
git clone https://github.com/ballaneypranav/mathml-rs
cd sbml-sim
cargo run [path to model]
```

As of now, only Euler method has been implemented.
To try it out, run 
```
cargo run [path to model]
```

This runs the Euler integrator for 5 seconds with a step size of 0.05.

