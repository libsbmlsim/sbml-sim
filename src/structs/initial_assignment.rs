use mathml_rs::{get_dependent_variables, MathNode};
use sbml_rs::{self, Model};
use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub struct InitialAssignment {
    pub id: Option<String>,
    pub symbol: String,
    pub math: Vec<MathNode>,
    pub dependent_variables: Vec<String>,
    pub topological_order: usize,
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
        let dependent_variables = get_dependent_variables(&math);
        InitialAssignment {
            id: assignment.id.clone(),
            symbol,
            math,
            dependent_variables,
            topological_order: 0,
            sbo_term: assignment.sbo_term.clone(),
        }
    }
    pub fn from_assignment_rule(assignment: &sbml_rs::AssignmentRule, model: &Model) -> Self {
        let symbol = assignment
            .variable
            .as_ref()
            .expect("ID is mandatory for InitialAssignments.")
            .clone();
        let math = assignment
            .math_tag(model)
            .expect("Can't simulator InitialAssignment without a Math tag.")
            .nodes;
        let dependent_variables = get_dependent_variables(&math);
        InitialAssignment {
            id: assignment.id.clone(),
            symbol,
            math,
            dependent_variables,
            topological_order: 0,
            sbo_term: assignment.sbo_term.clone(),
        }
    }
}

impl Ord for InitialAssignment {
    fn cmp(&self, other: &Self) -> Ordering {
        self.topological_order.cmp(&other.topological_order)
    }
}

impl PartialOrd for InitialAssignment {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.topological_order.partial_cmp(&other.topological_order)
    }
}

impl PartialEq for InitialAssignment {
    fn eq(&self, other: &Self) -> bool {
        self.topological_order == other.topological_order
    }
}

impl Eq for InitialAssignment {}
