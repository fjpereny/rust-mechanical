#[cfg(test)]
mod gas_tests {
    use mechanical_engineering::constants::gas::*;
    use mechanical_engineering::units::pressure::{self, Pressure};
    use mechanical_engineering::units::temperature::{self, Temperature};

    #[test]
    fn air_density() {
        let d = AIR.standard_density;
        assert_eq!(d, 1.293)
    }

    #[test]
    fn air_density_0c_1atm() {
        let p = Pressure::new(101.325, pressure::Unit::Kpa, true);
        let t = Temperature::new(0.0, temperature::Unit::C);
        let d = AIR.density(t, p);
        let expected = 1.293;
        let diff = f32::abs(d / expected - 1.0);
        println!("{diff}");
        assert!(diff <= 0.02);
    }

    #[test]
    fn air_density_20c_1atm() {
        let p = Pressure::new(101.325, pressure::Unit::Kpa, true);
        let t = Temperature::new(20.0, temperature::Unit::C);
        let d = AIR.density(t, p);
        let expected = 1.205;
        let diff = f32::abs(d / expected - 1.0);
        println!("{diff}");
        assert!(diff <= 0.02);
    }
}
