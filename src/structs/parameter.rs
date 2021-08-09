use sbml_rs;

#[derive(Debug, Clone)]
pub struct UnboundParameter {
    pub id: String,
    pub constant: bool,
    pub sbo_term: Option<String>,
    pub units: Option<String>,
}

impl UnboundParameter {
    pub fn from(parameter: &sbml_rs::Parameter) -> Self {
        let id = parameter
            .id
            .as_ref()
            .expect("ID is mandatory for parameters.")
            .to_owned();
        let constant = parameter
            .constant
            .expect("Constant attribute is mandatory for parameters.");
        let mut sbo_term = None;
        if let Some(value) = &parameter.sbo_term {
            sbo_term = Some(value.to_owned());
        }
        let mut units = None;
        if let Some(value) = &parameter.units {
            units = Some(value.to_owned());
        }
        UnboundParameter {
            id,
            constant,
            sbo_term,
            units,
        }
    }

    pub fn to_bound(&self, value: f64) -> Parameter {
        Parameter {
            id: self.id.clone(),
            value,
            constant: self.constant,
            sbo_term: self.sbo_term.clone(),
            units: self.units.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Parameter {
    pub id: String,
    pub value: f64,
    pub constant: bool,
    pub sbo_term: Option<String>,
    pub units: Option<String>,
}

impl Parameter {
    pub fn from(parameter: &sbml_rs::Parameter) -> Result<Self, UnboundParameter> {
        let unbound_parameter = UnboundParameter::from(parameter);
        if let Some(value) = parameter.value {
            Ok(unbound_parameter.to_bound(value))
        } else {
            Err(unbound_parameter)
        }
    }
}
