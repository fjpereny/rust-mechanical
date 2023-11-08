pub mod gas {
    // Ideal Gas Constant
    const IDEAL_GAS_CONST: f32 = 8.31446261815324;

    pub struct Gas {
        pub specific_heat_ratio: f32,
    }

    pub const AIR: Gas = Gas {
        specific_heat_ratio: 1.40,
    };
}

pub mod fluid {
    pub mod specific_heat {}
}
