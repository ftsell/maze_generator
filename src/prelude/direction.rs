use rand::prelude::*;

/// The four cardinal directions
///
/// Also defines convenience functions to work with them.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Direction {
    /// *North* or *up* direction
    North,
    /// *East* or *right* direction
    East,
    /// *South* or *down* direction
    South,
    /// *West* or *left* direction
    West,
}

impl Direction {
    /// Return the opposite direction of self
    pub fn opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }

    /// Generate a list of all collections but in random order
    pub fn gen_random_order(rng: &mut impl Rng) -> [Direction; 4] {
        let mut directions = Self::all();
        directions.shuffle(rng);
        directions
    }

    /// Return all directions as array
    pub fn all() -> [Direction; 4] {
        [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ]
    }
}
