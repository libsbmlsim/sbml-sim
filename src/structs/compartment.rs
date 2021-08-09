use sbml_rs;

#[derive(Debug, Clone)]
pub struct UnboundCompartment {
    pub id: String,
    pub constant: bool,
    pub sbo_term: Option<String>,
    pub units: Option<String>,
}

impl UnboundCompartment {
    pub fn from(compartment: &sbml_rs::Compartment) -> Self {
        let id = compartment
            .id
            .as_ref()
            .expect("ID is mandatory for compartments.")
            .to_owned();
        let constant = compartment
            .constant
            .expect("Constant attribute is mandatory for compartments.");
        let mut sbo_term = None;
        if let Some(value) = &compartment.sbo_term {
            sbo_term = Some(value.to_owned());
        }
        let mut units = None;
        if let Some(value) = &compartment.units {
            units = Some(value.to_owned());
        }
        UnboundCompartment {
            id,
            constant,
            sbo_term,
            units,
        }
    }

    pub fn to_bound(&self, size: f64) -> Compartment {
        Compartment {
            id: self.id.clone(),
            size,
            constant: self.constant,
            sbo_term: self.sbo_term.clone(),
            units: self.units.clone(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Compartment {
    pub id: String,
    pub size: f64,
    pub constant: bool,
    pub sbo_term: Option<String>,
    pub units: Option<String>,
}

impl Compartment {
    pub fn from(compartment: &sbml_rs::Compartment) -> Result<Self, UnboundCompartment> {
        let unbound_compartment = UnboundCompartment::from(compartment);
        if let Some(size) = compartment.size {
            Ok(unbound_compartment.to_bound(size))
        } else {
            Err(unbound_compartment)
        }
    }
}
