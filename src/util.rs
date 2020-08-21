use crate::prelude::*;

pub(crate) fn are_coordinates_inside<T>(grid: &T, coordinates: &Coordinates) -> bool
where
    T: Grid,
{
    coordinates.0 >= 0
        && coordinates.0 < grid.get_size().0
        && coordinates.1 >= 0
        && coordinates.1 < grid.get_size().1
}

pub(crate) fn _index2coords<T>(gird: &T, i: i32) -> Coordinates
where
    T: Grid,
{
    let x = i % gird.get_size().0;
    let y = (i - x) / gird.get_size().0;
    (x, y)
}

pub(crate) fn coords2index<T>(grid: &T, coords: &Coordinates) -> i32
where
    T: Grid,
{
    (coords.1 * grid.get_size().0) + coords.0
}
