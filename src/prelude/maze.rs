use crate::prelude::*;
use petgraph::algo::is_isomorphic;
use petgraph::graphmap::GraphMap;
use petgraph::stable_graph::DefaultIx;
use petgraph::Undirected;

pub(crate) type MazeGraph = GraphMap<Coordinates, (), Undirected>;

/// A collection of [`Field`]s with passages between them.
///
/// Use one of the provided [`Generator`]s to create an instance of this type.
#[derive(Clone)]
pub struct Maze {
    pub(crate) graph: MazeGraph,
    /// At which coordinates the start field lies
    pub start: Coordinates,
    /// At which coordinates the goal field lies
    pub goal: Coordinates,
    /// How large the maze is in (width, height) format
    pub size: (i32, i32),
}

impl Maze {
    pub(crate) fn new(width: i32, height: i32, start: Coordinates, goal: Coordinates) -> Self {
        debug_assert!(width > 0, "maze width should be >0");
        debug_assert!(height > 0, "maze height should be >0");

        Maze {
            graph: GraphMap::with_capacity((width * height) as usize, 0),
            size: (width, height),
            start,
            goal,
        }
    }

    /// Retrieve the [`Field`] which is located at `coordinates`
    pub fn get_field(&self, coordinates: &Coordinates) -> Option<Field> {
        if self.are_coordinates_inside(coordinates) {
            // figure out in which directions passages exist
            let passages: Vec<_> = Direction::all()
                .iter()
                .filter(|dir| {
                    self.graph
                        .contains_edge(coordinates.clone(), coordinates.next(dir))
                })
                .map(|dir| dir.clone())
                .collect();

            let field_type = if &self.start == coordinates {
                FieldType::Start
            } else if &self.goal == coordinates {
                FieldType::Goal
            } else {
                FieldType::Normal
            };

            Some(Field::new(field_type, coordinates.clone(), passages))
        } else {
            None
        }
    }

    pub(crate) fn are_coordinates_inside(&self, coordinates: &Coordinates) -> bool {
        coordinates.x >= 0
            && coordinates.x < self.size.0
            && coordinates.y >= 0
            && coordinates.y < self.size.1
    }
}

impl std::fmt::Debug for Maze {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        for iy in 0..self.size.1 {
            // print top passage
            for ix in 0..self.size.0 {
                f.write_str("·")?;
                if self
                    .get_field(&(ix, iy).into())
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
            for ix in 0..self.size.0 {
                let field = self.get_field(&(ix, iy).into()).unwrap();
                if field.has_passage(&Direction::West) {
                    f.write_str(" ")?;
                } else {
                    f.write_str("|")?;
                }

                f.write_str(match field.field_type {
                    FieldType::Start => "S",
                    FieldType::Goal => "G",
                    _ => " ",
                })?;
            }
            f.write_str("|\n")?;

            // print bottom line
            if iy == self.size.1 - 1 {
                for _ix in 0..self.size.0 {
                    f.write_str("·-")?;
                }
                f.write_str("·\n")?;
            }
        }

        Ok(())
    }
}

// implemented as into and not accessor because after exposing the internal graph, data integrity
// can not be guaranteed (size, start, goal could be made invalid).
impl Into<MazeGraph> for Maze {
    fn into(self) -> MazeGraph {
        self.graph
    }
}

impl PartialEq for Maze {
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start
            && self.goal == other.goal
            && self.size == other.size
            && is_isomorphic(
                &self.graph.clone().into_graph::<DefaultIx>(),
                &other.graph.clone().into_graph::<DefaultIx>(),
            )
    }
}

impl Eq for Maze {}
