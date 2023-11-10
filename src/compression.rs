use crate::errors::EngCalcError;
use crate::states::GasState;
use crate::units::pressure::Pressure;
use crate::units::temperature;

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
