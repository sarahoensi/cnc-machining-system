// application/mod.rs
 mod right_triangle;
 mod shared;

pub use shared::ApplicationError;
pub use right_triangle::solve_right_triangle_use_case::SolveRightTriangleUseCase;
pub use right_triangle::dto::{SolveRightTriangleInput, SolveRightTriangleOutput};
