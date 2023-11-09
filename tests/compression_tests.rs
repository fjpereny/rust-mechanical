#[cfg(test)]
mod compression_tests {
    use rust_mechanical::compression::isentropic_eff;
    use rust_mechanical::constants::gas;
    use rust_mechanical::states::GasState;
    use rust_mechanical::units::pressure::{self, Pressure};
    use rust_mechanical::units::temperature::{self, Temperature};

    #[test]
    fn isentropic_eff_1() {
        let p1 = Pressure::new(16.0, pressure::Unit::Psi, true);
        let p2 = Pressure::new(32.0, pressure::Unit::Psi, true);
        let t1 = Temperature::new(80.0, temperature::Unit::F);
        let t2 = Temperature::new(198.2, temperature::Unit::F);

        let state_1 = GasState::new(p1, t1, gas::AIR);
        let state_2 = GasState::new(p2, t2, gas::AIR);

        let eff = isentropic_eff(&state_1, &state_2).unwrap();
        let expected = 1.0;
        let diff = f32::abs(expected - eff);
        assert!(diff < 0.001);
        assert!(state_1.pressure().is_absolute());
        assert!(state_2.pressure().is_absolute());
    }

    #[test]
    fn isentropic_eff_2() {
        let p1 = Pressure::new(16.0, pressure::Unit::Bar, true);
        let p2 = Pressure::new(32.0, pressure::Unit::Bar, true);
        let t1 = Temperature::new(80.0, temperature::Unit::C);
        let t2 = Temperature::new(157.3, temperature::Unit::C);

        let state_1 = GasState::new(p1, t1, gas::AIR);
        let state_2 = GasState::new(p2, t2, gas::AIR);

        let eff = isentropic_eff(&state_1, &state_2).unwrap();
        let expected = 1.0;
        let diff = f32::abs(expected - eff);
        assert!(diff < 0.001);
        assert!(state_1.pressure().is_absolute());
        assert!(state_2.pressure().is_absolute());
    }

    #[test]
    fn isentropic_eff_3() {
        let p1 = Pressure::new(100.0, pressure::Unit::Kpa, true);
        let p2 = Pressure::new(350.0, pressure::Unit::Kpa, true);
        let t1 = Temperature::new(80.0, temperature::Unit::R);
        let t2 = Temperature::new(114.4, temperature::Unit::R);

        let state_1 = GasState::new(p1, t1, gas::AIR);
        let state_2 = GasState::new(p2, t2, gas::AIR);

        let eff = isentropic_eff(&state_1, &state_2).unwrap();
        let expected = 1.0;
        let diff = f32::abs(expected - eff);
        assert!(diff < 0.001);
        assert!(state_1.pressure().is_absolute());
        assert!(state_2.pressure().is_absolute());
    }
}
