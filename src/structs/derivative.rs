use mathml_rs::{evaluate_node, MathNode};
use std::collections::HashMap;

use super::bindings::Bindings;

#[derive(Debug, Clone)]
pub struct ODE {
    pub id: String,
    terms: Vec<ODETerm>,
    compartment: Option<String>,
}

impl ODE {
    pub fn new(id: String, compartment: Option<String>) -> Self {
        ODE {
            id,
            terms: Vec::new(),
            compartment,
        }
    }

    pub fn add_term(&mut self, term: ODETerm) {
        self.terms.push(term);
    }

    pub fn evaluate(
        &self,
        updated_assignments: &HashMap<String, f64>,
        bindings: &Bindings,
    ) -> Result<f64, String> {
        let mut result = 0.0;
        let mut assignments: HashMap<String, f64> = bindings.values();
        for (key, value) in updated_assignments {
            assignments.insert(key.clone(), *value);
            //println!("{}: {}", key, value);
        }
        for term in &self.terms {
            let mut rxn_assignments = assignments.clone();
            if let Some(reaction) = bindings.reactions.get(&term.rxn_id) {
                for (key, value) in reaction.local_parameter_values() {
                    rxn_assignments.insert(key, value);
                }
            }
            result += term.coefficient
                * evaluate_node(&term.math, 0, &rxn_assignments, &bindings.functions)?;
        }
        if let Some(compartment_var) = &self.compartment {
            let compartment = assignments.get(compartment_var).expect("Factor not found.");
            result = result * compartment;
            //println!("divided by compartment {}", compartment);
        }
        Ok(result)
    }
}

#[derive(Debug, Clone)]
pub struct ODETerm {
    coefficient: f64,
    math: Vec<MathNode>,
    rxn_id: String,
}

impl ODETerm {
    pub fn new(coefficient: f64, math: Vec<MathNode>, rxn_id: String) -> Self {
        ODETerm {
            coefficient,
            math,
            rxn_id,
        }
    }
}
