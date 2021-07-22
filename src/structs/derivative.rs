use mathml_rs::{evaluate_node, MathNode};
use sbml_rs::MathTag;
use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
};

#[derive(Debug)]
pub struct Derivative {
    terms: Vec<DerivativeTerm>,
    compartment_size: f64,
}

impl Derivative {
    pub fn new(compartment_size: f64) -> Self {
        Derivative {
            terms: Vec::new(),
            compartment_size,
        }
    }

    pub fn add_term(&mut self, term: DerivativeTerm) {
        self.terms.push(term);
    }

    pub fn evaluate(
        &self,
        assignments: &HashMap<String, f64>,
        local_parameters: &HashMap<String, HashMap<String, f64>>,
        functions: &HashMap<String, Vec<MathNode>>,
    ) -> Result<f64, String> {
        let mut result = 0.0;
        for term in &self.terms {
            let mut rxn_assignments: HashMap<String, f64> = assignments.clone();
            if let Some(local_param_hm) = local_parameters.get(&term.rxn_id) {
                for (key, value) in local_param_hm {
                    rxn_assignments.insert(key.clone(), *value);
                }
            }
            result +=
                term.coefficient * evaluate_node(&term.math.nodes, 0, &rxn_assignments, functions)?;
        }
        result /= self.compartment_size;
        Ok(result)
    }
}

impl Display for Derivative {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        for term in &self.terms {
            write!(f, "{} * ", term.coefficient)?;
            write!(f, "{}", &term.math)?;
            write!(f, "{}", &term.rxn_id)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct DerivativeTerm {
    coefficient: f64,
    math: MathTag,
    rxn_id: String,
}

impl DerivativeTerm {
    pub fn new(coefficient: f64, math: MathTag, rxn_id: String) -> Self {
        DerivativeTerm {
            coefficient,
            math,
            rxn_id,
        }
    }
}
