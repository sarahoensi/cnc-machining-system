// interface/tauri/right_triangle/mod.rs

 mod command;
 mod request;
 mod response;
 mod mapping;

pub use command::solve_right_triangle;
pub use request::SolveRightTriangleRequest;
pub use response::SolveRightTriangleResponse;
