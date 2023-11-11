pub mod specific_heat_ratio {

    pub enum TemperatureIndex {
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
    }

    pub enum PressureIndex {
        P0x01BAR,
        P0x10BAR,
        P0x40BAR,
        P0x70BAR,
        P1x00BAR,
        P4x00BAR,
        P7x00BAR,
        P10x00BAR,
        P40x00BAR,
        P70x00BAR,
        P100x00BAR,
    }

    pub fn table(temp: TemperatureIndex, press: PressureIndex) -> Option<f32> {
        match temp {
            TemperatureIndex::T200K => match press {
                PressureIndex::P0x01BAR => Some(1.4013),
                PressureIndex::P0x10BAR => Some(1.4017),
                PressureIndex::P0x40BAR => Some(1.4030),
                PressureIndex::P0x70BAR => Some(1.4043),
                PressureIndex::P1x00BAR => Some(1.4057),
                PressureIndex::P4x00BAR => Some(1.4197),
                PressureIndex::P7x00BAR => Some(1.4344),
                PressureIndex::P10x00BAR => Some(1.4489),
                PressureIndex::P40x00BAR => Some(1.6418),
                PressureIndex::P70x00BAR => Some(1.9000),
                PressureIndex::P100x00BAR => Some(2.1376),
            },
            _ => None,
        }
    }
}
