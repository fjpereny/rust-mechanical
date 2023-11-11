//! A collection of functions used for performing
//! engineering calculations related to gas compression.

use crate::errors::EngCalcError;
use crate::states::GasState;
use crate::units::pressure::Pressure;
use crate::units::temperature;

/// Calculates isentropic efficiency of compression.
///
/// # Examples
/// ```
/// let p1 = Pressure::new(16.0, pressure::Unit::Psi, true);
/// let p2 = Pressure::new(32.0, pressure::Unit::Psi, true);
/// let t1 = Temperature::new(80.0, temperature::Unit::F);
/// let t2 = Temperature::new(198.2, temperature::Unit::F);
///
/// let state_1 = GasState::new(p1, t1, gas::AIR);
/// let state_2 = GasState::new(p2, t2, gas::AIR);
///
/// let eff = isentropic_eff(&state_1, &state_2).unwrap();
/// let expected = 1.0;
/// let diff = f32::abs(expected - eff);
/// assert!(diff < 0.01);
/// assert!(state_1.pressure().is_absolute());
/// assert!(state_2.pressure().is_absolute());
/// ```
///
/// # Errors
/// Returns an EngCalcErr if the gas states are incompatable (for example comparing absolute and
/// relative pressures).
pub fn isentropic_eff(state_1: &GasState, state_2: &GasState) -> Result<f32, EngCalcError> {
    let mut t1 = state_1.temperature().clone();
    t1.convert_unit(temperature::Unit::K);
    let t1 = t1.value();

    let mut t2 = state_2.temperature().clone();
    t2.convert_unit(temperature::Unit::K);
    let t2 = t2.value();

    let k = state_1.gas().specific_heat_ratio();
    let pr = Pressure::pressure_ratio(state_1.pressure(), state_2.pressure());

    match pr {
        Ok(pr) => Ok(t1 * (pr.powf((k - 1.0) / k) - 1.0) / (t2 - t1)),
        Err(e) => Err(e),
    }
}
