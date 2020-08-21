use super::field::*;
use super::grid::*;
use crate::prelude::*;
use rand::prelude::*;
use rand_chacha::ChaChaRng;

/// [`Generator`] implementation used by the recursive-backtracking algorithm
#[derive(Debug, Clone)]
pub struct RbGenerator {
    rng: ChaChaRng,
}

impl RbGenerator {
    /// Create a new [`Generator`] instance which uses recursive-backtracking to generate a maze
    ///
    /// Optionally a 32 bit seed can be provided to seed the internal random generator.
    /// Giving a seed results in identical mazes being generated which omitting it sources the
    /// random generator from entropy.
    pub fn new(seed: Option<[u8; 32]>) -> RbGenerator {
        RbGenerator {
            rng: match seed {
                None => ChaChaRng::from_entropy(),
                Some(seed) => ChaChaRng::from_seed(seed),
            },
        }
    }

    fn carve_passages_from(&mut self, coordinates: Coordinates, grid: &mut RbGrid) {
        for i_dir in Direction::gen_random_order(&mut self.rng).iter() {
            let next_x = coordinates.0
                + match i_dir {
                    Direction::East => 1,
                    Direction::West => -1,
                    _ => 0,
                };
            let next_y = coordinates.1
                + match i_dir {
                    Direction::North => -1,
                    Direction::South => 1,
                    _ => 0,
                };

            match grid.get_field(&(next_x, next_y)) {
                Ok(next_field) => {
                    if next_field.is_untouched() {
                        // set passage on next field
                        let mut new_field = (*next_field).clone();
                        new_field.add_passage(&i_dir.opposite());
                        grid.set_field((next_x, next_y), new_field);

                        // set passage on original field
                        new_field = (*grid.get_field(&coordinates).unwrap()).clone();
                        new_field.add_passage(i_dir);
                        grid.set_field(coordinates, new_field);

                        self.carve_passages_from((next_x, next_y), grid);
                    }
                }
                Err(_) => { /* Coordinates are not inside grid */ }
            }
        }
    }
}

impl Generator for RbGenerator {
    type GridType = RbGrid;

    fn generate(&mut self, width: i32, height: i32) -> Self::GridType {
        let mut grid = RbGrid {
            size: (width, height),
            start: (0, 0),
            goal: (0, 0),
            data: Vec::new(),
        };

        for _ix in 0..width {
            for _iy in 0..height {
                grid.data.push(RbField::new());
            }
        }

        self.carve_passages_from((0, 0), &mut grid);
        grid
    }
}
