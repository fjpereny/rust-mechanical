pub mod specific_heat_ratio {
    pub enum TemperatureIndex {
        T200K,
        // T210K,
        // T220K,
        // T230K,
        // T240K,
        T250K,
        // T260K,
        // T270K,
        // T280K,
        // T290K,
        // T300K,
        // T310K,
        // T320K,
        // T330K,
        // T340K,
        // T350K,
        // T360K,
        // T380K,
        // T390K,
        // T400K,
        // T410K,
        // T420K,
        // T430K,
        // T440K,
        // T450K,
        // T460K,
        // T470K,
        // T480K,
        // T490K,
        // T500K,
    }

    pub enum PressureIndex {
        P0x01ATM,
        P0x10ATM,
        P0x40ATM,
        P0x70ATM,
        P1x00ATM,
        P4x00ATM,
        P7x00ATM,
        P10x00ATM,
        P40x00ATM,
        P70x00ATM,
        P100x00ATM,
    }

    pub fn table(temp: TemperatureIndex, press: PressureIndex) -> f32 {
        match temp {
            TemperatureIndex::T200K => match press {
                PressureIndex::P0x01ATM => 1.4013,
                PressureIndex::P0x10ATM => 1.4017,
                PressureIndex::P0x40ATM => 1.4030,
                PressureIndex::P0x70ATM => 1.4043,
                PressureIndex::P1x00ATM => 1.4057,
                PressureIndex::P4x00ATM => 1.4197,
                PressureIndex::P7x00ATM => 1.4344,
                PressureIndex::P10x00ATM => 1.4489,
                PressureIndex::P40x00ATM => 1.6418,
                PressureIndex::P70x00ATM => 1.9000,
                PressureIndex::P100x00ATM => 2.1376,
            },
            TemperatureIndex::T250K => match press {
                PressureIndex::P0x01ATM => 1.4009,
                PressureIndex::P0x10ATM => 1.4011,
                PressureIndex::P0x40ATM => 1.4020,
                PressureIndex::P0x70ATM => 1.4028,
                PressureIndex::P1x00ATM => 1.4036,
                PressureIndex::P4x00ATM => 1.4118,
                PressureIndex::P7x00ATM => 1.4201,
                PressureIndex::P10x00ATM => 1.4284,
                PressureIndex::P40x00ATM => 1.5185,
                PressureIndex::P70x00ATM => 1.6130,
                PressureIndex::P100x00ATM => 1.6990,
            },
        }
    }

    pub fn interpolate(temperature_k: f32, pressure_atm: f32) -> Option<f32> {
        if temperature_k < 200.0 || pressure_atm > 100.0 {
            return None;
        }

        let mut temp_index_a = TemperatureIndex::T200K;
        let mut temp_index_b = TemperatureIndex::T200K;
        let mut temp_delta_a = 0.0;
        let mut temp_delta_b = 0.0;
        if temperature_k <= 250.0 {
            temp_index_a = TemperatureIndex::T200K;
            temp_index_b = TemperatureIndex::T250K;
            temp_delta_a = f32::abs((temperature_k - 200.0) / 50.0);
            temp_delta_b = f32::abs((250.0 - temperature_k) / 50.0);
        }

        let mut press_index_a = PressureIndex::P0x01ATM;
        let mut press_index_b = PressureIndex::P0x01ATM;
        let mut press_delta_a = 0.0;
        let mut press_delta_b = 0.0;
        if pressure_atm < 0.1 {
            press_index_a = PressureIndex::P0x01ATM;
            press_index_b = PressureIndex::P0x10ATM;
            press_delta_a = f32::abs((pressure_atm - 0.01) / 0.09);
            press_delta_b = f32::abs((0.1 - pressure_atm) / 0.09);
        }

        let value_a = table(temp_index_a, press_index_a);
        let value_b = table(temp_index_b, press_index_b);

        let mut calculated = value_a * (1.0 - temp_delta_a) + value_b * (1.0 - temp_delta_b);
        calculated += value_a * (1.0 - press_delta_a) + value_b * (1.0 - press_delta_b);
        Some(calculated / 2.0)
    }
}
