use crate::prelude::*;

/// Two-Dimensional coordinates used for addressing fields in a maze.
#[derive(Debug, Copy, Clone, Default, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Coordinates {
    /// X component
    pub x: i32,
    /// Y component
    pub y: i32,
}

impl Coordinates {
    pub(crate) fn next(&self, direction: &Direction) -> Self {
        Self {
            x: self.x
                + match direction {
                    Direction::East => 1,
                    Direction::West => -1,
                    _ => 0,
                },
            y: self.y
                + match direction {
                    Direction::North => -1,
                    Direction::South => 1,
                    _ => 0,
                },
        }
    }
}

impl Into<(i32, i32)> for Coordinates {
    fn into(self) -> (i32, i32) {
        (self.x, self.y)
    }
}

impl From<(i32, i32)> for Coordinates {
    fn from(source: (i32, i32)) -> Self {
        Self {
            x: source.0,
            y: source.1,
        }
    }
}
