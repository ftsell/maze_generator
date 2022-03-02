use crate::prelude::*;
use petgraph::algo::is_isomorphic;
use petgraph::graphmap::GraphMap;
use petgraph::stable_graph::DefaultIx;
use petgraph::Undirected;

use std::collections::HashMap;
use std::fmt::Write;

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

impl Maze {
    /// Generate an SVG version of the maze
    pub fn to_svg(&self, options: Option<HashMap<&str, i32>>) -> Result<String, std::fmt::Error> {
        // Default generation options
        let mut opthash = HashMap::from([
            ("padding", 10),
            ("height", 10 + (self.size.0 + self.size.1) * 10),
            ("markersize", 2),
        ]);
        // Override with supplied options where possible
        match options {
            Some(options) => {
                for (opt, val) in options {
                    opthash.insert(opt, val);
                }
            }
            None => (),
        }
        // Use these options
        let padding = opthash["padding"]; // Pad the maze all around by this amount.
        let markersize = opthash["markersize"]; // Size of the Start and Goal markers
        let mut height = opthash["height"]; // Height and width of the maze image (excluding padding), in pixels
        let mut width = height * self.size.0 / self.size.1;

        // Scaling factors mapping maze coordinates to image/svg coordinates
        let scx = width / self.size.0;
        let scx2 = scx / 2;
        let scy = height / self.size.1;
        let scy2 = scy / 2;
        // Recalculate integer width, height now that we have the actual elements
        width = scx * self.size.0;
        height = scy * self.size.1;
        let mut x1;
        let mut x2;
        let mut y1;
        let mut y2;

        // Write the SVG to the return String
        let mut svg = String::new();
        writeln!(svg, "<?xml version=\"1.0\" encoding=\"utf-8\"?>").unwrap();
        writeln!(svg, "<svg xmlns=\"http://www.w3.org/2000/svg\"").unwrap();
        writeln!(svg, "    xmlns:xlink=\"http://www.w3.org/1999/xlink\"").unwrap();
        writeln!(
            svg,
            "    width=\"{}\" height=\"{}\" viewBox=\"{} {} {} {}\">",
            width + 2 * padding,
            height + 2 * padding,
            -padding,
            -padding,
            width + 2 * padding,
            height + 2 * padding
        )
        .unwrap();

        writeln!(svg, "<defs>\n<style type=\"text/css\"><![CDATA[").unwrap();
        writeln!(svg, "line {{").unwrap();
        writeln!(svg, "    stroke: #000000;\n    stroke-linecap: square;").unwrap();
        writeln!(svg, "    stroke-width: 5;\n}}").unwrap();
        writeln!(svg, "]]></style>\n</defs>").unwrap();

        for iy in 0..self.size.1 {
            // print top passage
            for ix in 0..self.size.0 {
                if self
                    .get_field(&(ix, iy).into())
                    .unwrap()
                    .has_passage(&Direction::North)
                {
                    // Do nothing. This code structure keeps the SVG output aligned with the original text debug output
                } else {
                    x1 = ix * scx;
                    y1 = iy * scy;
                    x2 = (ix + 1) * scx;
                    y2 = iy * scy;
                    writeln!(
                        svg,
                        "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\"/>",
                        x1, y1, x2, y2
                    )
                    .unwrap();
                }
            }

            // print left passage and room markers
            for ix in 0..self.size.0 {
                let field = self.get_field(&(ix, iy).into()).unwrap();
                if field.has_passage(&Direction::West) {
                    // Do nothing
                } else {
                    x1 = ix * scx;
                    y1 = iy * scy;
                    x2 = ix * scx;
                    y2 = (iy + 1) * scy;
                    writeln!(
                        svg,
                        "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\"/>",
                        x1, y1, x2, y2
                    )
                    .unwrap();
                }
                // Special cells
                match field.field_type {
                    FieldType::Start => {
                        x1 = ix * scx + scx2;
                        y1 = iy * scy + scy2;
                        writeln!(svg, "<circle cx=\"{}\" cy=\"{}\" r=\"{}\" stroke=\"red\" stroke-width=\"{}\" fill=\"red\" />", x1, y1, markersize, markersize + 1).unwrap();
                    }
                    FieldType::Goal => {
                        x1 = ix * scx + scx2;
                        y1 = iy * scy + scy2;
                        writeln!(svg, "<circle cx=\"{}\" cy=\"{}\" r=\"{}\" stroke=\"blue\" stroke-width=\"{}\" fill=\"blue\" />", x1, y1, markersize, markersize + 1).unwrap();
                    }
                    _ => continue,
                };
            }

            // print bottom border line
            x1 = 0;
            y1 = (self.size.1) * scy;
            x2 = (self.size.0) * scx;
            y2 = (self.size.1) * scy;
            writeln!(
                svg,
                "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\"/>",
                x1, y1, x2, y2
            )
            .unwrap();

            // print right border line
            x1 = (self.size.0) * scx;
            y1 = 0;
            x2 = (self.size.0) * scx;
            y2 = (self.size.1) * scy;
            writeln!(
                svg,
                "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\"/>",
                x1, y1, x2, y2
            )
            .unwrap();
        }
        writeln!(svg, "</svg>").unwrap();

        Ok(svg)
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
