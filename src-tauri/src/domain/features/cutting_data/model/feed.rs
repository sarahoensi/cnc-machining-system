// cutting_data/model/feed

use crate::domain::features::cutting_data::model::values::*;

#[derive(Debug, Clone, Copy)]
pub enum Feed {
    FeedRate(FeedRateMmMin),
    FeedPerTooth(FeedPerToothMm),
}
