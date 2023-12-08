use crate::cell::Cell;
use crate::coord::Coord;

#[derive(Debug, PartialEq, Default)]
pub struct Gear {
    pub ratio: u64,
    pub coord: Coord,
}

impl Gear {
    pub fn from_lines(lines: &Vec<&str>) -> Vec<Cell> {
        let mut gears = vec![];
        let mut y = 0;
        for line in lines {
            gears.extend(Self::from_line(&line, y));
            y += 1
        }
        gears
    }

    fn from_line(line: &str, y: u32) -> Vec<Cell> {
        let mut x = 0;

        let mut gears = vec![];

        for c in line.chars() {
            if c == '*' {
                gears.push(Cell {
                    value: c,
                    coord: Coord { x, y },
                })
            }
            x += 1;
        }
        gears
    }
}

#[test]
fn test_from_lines() {
    assert_eq!(
        Gear::from_lines(&vec!["..3", ".*.", "7.*"]),
        vec![
            Cell {
                value: '*',
                coord: Coord { x: 1, y: 1 }
            },
            Cell {
                value: '*',
                coord: Coord { x: 2, y: 2 }
            }
        ]
    )
}
