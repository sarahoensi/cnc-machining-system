// domain/units/motion/mod.rs

mod rpm;
mod feed_rate;
mod error;

pub use rpm::Rpm;
pub use feed_rate::FeedRate;
pub use error::MotionUnitError;