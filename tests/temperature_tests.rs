#[cfg(test)]
mod conversion_tests {
    use rust_mechanical::units::temperature::*;

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
        assert_eq!(212.0, t.value());
    }

    #[test]
    fn convert_temp_c() {
        let mut t = Temperature::new(100.0, Unit::C);
        t.convert_unit(Unit::C);
        assert_eq!(100.0, t.value());
    }
}

#[cfg(test)]
mod temperature_addition {
    use rust_mechanical::units::temperature::*;

    #[test]
    fn add_c_c() {
        let t1 = Temperature::new(100.0, Unit::C);
        let t2 = Temperature::new(100.0, Unit::C);
        let t3 = t1.add_temperature(&t2);
        assert_eq!(200.0, t3.value());
    }
}
