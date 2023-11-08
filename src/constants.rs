pub mod gas {
    use crate::units::pressure::{self, Pressure};
    use crate::units::temperature::{self, Temperature};

    // Ideal Gas Constant
    const IDEAL_GAS_CONST: f32 = 8.31446261815324;

    pub struct Gas {
        pub name: &'static str,
        pub specific_heat_ratio: f32,
        pub standard_density: f32,
    }

    impl Gas {
        pub fn density(&self, temperature: Temperature, pressure: Pressure) -> f32 {
            let ref_pressure = 100.0; // kPa
            let ref_temperature = 273.15; // K

            let mut temperature = temperature.clone();
            temperature.convert_unit(temperature::Unit::K);
            let mut pressure = pressure.clone();
            pressure.convert_unit(pressure::Unit::Kpa);
            self.standard_density * pressure.value() / ref_pressure * ref_temperature
                / temperature.value()
        }
    }

    pub const AIR: Gas = Gas {
        name: "Air",
        specific_heat_ratio: 1.40,
        standard_density: 1.2754, // kg/m^3 @ 0C / 100kPa
    };
}

pub mod fluid {
    pub mod specific_heat {}
}
