//! Recursive-Backtracking algorithm implementation
//!
//! Recursive backtracking is fast, easy to understand and straightforward.
//! Its downsides are relatively large memory and stack size requirements.
//!
//! The algorithm works as follows:
//!
//! 1. Choose a starting point in the field (in this implementation 0,0) and make it the current cell
//! 2. Randomly choose a direction, check if the field in that direction has not yet been visited.
//!     If that is the case, make the cell in that direction the new current cell and carve a passage between the two.
//! 3. If all adjacent fields have been visited, back up to the last field with unvisited neighbors.
//! 4. The algorithm terminates when it has backed up all the way to the starting point.
//!

mod field;
mod generator;
mod grid;

pub use field::RbField;
pub use generator::RbGenerator;
pub use grid::RbGrid;
