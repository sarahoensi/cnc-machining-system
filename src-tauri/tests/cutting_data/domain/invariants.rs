// tests/cutting_data/domain/invariants.rs

use cnc_machining_system_lib::domain::features::cutting_data::model::{
    CuttingData, Feed, Speed,
};

use cnc_machining_system_lib::domain::features::cutting_data::model::values::*;

// ======================================================
// VARIANT INTEGRITY
// ======================================================

#[test]
fn speed_variant_is_always_valid() {
    let data = CuttingData {
        diameter: DiameterMm::new(12.0).unwrap(),
        teeth: Some(ToothCount::new(3).unwrap()),
        speed: Some(Speed::SpindleSpeed(
            SpindleSpeedRpm::new(8000.0).unwrap(),
        )),
        feed: None,
    };

    match data.speed {
        Some(Speed::SpindleSpeed(_)) => {}
        _ => panic!("Invalid speed variant"),
    }
}

#[test]
fn feed_variant_is_always_valid() {
    let data = CuttingData {
        diameter: DiameterMm::new(12.0).unwrap(),
        teeth: Some(ToothCount::new(3).unwrap()),
        speed: None,
        feed: Some(Feed::FeedRate(
            FeedRateMmMin::new(500.0).unwrap(),
        )),
    };

    match data.feed {
        Some(Feed::FeedRate(_)) => {}
        _ => panic!("Invalid feed variant"),
    }
}

// ======================================================
// VALUE OBJECT INTEGRITY
// ======================================================

#[test]
fn domain_contains_only_valid_value_objects() {
    let data = CuttingData {
        diameter: DiameterMm::new(20.0).unwrap(),
        teeth: Some(ToothCount::new(5).unwrap()),
        speed: None,
        feed: None,
    };

    assert!(data.diameter.value() > 0.0);
    assert!(data.teeth.is_some());
    assert!(data.teeth.as_ref().unwrap().value() > 0);
}

