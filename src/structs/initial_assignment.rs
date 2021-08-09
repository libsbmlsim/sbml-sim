use mathml_rs::MathNode;
use sbml_rs::{self, Model};

#[derive(Debug, Clone)]
pub struct InitialAssignment {
    pub id: Option<String>,
    pub symbol: String,
    pub math: Vec<MathNode>,
    pub sbo_term: Option<String>,
}

impl InitialAssignment {
    pub fn from(assignment: &sbml_rs::InitialAssignment, model: &Model) -> Self {
        let symbol = assignment
            .symbol
            .as_ref()
            .expect("ID is mandatory for InitialAssignments.")
            .clone();
        let math = assignment
            .math_tag(model)
            .expect("Can't simulator InitialAssignment without a Math tag.")
            .nodes;
        InitialAssignment {
            id: assignment.id.clone(),
            symbol,
            math,
            sbo_term: assignment.sbo_term.clone(),
        }
    }
}
