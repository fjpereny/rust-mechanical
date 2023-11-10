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
        pub specific_heat_cp: f32,
        pub specific_heat_cv: f32,
        pub standard_density: f32,
        pub molar_mass: f32,
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

        pub fn formula(&self) -> &'static str {
            self.chemical_formula
        }

        pub fn specific_heat_cp(&self) -> f32 {
            self.specific_heat_cp
        }

        pub fn specific_heat_cv(&self) -> f32 {
            self.specific_heat_cv
        }

        pub fn specific_heat_ratio(&self) -> f32 {
            self.specific_heat_cp / self.specific_heat_cv
        }

        pub fn standard_density(&self) -> String {
            self.standard_density.to_string()
        }

        pub fn molar_mass(&self) -> f32 {
            self.molar_mass
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
        specific_heat_cp: 1.005,
        specific_heat_cv: 0.7164,
        standard_density: 1.293, // kg/m^3 @ 0C / 101.325 kPa (1 atm)
        molar_mass: 28.9647,
    };

    pub const ARGON: Gas = Gas {
        name: "Argon",
        chemical_formula: "Ar",
        specific_heat_cp: 0.520,
        specific_heat_cv: 0.312,
        standard_density: 1.7837, // kg/m^3 @ 0C / 101.325 kPa (1 atm)
        molar_mass: 39.948,
    };

    pub const NITROGEN: Gas = Gas {
        name: "Nitrogen",
        chemical_formula: "N2",
        specific_heat_cp: 1.04,
        specific_heat_cv: 0.743,
        standard_density: 1.2506, // kg/m^3 @ 0C / 101.325 kPa (1 atm)
        molar_mass: 28.013,
    };
}

pub mod fluid {
    pub mod specific_heat {}
}
