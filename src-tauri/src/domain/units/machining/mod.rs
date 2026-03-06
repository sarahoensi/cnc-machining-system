// domain/units/machining/mod.rs

mod cutting_speed;
mod chip_load;
mod error;
mod toothcount;

pub use cutting_speed::CuttingSpeed;
pub use chip_load::ChipLoad;
pub use error::MachiningUnitError;
pub use toothcount::ToothCount;