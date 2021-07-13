use mathml_rs::evaluate_node;
use sbml_rs::MathTag;
use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
};

#[derive(Debug, Default)]
pub struct Derivative {
    terms: Vec<(f64, MathTag)>,
}

impl Derivative {
    pub fn push(&mut self, coefficient: f64, math_tag: MathTag) {
        self.terms.push((coefficient, math_tag));
    }

    pub fn evaluate(&self, assignments: &HashMap<String, f64>) -> Result<f64, String> {
        let mut result = 0.0;
        for term in &self.terms {
            result += term.0 * evaluate_node(&term.1.nodes, 0, assignments)?;
        }
        Ok(result)
    }
}

impl Display for Derivative {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        for term in &self.terms {
            write!(f, "{} * ", term.0)?;
            write!(f, "{}", &term.1)?;
        }
        Ok(())
    }
}
