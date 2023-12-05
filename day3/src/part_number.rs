use crate::cell::Cell;

#[derive(Debug, PartialEq)]
pub struct PartNumber {
    value: u32,
    first_cell: Cell,
    last_cell: Cell,
}
