// tests/cutting_data/value_objects/properties.rs

use cnc_machining_system_lib::domain::features::cutting_data::model::values::*;
use proptest::prelude::*;

// ======================================================
// DIAMETER
// ======================================================

proptest! {

    #[test]
    fn diameter_roundtrip(v in positive_f64()) {
        let d = DiameterMm::new(v).unwrap();
        prop_assert_eq!(d.value(), v);
    }
}

// ======================================================
// CUTTING SPEED
// ======================================================

proptest! {

    #[test]
    fn cutting_speed_roundtrip(v in positive_f64()) {
        let vc = CuttingSpeedMMin::new(v).unwrap();
        prop_assert_eq!(vc.value(), v);
    }
}

// ======================================================
// SPINDLE SPEED
// ======================================================

proptest! {

    #[test]
    fn spindle_speed_roundtrip(v in positive_f64()) {
        let n = SpindleSpeedRpm::new(v).unwrap();
        prop_assert_eq!(n.value(), v);
    }
}

// ======================================================
// FEED RATE
// ======================================================

proptest! {

    #[test]
    fn feed_rate_roundtrip(v in positive_f64()) {
        let f = FeedRateMmMin::new(v).unwrap();
        prop_assert_eq!(f.value(), v);
    }
}

// ======================================================
// FEED PER TOOTH
// ======================================================

proptest! {

    #[test]
    fn feed_per_tooth_roundtrip(v in positive_f64()) {
        let fz = FeedPerToothMm::new(v).unwrap();
        prop_assert_eq!(fz.value(), v);
    }
}

// ======================================================
// TOOTH COUNT
// ======================================================

proptest! {

    #[test]
    fn tooth_count_roundtrip(v in positive_u32()) {
        let z = ToothCount::new(v).unwrap();
        prop_assert_eq!(z.value(), v);
    }
}

// ======================================================
// STRATEGIES
// ======================================================

fn positive_f64() -> impl Strategy<Value = f64> {
    0.000001f64..100000.0
}

fn positive_u32() -> impl Strategy<Value = u32> {
    1u32..128u32
}
