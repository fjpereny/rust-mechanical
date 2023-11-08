#[derive(Debug, Clone)]
pub enum Unit {
    R,
    K,
    C,
    F,
}

fn convert(from_value: f32, from_unit: &Unit, to_unit: &Unit) -> f32 {
    match from_unit {
        Unit::R => match to_unit {
            Unit::R => from_value,
            Unit::K => from_value * 5.0 / 9.0,
            Unit::F => from_value - 459.67,
            Unit::C => from_value * 5.0 / 9.0 - 273.15,
        },
        Unit::K => match to_unit {
            Unit::R => from_value * 9.0 / 5.0,
            Unit::K => from_value,
            Unit::F => from_value * 9.0 / 5.0 - 459.67,
            Unit::C => from_value - 273.15,
        },
        Unit::F => match to_unit {
            Unit::R => from_value + 459.67,
            Unit::K => (from_value + 459.67) * 5.0 / 9.0,
            Unit::F => from_value,
            Unit::C => (from_value - 32.0) * 5.0 / 9.0,
        },
        Unit::C => match to_unit {
            Unit::R => (from_value + 273.15) * 9.0 / 5.0,
            Unit::K => from_value + 273.15,
            Unit::F => from_value * 9.0 / 5.0 + 32.0,
            Unit::C => from_value,
        },
    }
}

#[derive(Debug, Clone)]
pub struct Temperature {
    value: f32,
    unit: Unit,
}

impl Temperature {
    pub fn new(value: f32, unit: Unit) -> Self {
        Temperature { value, unit }
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

    pub fn ratio(p1: &Temperature, p2: &Temperature) -> f32 {
        let t1_value_k = convert(p1.value, &p1.unit, &Unit::K);
        let t2_value_k = convert(p2.value, &p2.unit, &Unit::K);
        t2_value_k / t1_value_k
    }

    pub fn add(self, p2: &Temperature) -> Self {
        let t1_value_k = convert(self.value, &self.unit, &Unit::K);
        let t2_value_k = convert(p2.value, &p2.unit, &Unit::K);
        let mut total_temp = t1_value_k + t2_value_k;
        total_temp = convert(total_temp, &Unit::K, &self.unit);
        Temperature {
            value: total_temp,
            unit: self.unit,
        }
    }

    pub fn subtract(self, p2: &Temperature) -> Self {
        let t1_value_k = convert(self.value, &self.unit, &Unit::K);
        let t2_value_k = convert(p2.value, &p2.unit, &Unit::K);
        let mut total_temp = t1_value_k - t2_value_k;
        total_temp = convert(total_temp, &Unit::K, &self.unit);
        Temperature {
            value: total_temp,
            unit: self.unit,
        }
    }
}
