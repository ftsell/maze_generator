//! Growing tree implementation
//!
//! 1. Let C be a list of cells, initially empty. Add one cell to C, at random.
//! 2. Choose a cell from C, and carve a passage to any unvisited neighbor of
//!    that cell, adding that neighbor to C as well. If there are no unvisited
//!    neighbors, remove the cell from C.
//! 3. Repeat #2 until C is empty
//!
//! Pretty straight-forward, really. But the fun lies in how you choose the cells
//! from C, in step #2. If you always choose the newest cell (the one most
//! recently added), you’ll get the recursive backtracker. If you always choose a
//! cell at random, you get Prim’s. It’s remarkably fun to experiment with other
//! ways to choose cells from C.
//!
//! *Explanation and credits to
//! [Jamis Buck's Buckblog]( http://weblog.jamisbuck.org/2011/1/27/maze-generation-growing-tree-algorithm.html)*

use crate::prelude::*;
use rand::prelude::*;
use rand_chacha::ChaChaRng;

/// Different ways in which the next root cell is selected from the stack of possibilities
#[derive(Debug, Clone, Copy)]
pub enum GrowingTreeSelectionMethod {
    /// A random cell. Equivalent to Prim's algorithm.
    Random,
    /// The cell most recently added to the stack. Equivalent to Recursive backtracker algorithm.
    MostRecent,
    /// The first cell on the stack.
    First,
}

/// [`Generator`] implementation which uses the recursive-backtracking algorithm.
#[derive(Debug, Clone)]
pub struct GrowingTreeGenerator {
    rng: ChaChaRng,
    /// The method by which to select the next candidate cell from the available possibilities
    pub selection_method: GrowingTreeSelectionMethod,
    cell_stack: Vec<Coordinates>,
    visited: Vec<Coordinates>,
    neighbours: Vec<Coordinates>,
}

impl GrowingTreeGenerator {
    /// Create a new instance.
    ///
    /// Optionally a 32 bit seed can be provided to seed the internal random generator.
    /// Giving a seed results in identical mazes being generated which omitting it sources the
    /// random generator from entropy.
    pub fn new(seed: Option<[u8; 32]>) -> GrowingTreeGenerator {
        GrowingTreeGenerator {
            rng: match seed {
                None => ChaChaRng::from_entropy(),
                Some(seed) => ChaChaRng::from_seed(seed),
            },
            selection_method: GrowingTreeSelectionMethod::First,
            cell_stack: Vec::new(),
            visited: Vec::new(),
            neighbours: Vec::new(),
        }
    }

    /// Core algorithm implementation
    ///
    ///
    /// Returns coordinates of the goal field
    fn carve_passages_from(
        &mut self,
        maze: &mut Maze,
        start_coordinates: Coordinates,
    ) -> Coordinates {
        let mut current_coordinates = start_coordinates;
        let mut goal_coordinates = current_coordinates;
        let mut max_q = 0;

        self.cell_stack.clear();
        self.cell_stack.push(current_coordinates);
        self.visited.push(current_coordinates); // Mark it as visited

        while !self.cell_stack.is_empty() {
            self.find_unvisited_neighbours(maze, current_coordinates);

            if self.neighbours.is_empty() {
                // We've reached a dead end - remove the current_coordinates from the stack
                if self.cell_stack.contains(&current_coordinates) {
                    let idx = self
                        .cell_stack
                        .iter()
                        .position(|&r| r == current_coordinates)
                        .unwrap();
                    self.cell_stack.remove(idx);
                }

                // If there are no more cells, quit
                if self.cell_stack.is_empty() {
                    continue;
                }

                // And now select a new current cell according to 'selectionmethod' parameter
                // pop and remove wont fail because we just tested for non-zero length
                current_coordinates = match self.selection_method {
                    GrowingTreeSelectionMethod::MostRecent => self.cell_stack.pop().unwrap(),
                    GrowingTreeSelectionMethod::Random => {
                        self.cell_stack[self.rng.gen_range(0, self.cell_stack.len())]
                    }
                    GrowingTreeSelectionMethod::First => self.cell_stack.remove(0),
                };
            } else {
                // We have some neighbours so we can make a passage

                // Choose a random neighbouring cell and move to it.
                let next_coords = self.neighbours[self.rng.gen_range(0, self.neighbours.len())];
                maze.graph.add_edge(current_coordinates, next_coords, ()); // Knock down the wall between them
                self.cell_stack.push(next_coords);
                current_coordinates = next_coords;
                self.visited.push(current_coordinates); // Mark the new cell as visited

                // Keep track of the longest cell stack. Our target is at the end of this stack - the neighbour to which we just connected
                if self.cell_stack.len() > max_q {
                    max_q = self.cell_stack.len();
                    goal_coordinates = current_coordinates;
                }
            }
        }
        goal_coordinates
    }

    /// Find the neighbours of this cell that have NOT been visited
    fn find_unvisited_neighbours(&mut self, maze: &mut Maze, current_coordinates: Coordinates) {
        self.neighbours.clear(); // Clear the current neighbour list

        // Look all around, add any UNvisited neighbours to the list
        for i_dir in Direction::all().iter() {
            let next_coords = current_coordinates.next(i_dir);
            if maze.are_coordinates_inside(&next_coords) && !self.visited.contains(&next_coords) {
                self.neighbours.push(next_coords);
            }
        }
    }
}

impl Generator for GrowingTreeGenerator {
    fn generate(&mut self, width: i32, height: i32) -> Maze {
        let start = (0, 0).into();
        let mut maze = Maze::new(width, height, start, (0, 0).into());

        maze.goal = self.carve_passages_from(&mut maze, start);

        maze
    }
}

#[cfg(test)]
mod test {
    test_all_coordinates_have_fields!(super::GrowingTreeGenerator);
    test_route_from_start_to_goal_exists!(super::GrowingTreeGenerator);
    test_all_fields_connected!(super::GrowingTreeGenerator);
    test_generation_is_deterministic!(super::GrowingTreeGenerator);
}
