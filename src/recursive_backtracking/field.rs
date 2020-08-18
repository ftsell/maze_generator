use crate::prelude::*;
use bitflags::_core::fmt::Formatter;

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

#[derive(Clone, Copy)]
pub struct RbField {
    pub(super) passages: PassageFlags,
}

impl RbField {
    pub(super) fn add_passage(&mut self, direction: &Direction) {
        self.passages = self.passages | direction.into()
    }

    pub(super) fn is_untouched(&self) -> bool {
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

impl_field_debug!(RbField);
