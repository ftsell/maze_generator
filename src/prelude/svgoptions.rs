use crate::prelude::*;
use std::fmt::{Debug, Display, Formatter};

/// Otions for enerating SVG output
#[derive(Debug)]
pub struct SVGoptions {
    /// Padding, default: 10
    pub padding: i32,
    /// Height in pixels - use zero to derive a height based on the padding
    pub height: i32,
    /// Marker size - start and end, default: 2
    pub markersize: i32,
    /// Start marker colour - either a named colour like 'red' or a hex string like '#FF0000', default: "red"
    pub startcol: String,
    /// Goal marker colour, default: "blue"
    pub goalcol: String,
    /// Stroke width, default: 4
    pub strokewidth: i32,
    /// Stroke  colour, default: "#000000" (black)
    pub strokecol: String,
}

impl SVGoptions {
    /// Create a new instance from the specified coordinate components
    pub fn new() -> Self {
        Default::default()
    }
}

impl Default for SVGoptions {
    fn default() -> Self {
        SVGoptions {
            height: 0,
            padding: 10,
            markersize: 2,
            startcol: String::from("red"),
            goalcol: String::from("blue"),
            strokewidth: 4,
            strokecol: String::from("#000000"),
        }
    }
}
