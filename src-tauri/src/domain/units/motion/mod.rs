// domain/units/motion/mod.rs

mod error;
mod feed_rate;
mod rpm;

pub use error::MotionUnitError;
pub use feed_rate::FeedRate;
pub use rpm::Rpm;
