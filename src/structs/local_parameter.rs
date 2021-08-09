use sbml_rs;

#[derive(Clone, Debug)]
pub struct LocalParameter {
    pub id: String,
    pub value: f64,
    pub sbo_term: Option<String>,
    pub units: Option<String>,
}

impl LocalParameter {
    pub fn from(local_parameter: &sbml_rs::LocalParameter) -> Self {
        let id = local_parameter
            .id
            .as_ref()
            .expect("ID is mandatory for a Local Parameter.")
            .clone();
        let value = local_parameter
            .value
            .expect("Local parameter must have a value.");
        LocalParameter {
            id,
            value,
            units: local_parameter.units.clone(),
            sbo_term: local_parameter.sbo_term.clone(),
        }
    }
}
