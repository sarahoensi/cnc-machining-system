// tests/cutting_data/common/strategies/values.rs

use cnc_machining_system_lib::domain::features::cutting_data::model::values::*;
use proptest::prelude::*;

/// Biased positive f64 strategy with improved coverage.
/// 
/// Combines multiple sampling strategies to improve test coverage:
/// - Boundary values (min and max)
/// - Small numbers near min (for edge cases)
/// - Logarithmic distribution (for wide ranges)
/// - Uniform fallback (general coverage)
fn biased_positive_f64(min: f64, max: f64) -> impl Strategy<Value = f64> {
    prop_oneof![
        // Boundary values
        Just(min),
        Just(max),
        
        // Small numbers near minimum (edge case coverage)
        Just(min * 1.1),
        Just(min * 2.0),
        
        // Logarithmic distribution for better coverage of wide ranges
        (0.0f64..1.0)
            .prop_map(move |t| min * (max / min).powf(t)),
        
        // Uniform fallback for general coverage
        (min..max),
    ]
}

pub fn diameter() -> impl Strategy<Value = DiameterMm> {
    biased_positive_f64(0.1, 200.0).prop_filter_map("valid diameter", |d| DiameterMm::new(d).ok())
}

pub fn cutting_speed() -> impl Strategy<Value = CuttingSpeedMMin> {
    biased_positive_f64(1.0, 500.0).prop_filter_map("valid cutting speed", |v| CuttingSpeedMMin::new(v).ok())
}

pub fn spindle_speed() -> impl Strategy<Value = SpindleSpeedRpm> {
    biased_positive_f64(10.0, 50000.0).prop_filter_map("valid spindle speed", |n| SpindleSpeedRpm::new(n).ok())
}

pub fn tooth_count() -> impl Strategy<Value = ToothCount> {
    (1u32..16u32).prop_filter_map("valid tooth count", |z| ToothCount::new(z).ok())
}

pub fn feed_per_tooth() -> impl Strategy<Value = FeedPerToothMm> {
    biased_positive_f64(0.001, 0.5).prop_filter_map("valid feed per tooth", |f| FeedPerToothMm::new(f).ok())
}

pub fn feed_rate() -> impl Strategy<Value = FeedRateMmMin> {
    biased_positive_f64(1.0, 10000.0).prop_filter_map("valid feed rate", |f| FeedRateMmMin::new(f).ok())
}
