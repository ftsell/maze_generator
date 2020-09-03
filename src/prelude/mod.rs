//! Common traits and members

mod coordinates;
mod direction;
mod field;
mod maze;

pub use coordinates::*;
pub use direction::*;
pub use field::*;
pub use maze::*;

/// Generic generator Api implemented by all algorithms to generate a maze
pub trait Generator {
    /// Key function to generate a maze
    ///
    /// The returned [`Maze`] will have the provided width and height.
    /// It can be any rectangular shape.
    fn generate(&mut self, width: i32, height: i32) -> Maze;
}
