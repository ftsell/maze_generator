use crate::prelude::*;
use rand::prelude::*;
use rand_chacha::ChaChaRng;
use std::mem;

// ----------------------------------------
// Field
// ----------------------------------------

bitflags! {
    pub struct PassageFlags: u8 {
        const NORTH = 0b00000001;
        const EAST = 0b00000010;
        const SOUTH = 0b00000100;
        const WEST = 0b00001000;
        const NONE = 0;
    }
}

impl From<&Direction> for PassageFlags {
    fn from(d: &Direction) -> Self {
        match d {
            Direction::North => PassageFlags::NORTH,
            Direction::East => PassageFlags::EAST,
            Direction::South => PassageFlags::SOUTH,
            Direction::West => PassageFlags::WEST,
        }
    }
}

#[derive(Clone)]
pub struct RbField {
    passages: PassageFlags,
}

impl RbField {
    fn add_passage(&mut self, direction: &Direction) {
        self.passages = self.passages | direction.into()
    }

    fn is_untouched(&self) -> bool {
        self.passages.is_empty()
    }
}

impl Field for RbField {
    fn has_wall(&self, direction: &Direction) -> bool {
        !self.has_passage(direction)
    }

    fn has_passage(&self, direction: &Direction) -> bool {
        let direction_flag = direction.into();
        self.passages & direction_flag == direction_flag
    }
}

// ----------------------------------------
// Grid
// ----------------------------------------

#[derive(Clone)]
pub struct RbGrid {
    size: (i32, i32),
    start: Coordinates,
    goal: Coordinates,
    data: Vec<RbField>,
}

impl RbGrid {
    fn set_field(&mut self, coordinates: Coordinates, field: RbField) {
        if self.are_coordinates_inside(coordinates) {
            let i = self.coords2index(coordinates) as usize;
            let _ = mem::replace(&mut self.data[i], field);
        } else {
            panic!(format!("Cannot set field at {:?} because it is outside grid.", coordinates));
        }
    }

    fn _index2coords(&self, i: i32) -> Coordinates {
        let x = i % self.get_size().0;
        let y = (i - x) / self.get_size().0;
        (x, y)
    }

    fn coords2index(&self, coords: Coordinates) -> i32 {
        (coords.1 * self.get_size().0) + coords.0
    }
}

impl Grid for RbGrid {
    type FieldType = RbField;

    fn get_size(&self) -> &(i32, i32) {
        &self.size
    }

    fn get_start(&self) -> &(i32, i32) {
        &self.start
    }

    fn get_goal(&self) -> &(i32, i32) {
        &self.goal
    }

    fn get_field(&self, coordinates: Coordinates) -> Result<&Self::FieldType, ()> {
        if self.are_coordinates_inside(coordinates) {
            Ok(self.data.get(self.coords2index(coordinates) as usize).unwrap())
        } else {
            Err(())
        }
    }
}

impl_grid_debug!(RbGrid);


// ----------------------------------------
// Generator
// ----------------------------------------

pub struct RbGenerator {
    rng: ChaChaRng,
}

impl RbGenerator {
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
            let next_x = coordinates.0 + match i_dir {
                Direction::East => 1,
                Direction::West => -1,
                _ => 0,
            };
            let next_y = coordinates.1 + match i_dir {
                Direction::North => -1,
                Direction::South => 1,
                _ => 0,
            };

            match grid.get_field((next_x, next_y)) {
                Ok(next_field) => {
                    if next_field.is_untouched() {
                        // set passage on next field
                        let mut new_field = (*next_field).clone();
                        new_field.add_passage(&i_dir.opposite());
                        grid.set_field((next_x, next_y), new_field);

                        // set passage on original field
                        new_field = (*grid.get_field(coordinates).unwrap()).clone();
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
                grid.data.push(RbField {
                    passages: PassageFlags::NONE,
                });
            }
        }

        self.carve_passages_from((0, 0), &mut grid);
        grid
    }
}
