use sbml_rs;

#[derive(Clone, Debug)]
pub struct UnboundSpecies {
    pub id: String,
    pub compartment: String,
    pub amount: Option<f64>,
    pub concentration: Option<f64>,
    pub sbo_term: Option<String>,
    pub substance_units: Option<String>,
    pub has_only_substance_units: bool,
    pub boundary_condition: bool,
    pub constant: bool,
    pub conversion_factor: Option<String>,
}

impl UnboundSpecies {
    pub fn from(species: &sbml_rs::Species) -> Self {
        let id = species
            .id
            .as_ref()
            .expect("ID is mandatory for Species.")
            .to_owned();
        let compartment = species
            .compartment
            .as_ref()
            .expect("Compartment attribute is mandatory for species.")
            .to_owned();
        let has_only_substance_units = species
            .has_only_substance_units
            .expect("has_only_substance_units attribute is mandatory for species.");
        let boundary_condition = species
            .boundary_condition
            .expect("Boundary condition attribute is mandatory for species.");
        let constant = species
            .constant
            .expect("Constant attribute is mandatory for species.");
        let mut amount = None;
        if let Some(value) = &species.initial_amount {
            amount = Some(value.to_owned());
        }
        let mut concentration = None;
        if let Some(value) = &species.initial_concentration {
            concentration = Some(value.to_owned());
        }
        let mut sbo_term = None;
        if let Some(value) = &species.sbo_term {
            sbo_term = Some(value.to_owned());
        }
        let mut conversion_factor = None;
        if let Some(value) = &species.conversion_factor {
            conversion_factor = Some(value.to_owned());
        }
        let mut substance_units = None;
        if let Some(value) = &species.substance_units {
            substance_units = Some(value.to_owned());
        }
        UnboundSpecies {
            id,
            compartment,
            amount,
            concentration,
            has_only_substance_units,
            boundary_condition,
            constant,
            sbo_term,
            substance_units,
            conversion_factor,
        }
    }

    pub fn to_bound(&self, amount: f64, concentration: f64) -> Species {
        Species {
            id: self.id.clone(),
            compartment: self.compartment.clone(),
            concentration,
            amount,
            sbo_term: self.sbo_term.clone(),
            substance_units: self.substance_units.clone(),
            has_only_substance_units: self.has_only_substance_units,
            boundary_condition: self.boundary_condition,
            constant: self.constant,
            conversion_factor: self.conversion_factor.clone(),
        }
    }

    pub fn to_bound_with_size(&self, size: f64) -> Result<Species, ()> {
        if self.concentration.is_some() && self.amount.is_none() {
            let concentration = self.concentration.unwrap();
            let amount = concentration * size;
            Ok(self.to_bound(amount, concentration))
        } else if self.amount.is_some() && self.concentration.is_none() {
            let amount = self.amount.unwrap();
            let concentration = amount / size;
            Ok(self.to_bound(amount, concentration))
        } else {
            Err(())
        }
    }
}

#[derive(Clone, Debug)]
pub struct Species {
    pub id: String,
    pub compartment: String,
    concentration: f64,
    amount: f64,
    pub sbo_term: Option<String>,
    pub substance_units: Option<String>,
    pub has_only_substance_units: bool,
    pub boundary_condition: bool,
    pub constant: bool,
    pub conversion_factor: Option<String>,
}

impl Species {
    pub fn from_concentration(
        species: &sbml_rs::Species,
        concentration: f64,
        compartment_size: f64,
    ) -> Self {
        let unbound_species = UnboundSpecies::from(species);
        let amount = concentration * compartment_size;
        unbound_species.to_bound(amount, concentration)
    }

    pub fn from_amount(species: &sbml_rs::Species, amount: f64, compartment_size: f64) -> Self {
        let unbound_species = UnboundSpecies::from(species);
        let concentration = amount / compartment_size;
        unbound_species.to_bound(amount, concentration)
    }
    pub fn concentration(&self) -> f64 {
        self.concentration
    }

    pub fn amount(&self) -> f64 {
        self.amount
    }

    pub fn update_compartment_size(&mut self, size: f64) {
        self.concentration = self.amount / size;
    }

    pub fn update_concentration(&mut self, concentration: f64, size: f64) {
        self.concentration = concentration;
        self.amount = concentration * size;
    }

    pub fn update_amount(&mut self, amount: f64, size: f64) {
        self.amount = amount;
        self.concentration = amount / size;
    }

    pub fn update_amount_by(&mut self, delta: f64, size: f64) {
        self.amount += delta;
        self.concentration = self.amount / size;
    }
}
