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

As of now, only Euler, RK45 and RKF methods have been implemented.
To try it out, run 
```
cargo run -- -i {path to model} -t 10 -s 100 -o out.csv -a
```
This runs the RKF45 integrator for 10 seconds over 100 integration steps.

To test, run:
```
cargo test --test core-semantic -- --test-threads=8 --logfile tests/tests.log > tests/tests.full.log
awk '{ print $2" "$1 }' tests/tests.log | sort -u > tests/tests.log.awked; mv tests/tests.log.awked tests/tests.log
source tests/venv/bin/activate
python3 tests/analyze.py tests/tests.log
```

