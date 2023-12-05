use crate::coord::Coord;

#[derive(Debug, PartialEq)]
pub struct Cell {
    pub value: char,
    pub coord: Coord,
}
