use crate::prelude::*;
use crate::recursive_backtracking::RbField;
use crate::util::*;
use std::mem;

/// [`Grid`] implementation used by the recursive-backtracking algorithm
#[derive(Clone)]
pub struct RbGrid {
    pub(super) size: (i32, i32),
    pub(super) start: Coordinates,
    pub(super) goal: Coordinates,
    pub(super) data: Vec<RbField>,
}

impl RbGrid {
    pub(super) fn set_field(&mut self, coordinates: Coordinates, field: RbField) {
        if are_coordinates_inside(self, &coordinates) {
            let i = coords2index(self, &coordinates) as usize;
            let _ = mem::replace(&mut self.data[i], field);
        } else {
            panic!(format!(
                "Cannot set field at {:?} because it is outside grid.",
                coordinates
            ));
        }
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

    fn get_field(&self, coordinates: &Coordinates) -> Result<&Self::FieldType, ()> {
        if are_coordinates_inside(self, coordinates) {
            Ok(self
                .data
                .get(coords2index(self, coordinates) as usize)
                .unwrap())
        } else {
            Err(())
        }
    }
}

impl_grid_debug!(RbGrid);
