use crate::errors::EngCalcError;

#[derive(Debug, Clone)]
pub enum Unit {
    Psi,
    Kpa,
    Pa,
    Bar,
    Atm,
}

impl Unit {
    pub fn to_string(&self) -> &'static str {
        match self {
            Unit::Pa => "Pa",
            Unit::Psi => "psi",
            Unit::Kpa => "kPa",
            Unit::Bar => "bar",
            Unit::Atm => "atm",
        }
    }
}

fn conversion_kpa(unit: &Unit) -> f32 {
    match unit {
        Unit::Psi => 0.14503774,
        Unit::Kpa => 1.0,
        Unit::Pa => 1000.0,
        Unit::Bar => 0.01,
        Unit::Atm => 1.009_869_2,
    }
}

fn convert(from_value: f32, from_unit: &Unit, to_unit: &Unit) -> f32 {
    let from_conversion = conversion_kpa(from_unit);
    let to_conversion = conversion_kpa(to_unit);
    from_value * to_conversion / from_conversion
}

#[derive(Debug, Clone)]
pub struct Pressure {
    value: f32,
    unit: Unit,
    absolute: bool,
}

impl Pressure {
    pub fn new(value: f32, unit: Unit, absolute: bool) -> Self {
        Pressure {
            value,
            unit,
            absolute,
        }
    }

    pub fn value(&self) -> f32 {
        self.value
    }

    pub fn unit(&self) -> Unit {
        self.unit.clone()
    }

    pub fn is_absolute(&self) -> bool {
        self.absolute
    }

    pub fn convert_unit(&mut self, new_unit: Unit) {
        let new_value = convert(self.value, &self.unit, &new_unit);
        self.value = new_value;
        self.unit = new_unit;
    }

    pub fn pressure_ratio(p1: &Pressure, p2: &Pressure) -> Result<f32, EngCalcError> {
        if !p1.is_absolute() || !p2.is_absolute() {
            return Err(EngCalcError::InvalidUnits(
                "Pressure ratio calculation must use absolute units.",
            ));
        }

        let p1_value_kpa = convert(p1.value, &p1.unit, &Unit::Kpa);
        let p2_value_kpa = convert(p2.value, &p2.unit, &Unit::Kpa);
        Ok(p2_value_kpa / p1_value_kpa)
    }

    pub fn add_pressure(self, other: &Pressure) -> Self {
        let p2_value_conv = convert(other.value, &other.unit, &self.unit());
        let total_pressure = self.value() + p2_value_conv;
        Pressure {
            value: total_pressure,
            unit: self.unit,
            absolute: self.absolute,
        }
    }

    pub fn subtract_pressure(self, other: &Pressure) -> Self {
        let p2_value_conv = convert(other.value, &other.unit, &self.unit());
        let total_pressure = self.value() - p2_value_conv;
        Pressure {
            value: total_pressure,
            unit: self.unit,
            absolute: self.absolute,
        }
    }
}
