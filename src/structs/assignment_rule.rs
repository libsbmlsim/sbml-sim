use mathml_rs::MathNode;
use sbml_rs::{self, Model};

#[derive(Clone, Debug)]
pub struct AssignmentRule {
    pub id: Option<String>,
    pub variable: String,
    pub math: Vec<MathNode>,
    pub sbo_term: Option<String>,
}

impl AssignmentRule {
    pub fn from(rule: &sbml_rs::AssignmentRule, model: &Model) -> Self {
        let variable = rule
            .variable
            .as_ref()
            .expect("Variable attribute is mandatory for AssignmentRules.")
            .clone();
        let math = rule
            .math_tag(model)
            .expect("Can't simulate AssignmentRule without a Math tag.")
            .nodes;
        AssignmentRule {
            id: rule.id.clone(),
            variable,
            math,
            sbo_term: rule.sbo_term.clone(),
        }
    }
}
