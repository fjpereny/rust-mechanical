#[cfg(test)]
mod pressure_tests {
    use rust_mechanical::units::pressure::*;

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