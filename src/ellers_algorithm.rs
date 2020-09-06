//! Ellers algorithm implementation
//!
//! Fast algorithm for generating arbitrarily large mazes in linear time.
//!
//! # Algorithm rundown
//! 1. Initialize the fields of the first row to each exist in its own set.
//! 2. Randomly join fields but only if they are not already in the same set.
//!     When joining, merge the two sets (which indicates that the cells are now connected)
//! 3. For each set, randomly create vertical connections downward to the next row.
//!     Each set must have at least one vertical connection created in this way.
//!     The cells in the next row share the same set because they are connected.
//! 4. Flesh out the next row by creating sets for the fields not already vertically connected.
//! 5. Repeat from *2.* until the last row is reached
//! 6. For the last row, join all adjacent cells which do not yet share a set.
//!
//! ## Explanation by example
//! If the above explanation seems a bit complex, here's an example for a 4x5 maze:
//!
//! 1. First, we initialize each field in the row to be in its own set (represented by numbers):
//!     
//!     ```text
//!     ·-·-·-·-·-·
//!     |1|2|3|4|5|
//!     ·-·-·-·-·-·
//!     ```
//!
//! 2. Next, we randomly join adjacent fields that belong to different sets.
//!     The fields so joined also are merged into the same set:
//!
//!     ```text
//!     ·-·-·-·-·-·
//!     |1 1 1|4 4|
//!     ·-·-·-·-·-·
//!     ```
//!
//!  3. Now we randomly determine the vertical connections, at least one per set.
//!     The fields in the next row that we connected to must be assigned to the set of the cell above them:
//!
//!     ```text
//!     ·-·-·-·-·-·
//!     |1 1 1|4 4|
//!     · ·-· ·-· ·
//!     |1| |1| |4|
//!     ·-·-·-·-·-·
//!     ```
//!
//!  4. Next, we flesh out the next row, assigning each remaining field to its own set:
//!
//!     ```text
//!     ·-·-·-·-·-·
//!     |1 1 1|4 4|
//!     · ·-· ·-· ·
//!     |1|6|1|7|4|
//!     ·-·-·-·-·-·
//!     ```
//!
//! 5. Now, we just repeat the previous steps on our new row.
//!     We randomly connect adjacent sets that do not share a set. Something like this:
//!
//!     ```text
//!     ·-·-·-·-·-·
//!     |1 1 1|4 4|
//!     · ·-· ·-· ·
//!     |1 1|1 1|4|
//!     ·-·-·-·-·-·
//!     ```
//!
//! 6. Add at east one vertical connection to each set:
//!
//!     ```text
//!     ·-·-·-·-·-·
//!     |1 1 1|4 4|
//!     · ·-· ·-· ·
//!     |1 1|1 1|4|
//!     ·-· ·-·-· ·
//!     | |1| | |4|
//!     ·-·-·-·-·-·
//!     ```
//!
//! 7. Flesh out the next row (I'm reusing extinct set numbers for simplicity):
//!
//!     ```text
//!     ·-·-·-·-·-·
//!     |1 1 1|4 4|
//!     · ·-· ·-· ·
//!     |1 1|1 1|4|
//!     ·-· ·-·-· ·
//!     |8|1|9|2|4|
//!     ·-·-·-·-·-·
//!     ```
//!
//!  8. Final iteration for the last row now. First, randomly join adjacent cells:
//!
//!     ```text
//!     ·-·-·-·-·-·
//!     |1 1 1|4 4|
//!     · ·-· ·-· ·
//!     |1 1|1 1|4|
//!     ·-· ·-·-· ·
//!     |8|1|4 4 4|
//!     ·-·-·-·-·-·
//!     ```
//!
//! 9. Add vertical connections (at least one per set):
//!
//!     ```text
//!     ·-·-·-·-·-·
//!     |1 1 1|4 4|
//!     · ·-· ·-· ·
//!     |1 1|1 1|4|
//!     ·-· ·-·-· ·
//!     |8|1|4 4 4|
//!     · · ·-· ·-·
//!     |8|1| |4| |
//!     ·-·-·-·-·-·
//!     ```
//!
//! 10. Flesh out the next row:
//!
//!     ```text
//!     ·-·-·-·-·-·
//!     |1 1 1|4 4|
//!     · ·-· ·-· ·
//!     |1 1|1 1|4|
//!     ·-· ·-·-· ·
//!     |8|1|4 4 4|
//!     · · ·-· ·-·
//!     |8|1|3|4|5|
//!     ·-·-·-·-·-·
//!     ```
//!
//! 11. And now the final step.
//!     This time, we must connect ALL adjacent (but disjoint) fields.
//!     In this case, that means all of them:
//!
//!     ```text
//!     ·-·-·-·-·-·
//!     |     |   |
//!     · ·-· ·-· ·
//!     |   |   | |
//!     ·-· ·-·-· ·
//!     | | |     |
//!     · · ·-· ·-·
//!     |         |
//!     ·-·-·-·-·-·
//!     ```
//!
//! *Explanation and example credits to
//! [Jamis Buck's Buckblog](http://weblog.jamisbuck.org/2010/12/29/maze-generation-eller-s-algorithm.html)*
//!

use crate::prelude::*;
use rand::prelude::*;
use rand_chacha::ChaChaRng;
use std::collections::BTreeSet;

const HORIZONTAL_JOIN_CHANCE: f64 = 0.5;

type EllersSet = BTreeSet<Coordinates>;

/// [`Generator`] implementation which uses Ellers algorithm.
#[derive(Debug, Clone)]
pub struct EllersGenerator {
    rng: ChaChaRng,
    sets: Vec<EllersSet>,
    graph: MazeGraph,
}

impl EllersGenerator {
    /// Create a new instance.
    ///
    /// Optionally a 32 bit seed can be provided to seed the internal random generator.
    /// Giving a seed results in identical mazes being generated while omitting it sources the
    /// random generator from entropy.
    pub fn new(seed: Option<[u8; 32]>) -> Self {
        EllersGenerator {
            rng: match seed {
                None => ChaChaRng::from_entropy(),
                Some(seed) => ChaChaRng::from_seed(seed),
            },
            sets: Vec::new(),
            graph: MazeGraph::new(),
        }
    }

    /// Join the containing sets of two given fields.
    ///
    /// Only changes anything if the two fields are not already in the same set.
    /// Also keeps track of the connection in `self.graph`.
    fn join_sets_of_fields(&mut self, field1: Coordinates, field2: Coordinates) {
        let set1 = self
            .sets
            .iter()
            .find(|set| set.contains(&field1))
            .expect(&format!("Expected to find coordinates {}", field1));
        let set2 = self
            .sets
            .iter()
            .find(|set| set.contains(&field2))
            .expect(&format!("Expected to find coordinates {}", field2));

        if set1 != set2 {
            let index1 = self.sets.iter().position(|set| set == set1).unwrap();
            let index2 = self.sets.iter().position(|set| set == set2).unwrap();

            self.sets[index1] = set1.union(set2).cloned().collect();
            self.sets[index2] = EllersSet::new();
            self.graph.add_edge(field1, field2, ());
        }
    }

    /// Initialize the fields of the first row to each exist in its own set.
    fn init_fields_first_row(&mut self, width: i32) {
        self.sets = (0..width)
            .map(|x| {
                let mut set = EllersSet::new();
                set.insert(Coordinates::new(x, 0));
                set
            })
            .collect();
    }

    /// Randomly join fields but only if they are not already in the same set.
    /// When joining, merge the two sets (which indicates that the cells are now connected)
    fn randomly_join_fields(&mut self, current_y: i32) {
        // iterate over all fields and randomly join them with the field on the right
        for i_x in 0..(self.sets.len() - 1) {
            if self.rng.gen_bool(HORIZONTAL_JOIN_CHANCE) {
                self.join_sets_of_fields(
                    (i_x as i32, current_y).into(),
                    (i_x as i32 + 1, current_y).into(),
                );
            }
        }
    }

    /// For each set, randomly create vertical connections downward to the next row.
    /// Each set must have at least one vertical connection created in this way.
    /// The cells in the next row share the same set because they are connected.
    fn create_downward_connections(&mut self, current_y: i32) {
        for i_set in &mut self.sets {
            if !i_set.is_empty() {
                // filter to those fields which are located on the bottom-most row
                let bottom_most_fields: Vec<_> =
                    i_set.iter().filter(|c| c.y == current_y).cloned().collect();

                // how many downward connections should be added
                let count = if bottom_most_fields.len() >= 1 {
                    1
                } else {
                    self.rng.gen_range(1, bottom_most_fields.len())
                };

                for coordinates in bottom_most_fields.choose_multiple(&mut self.rng, count) {
                    let next_coords = coordinates.next(&Direction::South);
                    i_set.insert(next_coords);
                    self.graph.add_edge(*coordinates, next_coords, ());
                }
            }
        }
    }

    /// Flesh out the next row by creating sets for the fields not already vertically connected.
    fn flesh_out_next_row(&mut self, current_y: i32) {
        let next_y = current_y + 1;
        // add the coordinate to its set if not already present
        for i_x in 0..self.sets.len() {
            let coordinates = (i_x as i32, next_y).into();

            if self
                .sets
                .iter()
                .find(|set| set.contains(&coordinates))
                .is_none()
            {
                self.sets
                    .iter_mut()
                    .find(|set| set.is_empty())
                    .expect("no empty set found")
                    .insert(coordinates);
                //self.sets[i_x].insert(coordinates);
            }
        }
    }

    /// For the last row, join all adjacent cells which do not yet share a set.
    fn join_last_rows(&mut self, current_y: i32) {
        for i_x in 0..(self.sets.len() - 1) {
            self.join_sets_of_fields(
                (i_x as i32, current_y).into(),
                (i_x as i32 + 1, current_y).into(),
            );
        }
    }
}

impl Generator for EllersGenerator {
    fn generate(&mut self, width: i32, height: i32) -> Maze {
        self.graph = MazeGraph::with_capacity((width * height) as usize, 0);

        self.init_fields_first_row(width);
        for y in 0..height {
            self.randomly_join_fields(y);
            self.create_downward_connections(y);
            self.flesh_out_next_row(y);
        }
        self.join_last_rows(height - 1);

        // convert hashset representation to final maze
        let start = (0, 0).into();
        let goal = (0, 0).into();
        let mut maze = Maze::new(width, height, start, goal);
        maze.graph = self.graph.clone();

        maze
    }
}

#[cfg(test)]
mod test {
    test_all_coordinates_have_fields!(super::EllersGenerator);
    test_route_from_start_to_goal_exists!(super::EllersGenerator);
    test_all_fields_connected!(super::EllersGenerator);
}
