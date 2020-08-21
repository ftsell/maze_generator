//! Common traits and members
use rand::prelude::*;
use rand::Rng;

/// The four cardinal directions
///
/// Also defines convenience functions to work with them.
#[derive(Debug, Copy, Clone)]
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
    pub fn gen_random_order<T>(rng: &mut T) -> [Direction; 4]
    where
        T: Rng,
    {
        let mut directions = [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ];
        directions.shuffle(rng);
        directions
    }
}

/// Two-Dimensional coordinates used for addressing fields in a maze.
///
/// The first field represents the X coordinate while the seconds represents the Y coordinate.
pub type Coordinates = (i32, i32);

/// Generic Field Api which are ordered in a [`Grid`]
///
/// The Api defines functions for querying walls and passages in certain directions which should
/// always return the opposite of each other.
pub trait Field {
    /// Whether or not this field has a wall in the specified direction
    fn has_wall(&self, direction: &Direction) -> bool;

    /// Whether or not this field has a passage in the specified direction
    fn has_passage(&self, direction: &Direction) -> bool;
}

/// Implements [`std::fmt::Debug`] trait for the specified type. The type must also implement [`Field`]
macro_rules! impl_field_debug {
    ($t:ty) => {
        impl std::fmt::Debug for $t {
            fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
                f.debug_struct(stringify!($t))
                    .field(
                        "north",
                        &if self.has_wall(&Direction::North) {
                            String::from("wall")
                        } else {
                            String::from("passage")
                        },
                    )
                    .field(
                        "east",
                        &if self.has_wall(&Direction::East) {
                            String::from("wall")
                        } else {
                            String::from("passage")
                        },
                    )
                    .field(
                        "south",
                        &if self.has_wall(&Direction::South) {
                            String::from("wall")
                        } else {
                            String::from("passage")
                        },
                    )
                    .field(
                        "west",
                        &if self.has_wall(&Direction::West) {
                            String::from("wall")
                        } else {
                            String::from("passage")
                        },
                    )
                    .finish()
            }
        }
    };
}

/// Generic Grid Api which is used to access the generated maze
///
/// Each grid implementation needs to define a *size*, *start-point* and *goal-point* in addition
/// to the actual [`Field`]s.
pub trait Grid
where
    Self::FieldType: Field,
{
    /// The [`Field`] implementation stored in this grid
    type FieldType;

    /// How large this maze is in (width, height) format
    fn get_size(&self) -> &(i32, i32);

    /// Where the starting point is
    fn get_start(&self) -> &Coordinates;

    /// Where the goal point is
    fn get_goal(&self) -> &Coordinates;

    /// Returns a reference to the [`Field`] addressed by the given coordinates
    ///
    /// Returns an [`Err`] variant when the coordinates are not inside the grid and [`Ok`] otherwise.
    fn get_field(&self, coordinates: &Coordinates) -> Result<&Self::FieldType, ()>;
}

/// Implements [`std::fmt::Debug`] trait for the specified type. The type must also implement [`Grid`]
macro_rules! impl_grid_debug {
    ($t:ty) => {
        impl std::fmt::Debug for $t {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                for iy in 0..self.get_size().1 {
                    // print top passage
                    for ix in 0..self.get_size().0 {
                        f.write_str("·")?;
                        if self
                            .get_field(&(ix, iy))
                            .unwrap()
                            .has_passage(&Direction::North)
                        {
                            f.write_str(" ")?;
                        } else {
                            f.write_str("-")?;
                        }
                    }
                    f.write_str("·\n")?;

                    // print left passage and room icon
                    for ix in 0..self.get_size().0 {
                        let field = self.get_field(&(ix, iy)).unwrap();
                        if field.has_passage(&Direction::West) {
                            f.write_str(" ")?;
                        } else {
                            f.write_str("|")?;
                        }

                        if self.start == (ix, iy) {
                            f.write_str("S")?;
                        } else if self.goal == (ix, iy) {
                            f.write_str("G")?;
                        } else {
                            f.write_str(" ")?;
                        }
                    }
                    f.write_str("|\n")?;

                    // print bottom line
                    if iy == self.get_size().1 - 1 {
                        for _ix in 0..self.get_size().0 {
                            f.write_str("·-")?;
                        }
                        f.write_str("·\n")?;
                    }
                }

                Ok(())
            }
        }
    };
}

/// Generator Api implemented by all algorithms to generate a maze
pub trait Generator
where
    Self::GridType: Grid,
{
    /// The [`Grid`] implementation returned by this generator
    type GridType;

    /// Key function to generate a maze
    ///
    /// The returned [`Grid`] will have the provided width and height. It can be any rectangular
    /// shape.
    fn generate(&mut self, width: i32, height: i32) -> Self::GridType;
}
