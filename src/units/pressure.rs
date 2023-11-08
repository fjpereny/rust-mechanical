#[derive(Debug, Clone)]
pub enum Unit {
    Psi,
    Kpa,
    Pa,
    Bar,
}

fn conversion_kpa(unit: &Unit) -> f32 {
    match unit {
        Unit::Psi => 0.14503774,
        Unit::Kpa => 1.0,
        Unit::Pa => 1000.0,
        Unit::Bar => 0.01,
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
}

impl Pressure {
    pub fn new(value: f32, unit: Unit) -> Self {
        Pressure { value, unit }
    }

    pub fn value(&self) -> f32 {
        self.value
    }

    pub fn unit(&self) -> Unit {
        self.unit.clone()
    }

    pub fn convert_unit(&mut self, new_unit: Unit) {
        let new_value = convert(self.value, &self.unit, &new_unit);
        self.value = new_value;
        self.unit = new_unit;
    }

    pub fn ratio(p1: &Pressure, p2: &Pressure) -> f32 {
        let p1_value_kpa = convert(p1.value, &p1.unit, &Unit::Kpa);
        let p2_value_kpa = convert(p2.value, &p2.unit, &Unit::Kpa);
        p2_value_kpa / p1_value_kpa
    }

    pub fn add(self, p2: &Pressure) -> Self {
        let p1_value_kpa = convert(self.value, &self.unit, &Unit::Kpa);
        let p2_value_kpa = convert(p2.value, &p2.unit, &Unit::Kpa);
        let mut total_pressure = p1_value_kpa + p2_value_kpa;
        total_pressure = convert(total_pressure, &Unit::Kpa, &self.unit);
        Pressure {
            value: total_pressure,
            unit: self.unit,
        }
    }

    pub fn subtract(self, p2: &Pressure) -> Self {
        let p1_value_kpa = convert(self.value, &self.unit, &Unit::Kpa);
        let p2_value_kpa = convert(p2.value, &p2.unit, &Unit::Kpa);
        let mut total_pressure = p1_value_kpa - p2_value_kpa;
        total_pressure = convert(total_pressure, &Unit::Kpa, &self.unit);
        Pressure {
            value: total_pressure,
            unit: self.unit,
        }
    }
}
