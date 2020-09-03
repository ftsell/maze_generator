use super::*;

/// Defines the possible types of fields that exist in a maze
#[derive(Debug, Copy, Clone)]
pub enum FieldType {
    /// Start field from which a potential user should start exploring
    Start,
    /// Goal field which a potential user tries to reach
    Goal,
    /// Standard field with no special meaning
    Normal,
}

/// Representation of a single field.
///
/// This type is not used in internal representation but provides a nicer API to work with fields
/// than always querying the [`Maze`].
#[derive(Clone)]
pub struct Field {
    passages: Vec<Direction>,
    /// Role which this field position serves in the maze
    pub field_type: FieldType,
    /// Where this field is located in the maze
    pub coordinates: Coordinates,
}

impl Field {
    pub(crate) fn new(
        field_type: FieldType,
        coordinates: Coordinates,
        passages: Vec<Direction>,
    ) -> Self {
        Field {
            passages,
            field_type,
            coordinates,
        }
    }

    /// Whether or not a passage (a way) exists from this field to another one which lies in the
    /// specified direction.
    pub fn has_passage(&self, direction: &Direction) -> bool {
        self.passages.contains(direction)
    }
}

impl std::fmt::Debug for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(Field))
            .field(
                "north",
                &if !self.has_passage(&Direction::North) {
                    String::from("wall")
                } else {
                    String::from("passage")
                },
            )
            .field(
                "east",
                &if !self.has_passage(&Direction::East) {
                    String::from("wall")
                } else {
                    String::from("passage")
                },
            )
            .field(
                "south",
                &if !self.has_passage(&Direction::South) {
                    String::from("wall")
                } else {
                    String::from("passage")
                },
            )
            .field(
                "west",
                &if !self.has_passage(&Direction::West) {
                    String::from("wall")
                } else {
                    String::from("passage")
                },
            )
            .finish()
    }
}
