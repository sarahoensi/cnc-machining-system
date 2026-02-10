// interface/tauri/finishing/mod.rs

// interface/tauri/finishing/mod.rs

// interface/tauri/finishing/mod.rs

mod request;
mod response;
mod mapping;
mod command;

pub use command::{
    generate_finishing_plan,
    register_finishing_measurement,
    clear_finishing_measurement,
};
