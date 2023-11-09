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
mod temperature_add_sub {
    use rust_mechanical::units::temperature::*;

    #[test]
    fn add_c_c() {
        let t1 = Temperature::new(100.0, Unit::C);
        let t2 = Temperature::new(100.0, Unit::C);
        let t3 = t1.add_temperature(&t2);
        assert_eq!(200.0, t3.value());
    }

    #[test]
    fn sub_c_c() {
        let t1 = Temperature::new(100.0, Unit::C);
        let t2 = Temperature::new(100.0, Unit::C);
        let t3 = t1.subtract_temperature(&t2);
        assert_eq!(0.0, t3.value());
    }

    #[test]
    fn add_f_k() {
        let t1 = Temperature::new(100.0, Unit::F);
        let t2 = Temperature::new(100.0, Unit::R);
        let t3 = t1.add_temperature(&t2);
        assert_eq!(200.0, t3.value());
    }

    #[test]
    fn sub_c_r() {
        let t1 = Temperature::new(100.0, Unit::C);
        let t2 = Temperature::new(100.0, Unit::K);
        let t3 = t1.subtract_temperature(&t2);
        assert_eq!(0.0, t3.value());
    }

    #[test]
    fn add_f_c() {
        let t1 = Temperature::new(100.0, Unit::F);
        let t2 = Temperature::new(100.0, Unit::C);
        let t3 = t1.add_temperature(&t2);
        assert_eq!(280.0, t3.value());
    }

    #[test]
    fn add_c_f() {
        let t1 = Temperature::new(100.0, Unit::C);
        let t2 = Temperature::new(100.0, Unit::F);
        let t3 = t1.add_temperature(&t2);
        assert_eq!(155.5555555, t3.value());
    }

    #[test]
    fn sub_f_c() {
        let t1 = Temperature::new(100.0, Unit::F);
        let t2 = Temperature::new(100.0, Unit::C);
        let t3 = t1.subtract_temperature(&t2);
        assert_eq!(-80.0, t3.value());
    }

    #[test]
    fn sub_c_f() {
        let t1 = Temperature::new(0.0, Unit::C);
        let t2 = Temperature::new(100.0, Unit::F);
        let t3 = t1.subtract_temperature(&t2);
        println!("{}", t3.value());
        assert_eq!(-55.5555555, t3.value());
    }
}
