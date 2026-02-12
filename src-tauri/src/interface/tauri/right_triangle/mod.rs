// interface/tauri/right_triangle/mod.rs

// interface/tauri/right_triangle/mod.rs

pub mod command;
pub mod request;
pub mod response;
pub mod mapping;

pub use command::solve_right_triangle;
pub use request::SolveRightTriangleRequest;
pub use response::SolveRightTriangleResponse;
