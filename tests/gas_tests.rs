#[cfg(test)]
mod gas_tests {
    use rust_mechanical::constants::gas::*;
    use rust_mechanical::units::pressure::{self, Pressure};
    use rust_mechanical::units::temperature::{self, Temperature};

    #[test]
    fn air_density() {
        let d = AIR.standard_density;
        assert_eq!(d, 1.2754)
    }

    #[test]
    fn air_density_0c_100kpa() {
        let p = Pressure::new(100.0, pressure::Unit::Kpa);
        let t = Temperature::new(0.0, temperature::Unit::C);
        let d = AIR.density(t, p);
        assert_eq!(d, 1.2754)
    }

    #[test]
    fn air_density_20c_101kpa() {
        let p = Pressure::new(101.325, pressure::Unit::Kpa);
        let t = Temperature::new(20.0, temperature::Unit::C);
        let d = AIR.density(t, p);
        println!("{d}");
        let diff = f32::abs(d - 1.2041);
        assert!(diff < 0.0001)
    }
}
