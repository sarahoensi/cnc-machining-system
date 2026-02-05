// tests/cutting_data/common/strategies/domain.rs

use cnc_machining_system_lib::domain::features::cutting_data::model::*;
use proptest::prelude::*;

use super::values::*;

pub fn cutting_data_with_speed_and_feed() -> impl Strategy<Value = CuttingData> {

    (
        diameter(),
        tooth_count(),
        cutting_speed(),
        feed_per_tooth(),
    )
    .prop_map(|(d, z, vc, fz)| CuttingData {
        diameter: d,
        teeth: z,
        speed: Some(Speed::CuttingSpeed(vc)),
        feed: Some(Feed::FeedPerTooth(fz)),
    })
}
