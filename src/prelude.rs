use rand::prelude::*;
use rand::Rng;

pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }

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

pub type Coordinates = (i32, i32);

pub trait Grid
where
    Self::FieldType: Field,
{
    type FieldType;

    fn get_size(&self) -> &(i32, i32);

    fn get_start(&self) -> &Coordinates;

    fn get_goal(&self) -> &Coordinates;

    fn get_field(&self, coordinates: Coordinates) -> Result<&Self::FieldType, ()>;

    fn are_coordinates_inside(&self, coordinates: Coordinates) -> bool {
        coordinates.0 >= 0
            && coordinates.0 < self.get_size().0
            && coordinates.1 >= 0
            && coordinates.1 < self.get_size().1
    }
}

#[macro_export]
macro_rules! impl_grid_debug {
    ($t:ty) => {
        impl std::fmt::Debug for $t {
            fn fmt(
                &self,
                f: &mut core::fmt::Formatter<'_>,
            ) -> core::result::Result<(), std::fmt::Error> {
                for iy in 0..self.get_size().1 {
                    // print top passage
                    for ix in 0..self.get_size().0 {
                        f.write_str("·")?;
                        if self
                            .get_field((ix, iy))
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
                        let field = self.get_field((ix, iy)).unwrap();
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

                core::result::Result::Ok(())
            }
        }
    };
}

pub trait Field {
    fn has_wall(&self, direction: &Direction) -> bool;

    fn has_passage(&self, direction: &Direction) -> bool {
        !self.has_wall(direction)
    }
}

pub trait Generator
where
    Self::GridType: Grid,
{
    type GridType;

    fn generate(&mut self, width: i32, height: i32) -> Self::GridType;
}
