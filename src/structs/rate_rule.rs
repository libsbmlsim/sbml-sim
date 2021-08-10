use mathml_rs::MathNode;
use sbml_rs::{self, Model};

#[derive(Clone, Debug)]
pub struct RateRule {
    pub id: Option<String>,
    pub variable: String,
    pub math: Vec<MathNode>,
    pub sbo_term: Option<String>,
}

impl RateRule {
    pub fn from(rule: &sbml_rs::RateRule, model: &Model) -> Self {
        let variable = rule
            .variable
            .as_ref()
            .expect("Variable attribute is mandatory for RateRules.")
            .clone();
        let math = rule
            .math_tag(model)
            .expect("Can't simulate RateRule without a Math tag.")
            .nodes;
        RateRule {
            id: rule.id.clone(),
            variable,
            math,
            sbo_term: rule.sbo_term.clone(),
        }
    }
}
