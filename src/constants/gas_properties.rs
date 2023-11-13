use crate::units::pressure::{self, Pressure};
use crate::units::temperature::{self, Temperature};

pub mod air;

pub enum Gas {
    Air,
}

impl Gas {
    pub fn specific_heat_ratio(
        &self,
        temperature: &Temperature,
        pressure: &Pressure,
    ) -> Option<f32> {
        let mut temperature_k = temperature.clone();
        temperature_k.convert_unit(temperature::Unit::K);

        let mut pressure_atm = pressure.clone();
        pressure_atm.convert_unit(pressure::Unit::Atm);

        match self {
            Gas::Air => {
                air::specific_heat_ratio::interpolate(temperature_k.value(), pressure_atm.value())
            }
        }
    }
}
