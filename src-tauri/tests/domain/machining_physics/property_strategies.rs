// tests/domain/machining_physics/property_strategies.rs
#![allow(dead_code)]


use cnc_machining_system_lib::domain::*;
use proptest::prelude::*;

/// Positive finite f64 helper
fn positive_f64(min: f64, max: f64) -> impl Strategy<Value = f64> {
    (min..max).prop_filter("finite", |v| v.is_finite())
}

pub fn diameter() -> impl Strategy<Value = Diameter> {
    positive_f64(0.1, 200.0)
        .prop_map(|v| Diameter::mm(v).unwrap())
}

pub fn rpm() -> impl Strategy<Value = Rpm> {
    positive_f64(10.0, 50000.0)
        .prop_map(|v| Rpm::new(v).unwrap())
}

pub fn cutting_speed() -> impl Strategy<Value = CuttingSpeed> {
    positive_f64(1.0, 500.0)
        .prop_map(|v| CuttingSpeed::meters_per_min(v).unwrap())
}

pub fn chip_load() -> impl Strategy<Value = ChipLoad> {
    positive_f64(0.001, 0.5)
        .prop_map(|v| ChipLoad::mm_per_tooth(v).unwrap())
}

pub fn feed_rate() -> impl Strategy<Value = FeedRate> {
    positive_f64(1.0, 20000.0)
        .prop_map(|v| FeedRate::mm_per_min(v).unwrap())
}

pub fn tooth_count() -> impl Strategy<Value = ToothCount> {
    (1u32..16u32).prop_map(|z| ToothCount::new(z).unwrap())
}
