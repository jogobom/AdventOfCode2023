use crate::cell::Cell;
use crate::coord::Coord;

#[derive(Debug, PartialEq, Default)]
pub struct PartNumber {
    pub value: u64,
    pub cells: Vec<Cell>,
}

impl PartNumber {
    pub fn from_lines(lines: &Vec<&str>) -> Vec<PartNumber> {
        let mut part_numbers = vec![];
        let mut y = 0;
        for line in lines {
            part_numbers.extend(Self::from_line(&line, y));
            y += 1
        }
        part_numbers
    }

    fn from_line(line: &str, y: u32) -> Vec<PartNumber> {
        let mut x = 0;

        let mut cells_in_part_number = vec![];
        let mut part_numbers = vec![];

        for c in line.chars() {
            match c.is_numeric() {
                true => cells_in_part_number.push(Cell {
                    value: c,
                    coord: Coord { x, y },
                }),
                _ => {
                    if !cells_in_part_number.is_empty() {
                        part_numbers.push(Self::from_cells(cells_in_part_number));
                        cells_in_part_number = vec![]
                    }
                }
            }
            x += 1;
        }
        if !cells_in_part_number.is_empty() {
            part_numbers.push(Self::from_cells(cells_in_part_number));
        }
        part_numbers
    }

    fn from_cells(cells_in_part_number: Vec<Cell>) -> PartNumber {
        let value = cells_in_part_number
            .iter()
            .map(|c| c.value)
            .collect::<String>()
            .parse::<u64>()
            .unwrap();
        let new_part_number = PartNumber {
            value,
            cells: cells_in_part_number,
        };
        new_part_number
    }
}

#[test]
fn test_from_lines() {
    assert_eq!(
        PartNumber::from_lines(&vec!["123", "456", "789"]),
        vec![
            PartNumber {
                value: 123,
                cells: vec![
                    Cell {
                        value: '1',
                        coord: Coord { x: 0, y: 0 }
                    },
                    Cell {
                        value: '2',
                        coord: Coord { x: 1, y: 0 }
                    },
                    Cell {
                        value: '3',
                        coord: Coord { x: 2, y: 0 }
                    }
                ]
            },
            PartNumber {
                value: 456,
                cells: vec![
                    Cell {
                        value: '4',
                        coord: Coord { x: 0, y: 1 }
                    },
                    Cell {
                        value: '5',
                        coord: Coord { x: 1, y: 1 }
                    },
                    Cell {
                        value: '6',
                        coord: Coord { x: 2, y: 1 }
                    }
                ]
            },
            PartNumber {
                value: 789,
                cells: vec![
                    Cell {
                        value: '7',
                        coord: Coord { x: 0, y: 2 }
                    },
                    Cell {
                        value: '8',
                        coord: Coord { x: 1, y: 2 }
                    },
                    Cell {
                        value: '9',
                        coord: Coord { x: 2, y: 2 }
                    }
                ]
            }
        ]
    )
}
