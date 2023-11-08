use std::fmt::Display;

use crate::constants::gas::Gas;
use crate::units::pressure::Pressure;
use crate::units::temperature::Temperature;

pub struct GasState {
    pressure: Pressure,
    temperature: Temperature,
    gas: Gas,
}

impl GasState {
    pub fn pressure(&self) -> &Pressure {
        &self.pressure
    }

    pub fn temperature(&self) -> &Temperature {
        &self.temperature
    }

    pub fn gas(&self) -> &Gas {
        &self.gas
    }

    pub fn new(pressure: Pressure, temperature: Temperature, gas: Gas) -> Self {
        GasState {
            pressure,
            temperature,
            gas,
        }
    }
}
