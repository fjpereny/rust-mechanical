pub mod gas {
    use crate::units::pressure::{self, Pressure};
    use crate::units::temperature::{self, Temperature};
    use ratatui::text::Text;

    // Ideal Gas Constant
    const IDEAL_GAS_CONST: f32 = 8.314_463;

    pub fn gas_list() -> Vec<Gas> {
        vec![AIR, ARGON, NITROGEN]
    }

    pub struct Gas {
        pub name: &'static str,
        pub chemical_formula: &'static str,
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

        pub fn name(&self) -> &'static str {
            self.name
        }
    }

    impl<'a> Into<Text<'a>> for Gas {
        fn into(self) -> Text<'a> {
            let name = ratatui::text::Line::from(self.name);
            let lines = vec![name];
            Text { lines }
        }
    }

    pub const AIR: Gas = Gas {
        name: "Air",
        chemical_formula: "",
        specific_heat_ratio: 1.40,
        standard_density: 1.293, // kg/m^3 @ 0C / 101.325 kPa (1 atm)
    };

    pub const ARGON: Gas = Gas {
        name: "Argon",
        chemical_formula: "Ar",
        specific_heat_ratio: 1.667,
        standard_density: 1.7837, // kg/m^3 @ 0C / 101.325 kPa (1 atm)
    };

    pub const NITROGEN: Gas = Gas {
        name: "Nitrogen",
        chemical_formula: "N2",
        specific_heat_ratio: 1.40,
        standard_density: 1.2506, // kg/m^3 @ 0C / 101.325 kPa (1 atm)
    };
}

pub mod fluid {
    pub mod specific_heat {}
}
