// tests/cutting_data/common/builders.rs
use cnc_machining_system_lib::domain::features::cutting_data::input::raw::RawCuttingInput;

// ======================================================
// BASELINE
// ======================================================

pub fn valid_raw() -> RawCuttingInput {
    RawCuttingInput {
        d: Some(10.0),
        z: Some(4),
        vc: Some(200.0),
        n: None,
        f: None,
        fz: None,
    }
}

// ======================================================
// MUTATION HELPERS
// ======================================================

pub fn without_speed(mut raw: RawCuttingInput) -> RawCuttingInput {
    raw.vc = None;
    raw.n = None;
    raw
}

pub fn with_spindle(mut raw: RawCuttingInput) -> RawCuttingInput {
    raw.vc = None;
    raw.n = Some(6000.0);
    raw
}

pub fn with_feed_rate(mut raw: RawCuttingInput) -> RawCuttingInput {
    raw.f = Some(1000.0);
    raw.fz = None;
    raw
}

pub fn with_feed_per_tooth(mut raw: RawCuttingInput) -> RawCuttingInput {
    raw.f = None;
    raw.fz = Some(0.05);
    raw
}

pub fn without_feed(mut raw: RawCuttingInput) -> RawCuttingInput {
    raw.f = None;
    raw.fz = None;
    raw
}

// ======================================================
// ERROR SCENARIOS
// ======================================================

pub fn missing_d(mut raw: RawCuttingInput) -> RawCuttingInput {
    raw.d = None;
    raw
}

pub fn missing_z(mut raw: RawCuttingInput) -> RawCuttingInput {
    raw.z = None;
    raw
}
