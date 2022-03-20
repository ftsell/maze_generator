//! Prim's algorithm implementation
//!
//! Prim’s approaches the problem from a different angle. Rather than working
//! edgewise across the entire graph, it starts at one point, and grows outward
//! from that point. The standard version of the algorithm works something like
//! this:
//!
//! 1. Choose an arbitrary vertex from G (the graph), and add it to some (initially empty) set V.
//! 2. Choose the edge with the smallest weight from G, that connects a vertex in V with another vertex not in V.
//! 3. Add that edge to the minimal spanning tree, and the edge’s other vertex to V.
//! 4. Repeat steps 2 and 3 until V includes every vertex in G.
//!
//! And the result is a minimal spanning tree of G. Straightforward enough!
//! With one little change, it becomes a suitable method for generating mazes:
//! just change step 2 so that instead of selecting the edge with the smallest
//! weight, you select an edge at random, as long as it bridges the so-called
//! “frontier” of the maze (the set of edges that move from within the maze, to
//! a cell that is outside the maze).
//!
//! *Explanation and credits to
//! [Jamis Buck's Buckblog](http://weblog.jamisbuck.org/2011/1/10/maze-generation-prim-s-algorithm.html)*

use std::collections::{HashSet, VecDeque};

use anyhow::{Context, Result};
use rand::prelude::*;
use rand_chacha::ChaChaRng;

use crate::prelude::*;

/// [`Generator`] implementation which uses the recursive-backtracking algorithm.
#[derive(Debug, Clone)]
pub struct PrimsGenerator {
    rng: ChaChaRng,
    frontier: Vec<Coordinates>,
    visited: Vec<Coordinates>,
    neighbours: Vec<Coordinates>,
}

impl PrimsGenerator {
    /// Create a new instance.
    ///
    /// Optionally a 32 bit seed can be provided to seed the internal random generator.
    /// Giving a seed results in identical mazes being generated which omitting it sources the
    /// random generator from entropy.
    pub fn new(seed: Option<[u8; 32]>) -> PrimsGenerator {
        PrimsGenerator {
            rng: match seed {
                None => ChaChaRng::from_entropy(),
                Some(seed) => ChaChaRng::from_seed(seed),
            },
            frontier: Vec::new(),
            visited: Vec::new(),
            neighbours: Vec::new(),
        }
    }

    /// Core algorithm implementation
    ///
    /// Carves passages in all directions in random order from the current coordinates but only
    /// if the field in that direction has not yet been processed.
    ///
    /// Returns coordinates of the goal field
    fn carve_passages_from(
        &mut self,
        maze: &mut Maze,
        current_coordinates: Coordinates,
    ) -> Result<()> {
        // Mark our starting cell as 'in' and find its frontier
        self.mark_cell(maze, current_coordinates)
            .with_context(|| String::from("Could not parse passages"))?;

        while !self.frontier.is_empty() {
            // Choose a random frontier cell
            let next_coords = self.frontier[self.rng.gen_range(0, self.frontier.len())];

            // Choose a random 'in' neighbour of that cell
            self.find_visited_neighbours(maze, next_coords);
            if !self.neighbours.is_empty() {
                let ncell = self.neighbours[self.rng.gen_range(0, self.neighbours.len())]; // neighbours is  aways non-zero length
                maze.graph.add_edge(next_coords, ncell, ()); // Knock down the wall between them
                self.mark_cell(maze, next_coords)
                    .with_context(|| "Could not parse passages")?; // frontier cell is now 'in'
            } else {
                // No neighbours - panic
                self.frontier.clear(); // Will cause a non-panic return but the maze will be incomplete
                eprintln!("No neighbours! {:?}", next_coords);
            }
        }

        Ok(())
    }

    /// Mark a cell as visited and it's unvisited neighbours as frontier cells
    fn mark_cell(&mut self, maze: &mut Maze, current_coordinates: Coordinates) -> Result<()> {
        // Mark the current cell as visited
        if !self.visited.contains(&current_coordinates) {
            self.visited.push(current_coordinates);
        }

        // Mark the current cell as not part of the frontier
        if self.frontier.contains(&current_coordinates) {
            let idx = self
                .frontier
                .iter()
                .position(|&r| r == current_coordinates)
                .ok_or_else(|| {
                    GenericGeneratorError::InternalError(String::from(
                        "Could not find coordinates in frontier list",
                    ))
                })
                .with_context(|| "Could not mark cell")?;
            self.frontier.remove(idx);
        }

        // Add any unvisited neighbours to the frontier
        for i_dir in Direction::all().iter() {
            let next_coords = current_coordinates.next(i_dir);
            if maze.are_coordinates_inside(&next_coords)
                && !self.frontier.contains(&next_coords)
                && !self.visited.contains(&next_coords)
            {
                self.frontier.push(next_coords);
            }
        }

        Ok(())
    }

    /// Find the neighbours of this cell that have been visited
    fn find_visited_neighbours(&mut self, maze: &mut Maze, current_coordinates: Coordinates) {
        self.neighbours.clear(); // Clear the current neighbour list

        // Look all around, add any visited neighbours to the list
        for i_dir in Direction::all().iter() {
            let next_coords = current_coordinates.next(i_dir);
            if maze.are_coordinates_inside(&next_coords) && self.visited.contains(&next_coords) {
                self.neighbours.push(next_coords);
            }
        }
    }

    /// Do breadth-first search for the field which has the most distance
    // Cloned from ellers_algorithm, but passing in maze rather than using the self.graph element (which we don't have here)
    fn find_suitable_goal(&self, maze: &mut Maze, start: Coordinates) -> Coordinates {
        let mut already_visited = HashSet::new();
        let mut queue: VecDeque<Coordinates> = maze.graph.neighbors(start).collect();
        let mut last_coords = start;

        while let Some(i_coords) = queue.pop_front() {
            queue.extend(
                maze.graph
                    .neighbors(i_coords)
                    .filter(|c| !already_visited.contains(c)),
            );
            already_visited.insert(i_coords);
            last_coords = i_coords;
        }

        last_coords
    }
}

impl Generator for PrimsGenerator {
    fn generate(&mut self, width: i32, height: i32) -> Result<Maze> {
        let start = (0, 0).into();
        let mut maze = Maze::new(width, height, start, (0, 0).into());

        self.carve_passages_from(&mut maze, start)
            .with_context(|| "Could not generate maze")?;
        maze.goal = self.find_suitable_goal(&mut maze, start);

        Ok(maze)
    }
}

#[cfg(test)]
mod test {
    test_all_coordinates_have_fields!(super::PrimsGenerator);
    test_route_from_start_to_goal_exists!(super::PrimsGenerator);
    test_all_fields_connected!(super::PrimsGenerator);
    test_generation_is_deterministic!(super::PrimsGenerator);
}
