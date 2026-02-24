// domain/units/machining/mod.rs

mod cutting_speed;
mod chip_load;
mod error;

pub use cutting_speed::CuttingSpeed;
pub use chip_load::ChipLoad;
pub use error::MachiningUnitError;