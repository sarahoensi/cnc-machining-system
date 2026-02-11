// application/mod.rs
 mod right_triangle;
 mod helix;
 mod shared;
 mod cutting_data;
 pub mod finishing;

pub use shared::ApplicationError;

pub use right_triangle::solve_right_triangle_use_case::SolveRightTriangleUseCase;
pub use right_triangle::dto::{SolveRightTriangleInput, SolveRightTriangleOutput};

pub use helix::solve_helix_use_case::SolveHelixUseCase;
pub use helix::dto::{SolveHelixInput, SolveHelixOutput, HelixMode};

pub use cutting_data::dto::{SolveCuttingDataInput, SolveCuttingDataOutput};
pub use cutting_data::solve_cutting_data_use_case::SolveCuttingDataUseCase;