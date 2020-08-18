#![deny(trivial_casts, trivial_numeric_casts, unsafe_code)]
#![warn(
    missing_crate_level_docs,
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    unused_crate_dependencies,
    unused_extern_crates,
    unused_import_braces,
    unused_lifetimes,
    unused_qualifications,
    unused_results
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
//! let grid = generator.generate(3, 3);
//!
//! assert_eq!(format!("{:?}", grid),
//! "·-·-·-·
//! |S|   |
//! · ·-· ·
//! |     |
//! ·-·-· ·
//! |     |
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
//! let grid = generator.generate(3, 3);
//!
//! assert_eq!(format!("{:?}", grid.get_field(grid.get_start()).unwrap()),
//!            "RbField { north: \"wall\", east: \"wall\", south: \"passage\", west: \"wall\" }");
//! ```
//!

#[macro_use]
pub mod prelude;
pub mod recursive_backtracking;

#[macro_use]
extern crate bitflags;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
