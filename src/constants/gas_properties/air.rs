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
        T300K,
        // T310K,
        // T320K,
        // T330K,
        // T340K,
        T350K,
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

    pub fn table(temp: &TemperatureIndex, press: &PressureIndex) -> f32 {
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
            TemperatureIndex::T300K => match press {
                PressureIndex::P0x01ATM => 1.4000,
                PressureIndex::P0x10ATM => 1.4001,
                PressureIndex::P0x40ATM => 1.4006,
                PressureIndex::P0x70ATM => 1.4012,
                PressureIndex::P1x00ATM => 1.4017,
                PressureIndex::P4x00ATM => 1.4070,
                PressureIndex::P7x00ATM => 1.4123,
                PressureIndex::P10x00ATM => 1.4177,
                PressureIndex::P40x00ATM => 1.4717,
                PressureIndex::P70x00ATM => 1.5240,
                PressureIndex::P100x00ATM => 1.5711,
            },
            TemperatureIndex::T350K => match press {
                PressureIndex::P0x01ATM => 1.3981,
                PressureIndex::P0x10ATM => 1.3982,
                PressureIndex::P0x40ATM => 1.3985,
                PressureIndex::P0x70ATM => 1.3989,
                PressureIndex::P1x00ATM => 1.3993,
                PressureIndex::P4x00ATM => 1.4030,
                PressureIndex::P7x00ATM => 1.4067,
                PressureIndex::P10x00ATM => 1.4104,
                PressureIndex::P40x00ATM => 1.4465,
                PressureIndex::P70x00ATM => 1.4804,
                PressureIndex::P100x00ATM => 1.5109,
            },
        }
    }

    pub fn interpolate(temperature_k: f32, pressure_atm: f32) -> Option<f32> {
        if temperature_k < 200.0 || temperature_k > 2000.0 {
            return None;
        }
        if pressure_atm < 0.01 || pressure_atm > 100.0 {
            return None;
        }

        let mut temp_index_a = TemperatureIndex::T200K;
        let mut temp_index_b = TemperatureIndex::T200K;
        let mut temp_delta_a = 0.0;
        let mut temp_delta_b = 0.0;

        let next_temp_interval = 250.0;
        let t_low = 200.0;
        let t_high = next_temp_interval;
        let t_range = t_high - t_low;
        if temperature_k > t_low && temperature_k <= t_high {
            temp_index_a = TemperatureIndex::T200K;
            temp_index_b = TemperatureIndex::T250K;
            temp_delta_a = f32::abs((temperature_k - t_low) / t_range);
            temp_delta_b = f32::abs((t_high - temperature_k) / t_range);
        }
        let next_temp_interval = 300.0;
        let t_low = t_high;
        let t_high = next_temp_interval;
        let t_range = t_high - t_low;
        if temperature_k > t_low && temperature_k <= t_high {
            temp_index_a = TemperatureIndex::T250K;
            temp_index_b = TemperatureIndex::T300K;
            temp_delta_a = f32::abs((temperature_k - t_low) / t_range);
            temp_delta_b = f32::abs((t_high - temperature_k) / t_range);
        }
        let next_temp_interval = 350.0;
        let t_low = t_high;
        let t_high = next_temp_interval;
        let t_range = t_high - t_low;
        if temperature_k > t_low && temperature_k <= t_high {
            temp_index_a = TemperatureIndex::T300K;
            temp_index_b = TemperatureIndex::T350K;
            temp_delta_a = f32::abs((temperature_k - t_low) / t_range);
            temp_delta_b = f32::abs((t_high - temperature_k) / t_range);
        }

        let mut press_index_a = PressureIndex::P0x01ATM;
        let mut press_index_b = PressureIndex::P0x01ATM;
        let mut press_delta_a = 0.0;
        let mut press_delta_b = 0.0;

        let pressure_interval_atm = 0.1;
        let p_low = 0.01;
        let p_high = pressure_interval_atm;
        let p_range = p_high - p_low;
        if pressure_atm <= pressure_interval_atm {
            press_index_a = PressureIndex::P0x01ATM;
            press_index_b = PressureIndex::P0x10ATM;
            press_delta_a = f32::abs((pressure_atm - p_low) / p_range);
            press_delta_b = f32::abs((p_high - pressure_atm) / p_range);
        }

        let next_pressure_interval = 0.4;
        let p_low = p_high;
        let p_high = next_pressure_interval;
        let p_range = p_high - p_low;
        if pressure_atm > p_low && pressure_atm <= p_high {
            press_index_a = PressureIndex::P0x10ATM;
            press_index_b = PressureIndex::P0x40ATM;
            press_delta_a = f32::abs((pressure_atm - p_low) / p_range);
            press_delta_b = f32::abs((p_high - pressure_atm) / p_range);
        }

        let next_pressure_interval = 0.7;
        let p_low = p_high;
        let p_high = next_pressure_interval;
        let p_range = p_high - p_low;
        if pressure_atm > p_low && pressure_atm <= p_high {
            press_index_a = PressureIndex::P0x40ATM;
            press_index_b = PressureIndex::P0x70ATM;
            press_delta_a = f32::abs((pressure_atm - p_low) / p_range);
            press_delta_b = f32::abs((p_high - pressure_atm) / p_range);
        }

        let next_pressure_interval = 1.0;
        let p_low = p_high;
        let p_high = next_pressure_interval;
        let p_range = p_high - p_low;
        if pressure_atm > p_low && pressure_atm <= p_high {
            press_index_a = PressureIndex::P0x70ATM;
            press_index_b = PressureIndex::P1x00ATM;
            press_delta_a = f32::abs((pressure_atm - p_low) / p_range);
            press_delta_b = f32::abs((p_high - pressure_atm) / p_range);
        }

        let next_pressure_interval = 4.0;
        let p_low = p_high;
        let p_high = next_pressure_interval;
        let p_range = p_high - p_low;
        if pressure_atm > p_low && pressure_atm <= p_high {
            press_index_a = PressureIndex::P1x00ATM;
            press_index_b = PressureIndex::P4x00ATM;
            press_delta_a = f32::abs((pressure_atm - p_low) / p_range);
            press_delta_b = f32::abs((p_high - pressure_atm) / p_range);
        }

        let next_pressure_interval = 7.0;
        let p_low = p_high;
        let p_high = next_pressure_interval;
        let p_range = p_high - p_low;
        if pressure_atm > p_low && pressure_atm <= p_high {
            press_index_a = PressureIndex::P4x00ATM;
            press_index_b = PressureIndex::P7x00ATM;
            press_delta_a = f32::abs((pressure_atm - p_low) / p_range);
            press_delta_b = f32::abs((p_high - pressure_atm) / p_range);
        }

        let next_pressure_interval = 10.0;
        let p_low = p_high;
        let p_high = next_pressure_interval;
        let p_range = p_high - p_low;
        if pressure_atm > p_low && pressure_atm <= p_high {
            press_index_a = PressureIndex::P7x00ATM;
            press_index_b = PressureIndex::P10x00ATM;
            press_delta_a = f32::abs((pressure_atm - p_low) / p_range);
            press_delta_b = f32::abs((p_high - pressure_atm) / p_range);
        }

        let next_pressure_interval = 40.0;
        let p_low = p_high;
        let p_high = next_pressure_interval;
        let p_range = p_high - p_low;
        if pressure_atm > p_low && pressure_atm <= p_high {
            press_index_a = PressureIndex::P10x00ATM;
            press_index_b = PressureIndex::P40x00ATM;
            press_delta_a = f32::abs((pressure_atm - p_low) / p_range);
            press_delta_b = f32::abs((p_high - pressure_atm) / p_range);
        }

        let next_pressure_interval = 70.0;
        let p_low = p_high;
        let p_high = next_pressure_interval;
        let p_range = p_high - p_low;
        if pressure_atm > p_low && pressure_atm <= p_high {
            press_index_a = PressureIndex::P40x00ATM;
            press_index_b = PressureIndex::P70x00ATM;
            press_delta_a = f32::abs((pressure_atm - p_low) / p_range);
            press_delta_b = f32::abs((p_high - pressure_atm) / p_range);
        }

        let next_pressure_interval = 100.0;
        let p_low = p_high;
        let p_high = next_pressure_interval;
        let p_range = p_high - p_low;
        if pressure_atm > p_low && pressure_atm <= p_high {
            press_index_a = PressureIndex::P70x00ATM;
            press_index_b = PressureIndex::P100x00ATM;
            press_delta_a = f32::abs((pressure_atm - p_low) / p_range);
            press_delta_b = f32::abs((p_high - pressure_atm) / p_range);
        }

        let value_t_1_p1 = table(&temp_index_a, &press_index_a);
        let value_t_1_p2 = table(&temp_index_a, &press_index_b);
        let value_t_2_p1 = table(&temp_index_b, &press_index_a);
        let value_t_2_p2 = table(&temp_index_b, &press_index_b);

        let scale_temp_1 = temp_delta_a;
        let scale_temp_2 = temp_delta_b;
        let scale_press_1 = press_delta_a;
        let scale_press_2 = press_delta_b;

        let value_t_1_p1 = value_t_1_p1 * scale_temp_1 * scale_press_1;
        let value_t_1_p2 = value_t_1_p2 * scale_temp_1 * scale_press_2;
        let value_t_2_p1 = value_t_2_p1 * scale_temp_2 * scale_press_1;
        let value_t_2_p2 = value_t_2_p2 * scale_temp_2 * scale_press_2;

        let total = value_t_1_p1 + value_t_1_p2 + value_t_2_p1 + value_t_2_p2;

        Some(total)
    }
}
