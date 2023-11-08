mod compression;
mod constants;
mod states;
mod units;

#[cfg(test)]
mod pressure_tests {
    use crate::units::pressure::*;

    #[test]
    fn create_pressure() {
        let value = 150.0;
        let p = Pressure::new(value, Unit::Psi);
        assert_eq!(p.value(), value);
    }

    #[test]
    fn convert_pressure_psi() {
        let mut p = Pressure::new(100.0, Unit::Psi);

        p.convert_unit(Unit::Psi);
        let p_expected = 100.0;
        let diff = p_expected - p.value();
        assert!(diff < 0.001);
    }

    #[test]
    fn convert_pressure_kpa() {
        let mut p = Pressure::new(100.0, Unit::Psi);

        p.convert_unit(Unit::Kpa);
        let p_expected = 689.475729;
        let diff = p_expected - p.value();
        assert!(diff < 0.001);
    }

    #[test]
    fn convert_pressure_pa() {
        let mut p = Pressure::new(100.0, Unit::Psi);

        p.convert_unit(Unit::Pa);
        let p_expected = 689475.729;
        let diff = p_expected - p.value();
        assert!(diff < 0.1);
    }

    #[test]
    fn convert_pressure_bar() {
        let mut p = Pressure::new(100.0, Unit::Psi);

        p.convert_unit(Unit::Bar);
        let p_expected = 6.894757;
        let diff = p_expected - p.value();
        assert!(diff < 0.001);
    }

    #[test]
    fn add_pressure_psi() {
        let p1 = Pressure::new(100.0, Unit::Psi);
        let p2 = Pressure::new(100.0, Unit::Psi);
        let p1 = p1.add(&p2);
        println!("{}", p1.value());
        assert_eq!(200.0, p1.value());
    }

    #[test]
    fn add_pressure_kpa() {
        let p1 = Pressure::new(100.0, Unit::Psi);
        let p2 = Pressure::new(100.0, Unit::Kpa);
        let p1 = p1.add(&p2);
        let diff = p1.value() - 114.50377;
        assert!(diff < 0.0001);
    }

    #[test]
    fn add_pressure_pa() {
        let p1 = Pressure::new(100.0, Unit::Psi);
        let p2 = Pressure::new(10000.0, Unit::Pa);
        let p1 = p1.add(&p2);
        let diff = p1.value() - 101.450377;
        assert!(diff < 0.0001);
    }

    #[test]
    fn add_pressure_bar() {
        let p1 = Pressure::new(100.0, Unit::Psi);
        let p2 = Pressure::new(10.0, Unit::Bar);
        let p1 = p1.add(&p2);
        let diff = p1.value() - 245.037738;
        assert!(diff < 0.0001);
    }

    #[test]
    fn sub_pressure_psi() {
        let p1 = Pressure::new(100.0, Unit::Psi);
        let p2 = Pressure::new(100.0, Unit::Psi);
        let p1 = p1.subtract(&p2);
        println!("{}", p1.value());
        assert_eq!(0.0, p1.value());
    }

    #[test]
    fn sub_pressure_kpa() {
        let p1 = Pressure::new(100.0, Unit::Psi);
        let p2 = Pressure::new(100.0, Unit::Kpa);
        let p1 = p1.subtract(&p2);
        let diff = p1.value() - 85.49622;
        assert!(diff < 0.0001);
    }

    #[test]
    fn sub_pressure_pa() {
        let p1 = Pressure::new(100.0, Unit::Psi);
        let p2 = Pressure::new(10000.0, Unit::Pa);
        let p1 = p1.subtract(&p2);
        let diff = p1.value() - 98.549623;
        assert!(diff < 0.0001);
    }

    #[test]
    fn sub_pressure_bar() {
        let p1 = Pressure::new(100.0, Unit::Psi);
        let p2 = Pressure::new(0.5, Unit::Bar);
        let p1 = p1.subtract(&p2);
        let diff = p1.value() - 92.74811311;
        assert!(diff < 0.0001);
    }

    #[test]
    fn ratio_pressure_psi() {
        let p1 = Pressure::new(100.0, Unit::Psi);
        let p2 = Pressure::new(100.0, Unit::Psi);
        let pr = Pressure::ratio(&p1, &p2);
        assert_eq!(1.0, pr);
    }

    #[test]
    fn ratio_pressure_kpa() {
        let p1 = Pressure::new(100.0, Unit::Psi);
        let p2 = Pressure::new(100.0, Unit::Kpa);
        let pr = Pressure::ratio(&p1, &p2);
        let diff = pr - 0.854962262;
        assert!(diff < 0.0001);
    }

    #[test]
    fn ratio_pressure_pa() {
        let p1 = Pressure::new(100.0, Unit::Psi);
        let p2 = Pressure::new(100_000.0, Unit::Pa);
        let pr = Pressure::ratio(&p1, &p2);
        let diff = pr - 0.854962262;
        assert!(diff < 0.0001);
    }

    #[test]
    fn ratio_pressure_bar() {
        let p1 = Pressure::new(100.0, Unit::Psi);
        let p2 = Pressure::new(1.5, Unit::Bar);
        let pr = Pressure::ratio(&p1, &p2);
        let diff = pr - 0.782443393;
        assert!(diff < 0.0001);
    }
}

#[cfg(test)]
mod temperature_tests {
    use crate::units::temperature::*;

    #[test]
    fn create_temp() {
        let t = Temperature::new(100.0, Unit::C);
        assert_eq!(100.0, t.value());
    }

    #[test]
    fn convert_temp_r() {
        let mut t = Temperature::new(100.0, Unit::C);
        t.convert_unit(Unit::R);
        assert_eq!(671.67, t.value());
    }

    #[test]
    fn convert_temp_k() {
        let mut t = Temperature::new(100.0, Unit::C);
        t.convert_unit(Unit::K);
        assert_eq!(373.15, t.value());
    }

    #[test]
    fn convert_temp_f() {
        let mut t = Temperature::new(100.0, Unit::C);
        t.convert_unit(Unit::F);
        println!("{}", t.value());
        assert_eq!(212.0, t.value());
    }

    #[test]
    fn convert_temp_c() {
        let mut t = Temperature::new(100.0, Unit::C);
        t.convert_unit(Unit::C);
        println!("{}", t.value());
        assert_eq!(100.0, t.value());
    }
}

#[cfg(test)]
mod compression_tests {
    use crate::compression::isentropic_eff;
    use crate::constants::gas;
    use crate::states::GasState;
    use crate::units::pressure::{self, Pressure};
    use crate::units::temperature::{self, Temperature};

    #[test]
    fn isentropic_eff_1() {
        let p1 = Pressure::new(16.0, pressure::Unit::Psi);
        let p2 = Pressure::new(32.0, pressure::Unit::Psi);
        let t1 = Temperature::new(80.0, temperature::Unit::F);
        let t2 = Temperature::new(198.2, temperature::Unit::F);

        let state_1 = GasState::new(p1, t1, gas::AIR);
        let state_2 = GasState::new(p2, t2, gas::AIR);

        let eff = isentropic_eff(&state_1, &state_2);
        let expected = 1.0;
        let diff = f32::abs(expected - eff);
        assert!(diff < 0.001);
    }

    #[test]
    fn isentropic_eff_2() {
        let p1 = Pressure::new(16.0, pressure::Unit::Bar);
        let p2 = Pressure::new(32.0, pressure::Unit::Bar);
        let t1 = Temperature::new(80.0, temperature::Unit::C);
        let t2 = Temperature::new(157.3, temperature::Unit::C);

        let state_1 = GasState::new(p1, t1, gas::AIR);
        let state_2 = GasState::new(p2, t2, gas::AIR);

        let eff = isentropic_eff(&state_1, &state_2);
        let expected = 1.0;
        let diff = f32::abs(expected - eff);
        assert!(diff < 0.001);
    }

    #[test]
    fn isentropic_eff_3() {
        let p1 = Pressure::new(100.0, pressure::Unit::Kpa);
        let p2 = Pressure::new(350.0, pressure::Unit::Kpa);
        let t1 = Temperature::new(80.0, temperature::Unit::R);
        let t2 = Temperature::new(114.4, temperature::Unit::R);

        let state_1 = GasState::new(p1, t1, gas::AIR);
        let state_2 = GasState::new(p2, t2, gas::AIR);

        let eff = isentropic_eff(&state_1, &state_2);
        let expected = 1.0;
        let diff = f32::abs(expected - eff);
        assert!(diff < 0.001);
    }
}
