pub mod specific_heat_ratio {
    pub enum TemperatureIndex {
        T90K,
        T100K,
        T110K,
        T120K,
        T130K,
        T140K,
        T150K,
        T160K,
        T170K,
        T180K,
        T190K,
        T200K,
        T210K,
        T220K,
        T230K,
        T240K,
        T250K,
        T260K,
        T270K,
        T280K,
        T290K,
        T300K,
        T310K,
        T320K,
        T330K,
        T340K,
        T350K,
        T360K,
        T380K,
        T390K,
        T400K,
        T410K,
        T420K,
        T430K,
        T440K,
        T450K,
        T460K,
        T470K,
        T480K,
        T490K,
        T500K,
        T510K,
        T520K,
        T530K,
        T540K,
        T550K,
        T560K,
        T570K,
        T580K,
        T590K,
        T600K,
        T610K,
        T620K,
        T630K,
        T640K,
        T650K,
        T660K,
        T670K,
        T680K,
        T690K,
        T700K,
        T710K,
        T720K,
        T730K,
        T740K,
        T750K,
        T760K,
        T770K,
        T780K,
        T790K,
        T800K,
        T850K,
        T900K,
        T950K,
        T1000K,
        T1100K,
        T1200K,
        T1300K,
        T1400K,
        T1500K,
        T1600K,
        T1700K,
        T1800K,
        T1900K,
        T2000K,
        T2100K,
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
            TemperatureIndex::T90K => match press {
                PressureIndex::P0x01ATM => 1.4017,
                PressureIndex::P0x10ATM => 1.4046,
                PressureIndex::P0x40ATM => 1.4139,
                PressureIndex::P0x70ATM => 1.4237,
                _ => -1.0,
            },
            TemperatureIndex::T100K => match press {
                PressureIndex::P0x01ATM => 1.4016,
                PressureIndex::P0x10ATM => 1.4038,
                PressureIndex::P0x40ATM => 1.4108,
                PressureIndex::P0x70ATM => 1.4182,
                _ => -1.0,
            },
            TemperatureIndex::T110K => match press {
                PressureIndex::P0x01ATM => 1.4015,
                PressureIndex::P0x10ATM => 1.4032,
                PressureIndex::P0x40ATM => 1.4087,
                PressureIndex::P0x70ATM => 1.4144,
                PressureIndex::P1x00ATM => 1.4202,
                PressureIndex::P4x00ATM => 1.4960,
                PressureIndex::P7x00ATM => 1.6035,
                PressureIndex::P10x00ATM => 1.7672,
                _ => -1.0,
            },
            TemperatureIndex::T120K => match press {
                PressureIndex::P0x01ATM => 1.4015,
                PressureIndex::P0x10ATM => 1.4029,
                PressureIndex::P0x40ATM => 1.4073,
                PressureIndex::P0x70ATM => 1.4119,
                PressureIndex::P1x00ATM => 1.4166,
                PressureIndex::P4x00ATM => 1.4730,
                PressureIndex::P7x00ATM => 1.5513,
                PressureIndex::P10x00ATM => 1.6395,
                _ => -1.0,
            },
            TemperatureIndex::T130K => match press {
                PressureIndex::P0x01ATM => 1.4015,
                PressureIndex::P0x10ATM => 1.4026,
                PressureIndex::P0x40ATM => 1.4063,
                PressureIndex::P0x70ATM => 1.4101,
                PressureIndex::P1x00ATM => 1.4139,
                PressureIndex::P4x00ATM => 1.4578,
                PressureIndex::P7x00ATM => 1.5139,
                PressureIndex::P10x00ATM => 1.5740,
                _ => -1.0,
            },
            TemperatureIndex::T140K => match press {
                PressureIndex::P0x01ATM => 1.4015,
                PressureIndex::P0x10ATM => 1.4024,
                PressureIndex::P0x40ATM => 1.4055,
                PressureIndex::P0x70ATM => 1.4087,
                PressureIndex::P1x00ATM => 1.4119,
                PressureIndex::P4x00ATM => 1.4473,
                PressureIndex::P7x00ATM => 1.4901,
                PressureIndex::P10x00ATM => 1.5350,
                _ => -1.0,
            },
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
            TemperatureIndex::T260K => match press {
                PressureIndex::P0x01ATM => 1.4008,
                PressureIndex::P0x10ATM => 1.4010,
                PressureIndex::P0x40ATM => 1.4017,
                PressureIndex::P0x70ATM => 1.4024,
                PressureIndex::P1x00ATM => 1.4032,
                PressureIndex::P4x00ATM => 1.4107,
                PressureIndex::P7x00ATM => 1.4183,
                PressureIndex::P10x00ATM => 1.4259,
                PressureIndex::P40x00ATM => 1.5062,
                PressureIndex::P70x00ATM => 1.5885,
                PressureIndex::P100x00ATM => 1.6631,
            },
            TemperatureIndex::T270K => match press {
                PressureIndex::P0x01ATM => 1.4006,
                PressureIndex::P0x10ATM => 1.4008,
                PressureIndex::P0x40ATM => 1.4014,
                PressureIndex::P0x70ATM => 1.4022,
                PressureIndex::P1x00ATM => 1.4029,
                PressureIndex::P4x00ATM => 1.4097,
                PressureIndex::P7x00ATM => 1.4166,
                PressureIndex::P10x00ATM => 1.4236,
                PressureIndex::P40x00ATM => 1.4956,
                PressureIndex::P70x00ATM => 1.5683,
                PressureIndex::P100x00ATM => 1.6339,
            },
            TemperatureIndex::T280K => match press {
                PressureIndex::P0x01ATM => 1.4004,
                PressureIndex::P0x10ATM => 1.4006,
                PressureIndex::P0x40ATM => 1.4012,
                PressureIndex::P0x70ATM => 1.4018,
                PressureIndex::P1x00ATM => 1.4024,
                PressureIndex::P4x00ATM => 1.4087,
                PressureIndex::P7x00ATM => 1.4150,
                PressureIndex::P10x00ATM => 1.4214,
                PressureIndex::P40x00ATM => 1.4865,
                PressureIndex::P70x00ATM => 1.5511,
                PressureIndex::P100x00ATM => 1.6094,
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
            TemperatureIndex::T400K => match press {
                PressureIndex::P0x01ATM => 1.3952,
                PressureIndex::P0x10ATM => 1.3953,
                PressureIndex::P0x40ATM => 1.3956,
                PressureIndex::P0x70ATM => 1.3958,
                PressureIndex::P1x00ATM => 1.3961,
                PressureIndex::P4x00ATM => 1.3987,
                PressureIndex::P7x00ATM => 1.4014,
                PressureIndex::P10x00ATM => 1.4041,
                PressureIndex::P40x00ATM => 1.4299,
                PressureIndex::P70x00ATM => 1.4537,
                PressureIndex::P100x00ATM => 1.4752,
            },
            TemperatureIndex::T500K => match press {
                PressureIndex::P0x01ATM => 1.3866,
                PressureIndex::P0x10ATM => 1.3867,
                PressureIndex::P0x40ATM => 1.3868,
                PressureIndex::P0x70ATM => 1.3870,
                PressureIndex::P1x00ATM => 1.3871,
                PressureIndex::P4x00ATM => 1.3887,
                PressureIndex::P7x00ATM => 1.3903,
                PressureIndex::P10x00ATM => 1.3918,
                PressureIndex::P40x00ATM => 1.4065,
                PressureIndex::P70x00ATM => 1.4199,
                PressureIndex::P100x00ATM => 1.4321,
            },
            TemperatureIndex::T600K => match press {
                PressureIndex::P0x01ATM => 1.3757,
                PressureIndex::P0x10ATM => 1.3758,
                PressureIndex::P0x40ATM => 1.3758,
                PressureIndex::P0x70ATM => 1.3759,
                PressureIndex::P1x00ATM => 1.3760,
                PressureIndex::P4x00ATM => 1.3770,
                PressureIndex::P7x00ATM => 1.3780,
                PressureIndex::P10x00ATM => 1.3788,
                PressureIndex::P40x00ATM => 1.3882,
                PressureIndex::P70x00ATM => 1.3967,
                PressureIndex::P100x00ATM => 1.4041,
            },
            TemperatureIndex::T700K => match press {
                PressureIndex::P0x01ATM => 1.3643,
                PressureIndex::P0x10ATM => 1.3644,
                PressureIndex::P0x40ATM => 1.3644,
                PressureIndex::P0x70ATM => 1.3645,
                PressureIndex::P1x00ATM => 1.3646,
                PressureIndex::P4x00ATM => 1.3652,
                PressureIndex::P7x00ATM => 1.3658,
                PressureIndex::P10x00ATM => 1.3664,
                PressureIndex::P40x00ATM => 1.3725,
                PressureIndex::P70x00ATM => 1.3783,
                PressureIndex::P100x00ATM => 1.3832,
            },
            TemperatureIndex::T800K => match press {
                PressureIndex::P0x01ATM => 1.354,
                PressureIndex::P0x10ATM => 1.354,
                PressureIndex::P0x40ATM => 1.354,
                PressureIndex::P0x70ATM => 1.354,
                PressureIndex::P1x00ATM => 1.354,
                PressureIndex::P4x00ATM => 1.354,
                PressureIndex::P7x00ATM => 1.355,
                PressureIndex::P10x00ATM => 1.3551,
                PressureIndex::P40x00ATM => 1.3592,
                PressureIndex::P70x00ATM => 1.3630,
                PressureIndex::P100x00ATM => 1.3665,
            },
            TemperatureIndex::T900K => match press {
                PressureIndex::P0x01ATM => 1.344,
                PressureIndex::P0x10ATM => 1.344,
                PressureIndex::P0x40ATM => 1.344,
                PressureIndex::P0x70ATM => 1.344,
                PressureIndex::P1x00ATM => 1.345,
                PressureIndex::P4x00ATM => 1.345,
                PressureIndex::P7x00ATM => 1.345,
                PressureIndex::P10x00ATM => 1.3452,
                PressureIndex::P40x00ATM => 1.3480,
                PressureIndex::P70x00ATM => 1.3506,
                PressureIndex::P100x00ATM => 1.3533,
            },
            TemperatureIndex::T1000K => match press {
                PressureIndex::P0x01ATM => 1.336,
                PressureIndex::P0x10ATM => 1.336,
                PressureIndex::P0x40ATM => 1.336,
                PressureIndex::P0x70ATM => 1.336,
                PressureIndex::P1x00ATM => 1.336,
                PressureIndex::P4x00ATM => 1.336,
                PressureIndex::P7x00ATM => 1.336,
                PressureIndex::P10x00ATM => 1.3364,
                PressureIndex::P40x00ATM => 1.3386,
                PressureIndex::P70x00ATM => 1.3406,
                PressureIndex::P100x00ATM => 1.3423,
            },
            TemperatureIndex::T1100K => match press {
                PressureIndex::P0x01ATM => 1.329,
                PressureIndex::P0x10ATM => 1.329,
                PressureIndex::P0x40ATM => 1.329,
                PressureIndex::P0x70ATM => 1.329,
                PressureIndex::P1x00ATM => 1.329,
                PressureIndex::P4x00ATM => 1.329,
                PressureIndex::P7x00ATM => 1.329,
                PressureIndex::P10x00ATM => 1.3288,
                PressureIndex::P40x00ATM => 1.3303,
                PressureIndex::P70x00ATM => 1.3319,
                PressureIndex::P100x00ATM => 1.3333,
            },
            TemperatureIndex::T1200K => match press {
                PressureIndex::P0x01ATM => 1.322,
                PressureIndex::P0x10ATM => 1.322,
                PressureIndex::P0x40ATM => 1.322,
                PressureIndex::P0x70ATM => 1.322,
                PressureIndex::P1x00ATM => 1.322,
                PressureIndex::P4x00ATM => 1.322,
                PressureIndex::P7x00ATM => 1.322,
                PressureIndex::P10x00ATM => 1.3221,
                PressureIndex::P40x00ATM => 1.3232,
                PressureIndex::P70x00ATM => 1.3243,
                PressureIndex::P100x00ATM => 1.3254,
            },
            TemperatureIndex::T1300K => match press {
                PressureIndex::P0x01ATM => 1.316,
                PressureIndex::P0x10ATM => 1.316,
                PressureIndex::P0x40ATM => 1.316,
                PressureIndex::P0x70ATM => 1.316,
                PressureIndex::P1x00ATM => 1.316,
                PressureIndex::P4x00ATM => 1.316,
                PressureIndex::P7x00ATM => 1.316,
                PressureIndex::P10x00ATM => 1.3156,
                PressureIndex::P40x00ATM => 1.3165,
                PressureIndex::P70x00ATM => 1.3174,
                PressureIndex::P100x00ATM => 1.3181,
            },
            TemperatureIndex::T1400K => match press {
                PressureIndex::P0x01ATM => 1.310,
                PressureIndex::P0x10ATM => 1.310,
                PressureIndex::P0x40ATM => 1.310,
                PressureIndex::P0x70ATM => 1.310,
                PressureIndex::P1x00ATM => 1.310,
                PressureIndex::P4x00ATM => 1.310,
                PressureIndex::P7x00ATM => 1.310,
                PressureIndex::P10x00ATM => 1.3098,
                PressureIndex::P40x00ATM => 1.3106,
                PressureIndex::P70x00ATM => 1.3111,
                PressureIndex::P100x00ATM => 1.3117,
            },
            TemperatureIndex::T1500K => match press {
                PressureIndex::P0x01ATM => 1.304,
                PressureIndex::P0x10ATM => 1.304,
                PressureIndex::P0x40ATM => 1.304,
                PressureIndex::P0x70ATM => 1.304,
                PressureIndex::P1x00ATM => 1.304,
                PressureIndex::P4x00ATM => 1.304,
                PressureIndex::P7x00ATM => 1.304,
                PressureIndex::P10x00ATM => 1.3043,
                PressureIndex::P40x00ATM => 1.3047,
                PressureIndex::P70x00ATM => 1.3052,
                PressureIndex::P100x00ATM => 1.3056,
            },
            TemperatureIndex::T1600K => match press {
                PressureIndex::P0x01ATM => 1.298,
                PressureIndex::P0x10ATM => 1.299,
                PressureIndex::P0x40ATM => 1.299,
                PressureIndex::P0x70ATM => 1.299,
                PressureIndex::P1x00ATM => 1.299,
                PressureIndex::P4x00ATM => 1.299,
                PressureIndex::P7x00ATM => 1.299,
                PressureIndex::P10x00ATM => 1.299,
                PressureIndex::P40x00ATM => 1.301,
                PressureIndex::P70x00ATM => 1.302,
                PressureIndex::P100x00ATM => 1.303,
            },
            TemperatureIndex::T1700K => match press {
                PressureIndex::P0x01ATM => 1.290,
                PressureIndex::P0x10ATM => 1.292,
                PressureIndex::P0x40ATM => 1.293,
                PressureIndex::P0x70ATM => 1.293,
                PressureIndex::P1x00ATM => 1.293,
                PressureIndex::P4x00ATM => 1.293,
                PressureIndex::P7x00ATM => 1.293,
                PressureIndex::P10x00ATM => 1.293,
                PressureIndex::P40x00ATM => 1.294,
                PressureIndex::P70x00ATM => 1.296,
                PressureIndex::P100x00ATM => 1.297,
            },
            TemperatureIndex::T1800K => match press {
                PressureIndex::P0x01ATM => 1.280,
                PressureIndex::P0x10ATM => 1.286,
                PressureIndex::P0x40ATM => 1.287,
                PressureIndex::P0x70ATM => 1.288,
                PressureIndex::P1x00ATM => 1.288,
                PressureIndex::P4x00ATM => 1.288,
                PressureIndex::P7x00ATM => 1.288,
                PressureIndex::P10x00ATM => 1.288,
                PressureIndex::P40x00ATM => 1.289,
                PressureIndex::P70x00ATM => 1.290,
                PressureIndex::P100x00ATM => 1.291,
            },
            TemperatureIndex::T1900K => match press {
                PressureIndex::P0x01ATM => 1.266,
                PressureIndex::P0x10ATM => 1.277,
                PressureIndex::P0x40ATM => 1.281,
                PressureIndex::P0x70ATM => 1.281,
                PressureIndex::P1x00ATM => 1.282,
                PressureIndex::P4x00ATM => 1.283,
                PressureIndex::P7x00ATM => 1.283,
                PressureIndex::P10x00ATM => 1.283,
                PressureIndex::P40x00ATM => 1.284,
                PressureIndex::P70x00ATM => 1.285,
                PressureIndex::P100x00ATM => 1.286,
            },
            TemperatureIndex::T2000K => match press {
                PressureIndex::P0x01ATM => 1.243,
                PressureIndex::P0x10ATM => 1.266,
                PressureIndex::P0x40ATM => 1.272,
                PressureIndex::P0x70ATM => 1.273,
                PressureIndex::P1x00ATM => 1.274,
                PressureIndex::P4x00ATM => 1.277,
                PressureIndex::P7x00ATM => 1.278,
                PressureIndex::P10x00ATM => 1.278,
                PressureIndex::P40x00ATM => 1.279,
                PressureIndex::P70x00ATM => 1.280,
                PressureIndex::P100x00ATM => 1.281,
            },
            _ => todo!("Temperature Index not implemented"),
        }
    }

    pub fn interpolate(temperature_k: f32, pressure_atm: f32) -> Option<f32> {
        if temperature_k < 90.0 || temperature_k > 3000.0 {
            return None;
        }
        if temperature_k < 200.0 && pressure_atm >= 100.0 {
            return None;
        }
        if temperature_k < 180.0 && pressure_atm >= 70.0 {
            return None;
        }
        if temperature_k < 150.0 && pressure_atm > 40.0 {
            return None;
        }
        if temperature_k < 110.0 && pressure_atm > 0.7 {
            return None;
        }
        if temperature_k > 2100.0 && pressure_atm < 0.1 {
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
        let next_temp_interval = 260.0;
        let t_low = t_high;
        let t_high = next_temp_interval;
        let t_range = t_high - t_low;
        if temperature_k > t_low && temperature_k <= t_high {
            temp_index_a = TemperatureIndex::T250K;
            temp_index_b = TemperatureIndex::T260K;
            temp_delta_a = f32::abs((temperature_k - t_low) / t_range);
            temp_delta_b = f32::abs((t_high - temperature_k) / t_range);
        }
        let next_temp_interval = 270.0;
        let t_low = t_high;
        let t_high = next_temp_interval;
        let t_range = t_high - t_low;
        if temperature_k > t_low && temperature_k <= t_high {
            temp_index_a = TemperatureIndex::T260K;
            temp_index_b = TemperatureIndex::T270K;
            temp_delta_a = f32::abs((temperature_k - t_low) / t_range);
            temp_delta_b = f32::abs((t_high - temperature_k) / t_range);
        }
        let next_temp_interval = 280.0;
        let t_low = t_high;
        let t_high = next_temp_interval;
        let t_range = t_high - t_low;
        if temperature_k > t_low && temperature_k <= t_high {
            temp_index_a = TemperatureIndex::T270K;
            temp_index_b = TemperatureIndex::T280K;
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
        let next_temp_interval = 400.0;
        let t_low = t_high;
        let t_high = next_temp_interval;
        let t_range = t_high - t_low;
        if temperature_k > t_low && temperature_k <= t_high {
            temp_index_a = TemperatureIndex::T350K;
            temp_index_b = TemperatureIndex::T400K;
            temp_delta_a = f32::abs((temperature_k - t_low) / t_range);
            temp_delta_b = f32::abs((t_high - temperature_k) / t_range);
        }
        let next_temp_interval = 500.0;
        let t_low = t_high;
        let t_high = next_temp_interval;
        let t_range = t_high - t_low;
        if temperature_k > t_low && temperature_k <= t_high {
            temp_index_a = TemperatureIndex::T400K;
            temp_index_b = TemperatureIndex::T500K;
            temp_delta_a = f32::abs((temperature_k - t_low) / t_range);
            temp_delta_b = f32::abs((t_high - temperature_k) / t_range);
        }
        let next_temp_interval = 600.0;
        let t_low = t_high;
        let t_high = next_temp_interval;
        let t_range = t_high - t_low;
        if temperature_k > t_low && temperature_k <= t_high {
            temp_index_a = TemperatureIndex::T500K;
            temp_index_b = TemperatureIndex::T600K;
            temp_delta_a = f32::abs((temperature_k - t_low) / t_range);
            temp_delta_b = f32::abs((t_high - temperature_k) / t_range);
        }
        let next_temp_interval = 700.0;
        let t_low = t_high;
        let t_high = next_temp_interval;
        let t_range = t_high - t_low;
        if temperature_k > t_low && temperature_k <= t_high {
            temp_index_a = TemperatureIndex::T600K;
            temp_index_b = TemperatureIndex::T700K;
            temp_delta_a = f32::abs((temperature_k - t_low) / t_range);
            temp_delta_b = f32::abs((t_high - temperature_k) / t_range);
        }
        let next_temp_interval = 800.0;
        let t_low = t_high;
        let t_high = next_temp_interval;
        let t_range = t_high - t_low;
        if temperature_k > t_low && temperature_k <= t_high {
            temp_index_a = TemperatureIndex::T700K;
            temp_index_b = TemperatureIndex::T800K;
            temp_delta_a = f32::abs((temperature_k - t_low) / t_range);
            temp_delta_b = f32::abs((t_high - temperature_k) / t_range);
        }
        let next_temp_interval = 900.0;
        let t_low = t_high;
        let t_high = next_temp_interval;
        let t_range = t_high - t_low;
        if temperature_k > t_low && temperature_k <= t_high {
            temp_index_a = TemperatureIndex::T800K;
            temp_index_b = TemperatureIndex::T900K;
            temp_delta_a = f32::abs((temperature_k - t_low) / t_range);
            temp_delta_b = f32::abs((t_high - temperature_k) / t_range);
        }
        let next_temp_interval = 1000.0;
        let t_low = t_high;
        let t_high = next_temp_interval;
        let t_range = t_high - t_low;
        if temperature_k > t_low && temperature_k <= t_high {
            temp_index_a = TemperatureIndex::T900K;
            temp_index_b = TemperatureIndex::T1000K;
            temp_delta_a = f32::abs((temperature_k - t_low) / t_range);
            temp_delta_b = f32::abs((t_high - temperature_k) / t_range);
        }
        let next_temp_interval = 1100.0;
        let t_low = t_high;
        let t_high = next_temp_interval;
        let t_range = t_high - t_low;
        if temperature_k > t_low && temperature_k <= t_high {
            temp_index_a = TemperatureIndex::T1000K;
            temp_index_b = TemperatureIndex::T1100K;
            temp_delta_a = f32::abs((temperature_k - t_low) / t_range);
            temp_delta_b = f32::abs((t_high - temperature_k) / t_range);
        }
        let next_temp_interval = 1200.0;
        let t_low = t_high;
        let t_high = next_temp_interval;
        let t_range = t_high - t_low;
        if temperature_k > t_low && temperature_k <= t_high {
            temp_index_a = TemperatureIndex::T1100K;
            temp_index_b = TemperatureIndex::T1200K;
            temp_delta_a = f32::abs((temperature_k - t_low) / t_range);
            temp_delta_b = f32::abs((t_high - temperature_k) / t_range);
        }
        let next_temp_interval = 1300.0;
        let t_low = t_high;
        let t_high = next_temp_interval;
        let t_range = t_high - t_low;
        if temperature_k > t_low && temperature_k <= t_high {
            temp_index_a = TemperatureIndex::T1200K;
            temp_index_b = TemperatureIndex::T1300K;
            temp_delta_a = f32::abs((temperature_k - t_low) / t_range);
            temp_delta_b = f32::abs((t_high - temperature_k) / t_range);
        }
        let next_temp_interval = 1400.0;
        let t_low = t_high;
        let t_high = next_temp_interval;
        let t_range = t_high - t_low;
        if temperature_k > t_low && temperature_k <= t_high {
            temp_index_a = TemperatureIndex::T1300K;
            temp_index_b = TemperatureIndex::T1400K;
            temp_delta_a = f32::abs((temperature_k - t_low) / t_range);
            temp_delta_b = f32::abs((t_high - temperature_k) / t_range);
        }
        let next_temp_interval = 1500.0;
        let t_low = t_high;
        let t_high = next_temp_interval;
        let t_range = t_high - t_low;
        if temperature_k > t_low && temperature_k <= t_high {
            temp_index_a = TemperatureIndex::T1400K;
            temp_index_b = TemperatureIndex::T1500K;
            temp_delta_a = f32::abs((temperature_k - t_low) / t_range);
            temp_delta_b = f32::abs((t_high - temperature_k) / t_range);
        }
        let next_temp_interval = 1600.0;
        let t_low = t_high;
        let t_high = next_temp_interval;
        let t_range = t_high - t_low;
        if temperature_k > t_low && temperature_k <= t_high {
            temp_index_a = TemperatureIndex::T1500K;
            temp_index_b = TemperatureIndex::T1600K;
            temp_delta_a = f32::abs((temperature_k - t_low) / t_range);
            temp_delta_b = f32::abs((t_high - temperature_k) / t_range);
        }
        let next_temp_interval = 1700.0;
        let t_low = t_high;
        let t_high = next_temp_interval;
        let t_range = t_high - t_low;
        if temperature_k > t_low && temperature_k < t_high {
            temp_index_a = TemperatureIndex::T1600K;
            temp_index_b = TemperatureIndex::T1700K;
            temp_delta_a = f32::abs((temperature_k - t_low) / t_range);
            temp_delta_b = f32::abs((t_high - temperature_k) / t_range);
        }
        let next_temp_interval = 1800.0;
        let t_low = t_high;
        let t_high = next_temp_interval;
        let t_range = t_high - t_low;
        if temperature_k > t_low && temperature_k <= t_high {
            temp_index_a = TemperatureIndex::T1700K;
            temp_index_b = TemperatureIndex::T1800K;
            temp_delta_a = f32::abs((temperature_k - t_low) / t_range);
            temp_delta_b = f32::abs((t_high - temperature_k) / t_range);
        }
        let next_temp_interval = 1900.0;
        let t_low = t_high;
        let t_high = next_temp_interval;
        let t_range = t_high - t_low;
        if temperature_k > t_low && temperature_k <= t_high {
            temp_index_a = TemperatureIndex::T1800K;
            temp_index_b = TemperatureIndex::T1900K;
            temp_delta_a = f32::abs((temperature_k - t_low) / t_range);
            temp_delta_b = f32::abs((t_high - temperature_k) / t_range);
        }
        let next_temp_interval = 2000.0;
        let t_low = t_high;
        let t_high = next_temp_interval;
        let t_range = t_high - t_low;
        if temperature_k > t_low && temperature_k <= t_high {
            temp_index_a = TemperatureIndex::T1900K;
            temp_index_b = TemperatureIndex::T2000K;
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

        let scale_temp_1 = 1.0 - temp_delta_a;
        let scale_temp_2 = 1.0 - temp_delta_b;
        let scale_press_1 = 1.0 - press_delta_a;
        let scale_press_2 = 1.0 - press_delta_b;

        let value_t_1_p1 = value_t_1_p1 * scale_temp_1 * scale_press_1;
        let value_t_1_p2 = value_t_1_p2 * scale_temp_1 * scale_press_2;
        let value_t_2_p1 = value_t_2_p1 * scale_temp_2 * scale_press_1;
        let value_t_2_p2 = value_t_2_p2 * scale_temp_2 * scale_press_2;

        let total = value_t_1_p1 + value_t_1_p2 + value_t_2_p1 + value_t_2_p2;

        Some(total)
    }
}
