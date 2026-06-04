// tests/domain/machining_physics/cutting_data_solver_test.rs

use cnc_machining_system_lib::domain::machining::{CuttingSolver, Tool};
use cnc_machining_system_lib::domain::units::*;
use cnc_machining_system_lib::test_utils::approx::{approx_eq, DEFAULT_EPS};

use proptest::prelude::*;

//
// ------------------------------------------------------------
// Helpers
// ------------------------------------------------------------
//

fn tool(d: f64, z: i32) -> Tool {
    Tool::new(Diameter::mm(d).unwrap(), ToothCount::new(z).unwrap())
}

//
// ------------------------------------------------------------
// Deterministic tests
// ------------------------------------------------------------
//

#[test]
fn from_speed_and_chip_load_produces_consistent_set() {
    let tool = tool(10.0, 4);

    let vc = CuttingSpeed::meters_per_min(120.0).unwrap();
    let chip = ChipLoad::mm_per_tooth(0.05).unwrap();

    let params = CuttingSolver::from_speed_and_chip_load(vc, chip, tool).unwrap();

    // Recalculate chip from feed
    let reconstructed =
        CuttingSolver::chip_from_feed(params.feed_rate(), params.rpm(), tool.teeth()).unwrap();

    assert!(approx_eq(
        chip.mm_per_tooth_value(),
        reconstructed.mm_per_tooth_value(),
        DEFAULT_EPS
    ));
}

#[test]
fn from_rpm_and_feed_produces_consistent_set() {
    let tool = tool(12.0, 3);

    let rpm = Rpm::new(800.0).unwrap();
    let feed = FeedRate::mm_per_min(180.0).unwrap();

    let params = CuttingSolver::from_rpm_and_feed(rpm, feed, tool).unwrap();

    let reconstructed =
        CuttingSolver::feed_from_chip_load(params.chip_load(), rpm, tool.teeth()).unwrap();

    assert!(approx_eq(
        feed.mm_per_min_value(),
        reconstructed.mm_per_min_value(),
        DEFAULT_EPS
    ));
}

#[test]
fn rpm_scales_inverse_with_diameter() {
    let tool1 = tool(10.0, 4);
    let tool2 = tool(20.0, 4);

    let vc = CuttingSpeed::meters_per_min(100.0).unwrap();
    let chip = ChipLoad::mm_per_tooth(0.04).unwrap();

    let p1 = CuttingSolver::from_speed_and_chip_load(vc, chip, tool1).unwrap();
    let p2 = CuttingSolver::from_speed_and_chip_load(vc, chip, tool2).unwrap();

    assert!(approx_eq(
        p2.rpm().value(),
        p1.rpm().value() / 2.0,
        DEFAULT_EPS
    ));
}

//
// ------------------------------------------------------------
// Property tests
// ------------------------------------------------------------
//

fn positive_f64(min: f64, max: f64) -> impl Strategy<Value = f64> {
    (min..max).prop_filter("finite", |v| v.is_finite())
}

fn diameter() -> impl Strategy<Value = Diameter> {
    positive_f64(0.5, 200.0).prop_map(|v| Diameter::mm(v).unwrap())
}

fn cutting_speed() -> impl Strategy<Value = CuttingSpeed> {
    positive_f64(10.0, 500.0).prop_map(|v| CuttingSpeed::meters_per_min(v).unwrap())
}

fn chip_load() -> impl Strategy<Value = ChipLoad> {
    positive_f64(0.001, 0.3).prop_map(|v| ChipLoad::mm_per_tooth(v).unwrap())
}

fn rpm() -> impl Strategy<Value = Rpm> {
    positive_f64(50.0, 40000.0).prop_map(|v| Rpm::new(v).unwrap())
}

fn feed_rate() -> impl Strategy<Value = FeedRate> {
    positive_f64(10.0, 20000.0).prop_map(|v| FeedRate::mm_per_min(v).unwrap())
}

fn tooth_count() -> impl Strategy<Value = ToothCount> {
    (1i32..16i32).prop_map(|z| ToothCount::new(z).unwrap())
}

proptest! {

    #[test]
    fn full_chain_consistency_speed_chip(
        d in diameter(),
        vc in cutting_speed(),
        chip in chip_load(),
        z in tooth_count()
    ) {

        let tool = Tool::new(d, z);

        let params =
            CuttingSolver::from_speed_and_chip_load(vc, chip, tool)
                .unwrap();

        let chip2 =
            CuttingSolver::chip_from_feed(
                params.feed_rate(),
                params.rpm(),
                tool.teeth(),
            ).unwrap();

        prop_assert!(approx_eq(
            chip.mm_per_tooth_value(),
            chip2.mm_per_tooth_value(),
            1e-9
        ));
    }

    #[test]
    fn full_chain_consistency_rpm_feed(
        d in diameter(),
        n in rpm(),
        f in feed_rate(),
        z in tooth_count()
    ) {

        let tool = Tool::new(d, z);

        let params =
            CuttingSolver::from_rpm_and_feed(n, f, tool)
                .unwrap();

        let feed2 =
            CuttingSolver::feed_from_chip_load(
                params.chip_load(),
                params.rpm(),
                tool.teeth(),
            ).unwrap();

        prop_assert!(approx_eq(
            f.mm_per_min_value(),
            feed2.mm_per_min_value(),
            1e-9
        ));
    }
}
