use crate::coord::Coord;

#[derive(Debug, PartialEq, Default)]
pub struct Cell {
    pub value: char,
    pub coord: Coord,
}
