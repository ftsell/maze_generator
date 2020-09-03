#![deny(trivial_numeric_casts, trivial_casts, unsafe_code)]
#![warn(
    missing_crate_level_docs,
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    unused_import_braces,
    unused_lifetimes,
    unused_qualifications
)]

//! This is a collection of different maze generation algorithms.
//!
//! The project's main goal is to provide an easy-to-use API to different algorithms with different characteristics.
//!
//! # Examples
//! ```
//! // Generate a 3 by 3 maze using a provided seed and the recursive-backtracking algorithm
//! use maze_generator::prelude::*;
//! use maze_generator::recursive_backtracking::RbGenerator;
//!
//! let mut generator = RbGenerator::new(Some([42; 32]));
//! let maze = generator.generate(3, 3);
//!
//! assert_eq!(format!("{:?}", maze),
//! "·-·-·-·
//! |S|   |
//! · ·-· ·
//! |     |
//! ·-·-· ·
//! |G    |
//! ·-·-·-·
//! ");
//! ```
//!
//! ```
//! // Retrieve information about a specific cell from the maze
//! use maze_generator::prelude::*;
//! use maze_generator::recursive_backtracking::RbGenerator;
//!
//! let mut generator = RbGenerator::new(Some([42; 32]));
//! let maze = generator.generate(3, 3);
//!
//! assert_eq!(format!("{:?}", maze.get_field(&maze.start).unwrap()),
//!            "Field { north: \"wall\", east: \"wall\", south: \"passage\", west: \"wall\" }");
//! ```

#[cfg(test)]
#[macro_use]
extern crate quickcheck;
#[cfg(test)]
#[macro_use]
mod test_util;

#[macro_use]
pub mod prelude;
pub mod recursive_backtracking;
