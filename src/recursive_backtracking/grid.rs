use crate::prelude::*;
use crate::recursive_backtracking::RbField;
use std::mem;

#[derive(Clone)]
pub struct RbGrid {
    pub(super) size: (i32, i32),
    pub(super) start: Coordinates,
    pub(super) goal: Coordinates,
    pub(super) data: Vec<RbField>,
}

impl RbGrid {
    pub(super) fn set_field(&mut self, coordinates: Coordinates, field: RbField) {
        if self.are_coordinates_inside(coordinates) {
            let i = self.coords2index(coordinates) as usize;
            let _ = mem::replace(&mut self.data[i], field);
        } else {
            panic!(format!(
                "Cannot set field at {:?} because it is outside grid.",
                coordinates
            ));
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
            Ok(self
                .data
                .get(self.coords2index(coordinates) as usize)
                .unwrap())
        } else {
            Err(())
        }
    }
}

impl_grid_debug!(RbGrid);
