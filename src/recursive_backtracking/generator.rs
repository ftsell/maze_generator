use crate::prelude::*;
use rand::prelude::*;
use rand_chacha::ChaChaRng;

/// [`Generator`] implementation which uses the recursive-backtracking algorithm.
#[derive(Debug, Clone)]
pub struct RbGenerator {
    rng: ChaChaRng,
}

impl RbGenerator {
    /// Create a new instance.
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

    /// Core algorithm implementation
    ///
    /// Carves passages in all directions in random order from the current coordinates but only
    /// if the field in that direction has not yet been processed.
    fn carve_passages_from(&mut self, maze: &mut Maze, current_coordinates: Coordinates) {
        for i_dir in Direction::gen_random_order(&mut self.rng).iter() {
            let next_coords = current_coordinates.next(i_dir);

            if maze.are_coordinates_inside(&next_coords)
                && maze.grid.neighbors(next_coords).count() == 0
            {
                // TODO set goal field correctly
                maze.grid.add_edge(current_coordinates, next_coords, ());
                self.carve_passages_from(maze, next_coords);
            }
        }
    }
}

impl Generator for RbGenerator {
    fn generate(&mut self, width: i32, height: i32) -> Maze {
        let start = (0, 0).into();
        let mut maze = Maze::new(width, height, start, (0, 0).into());

        self.carve_passages_from(&mut maze, start);

        maze
    }
}
