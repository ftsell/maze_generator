//! Common traits and members

use anyhow::Result;
use thiserror::Error;

pub use coordinates::*;
pub use direction::*;
pub use field::*;
pub use maze::*;
pub use svgoptions::*;

mod coordinates;
mod direction;
mod field;
mod maze;
mod svgoptions;

/// Generic error type that could be returned by all implemented generators.
#[derive(Error, Debug, Clone, Eq, PartialEq)]
pub enum GenericGeneratorError {
    /// Error that can be raised anywhere in a generator that is not otherwise explicitly handled.
    ///
    /// This is used as a way to signal bugs.
    /// They should hopefully never actually be raised but if they are, a bug should be reported.
    #[error("Unknown internal error. If this is reproducible, please report a bug at https://github.com/ftsell/maze_generator/issues/new : {0}")]
    InternalError(String),
}

/// Generic generator Api implemented by all algorithms to generate a maze
pub trait Generator {
    /// Key function to generate a maze
    ///
    /// The returned [`Maze`] will have the provided width and height.
    /// It can be any rectangular shape.
    fn generate(&mut self, width: i32, height: i32) -> Result<Maze>;
}
