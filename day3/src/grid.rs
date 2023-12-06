use crate::cell::Cell;
use crate::coord::Coord;
use crate::part_number::PartNumber;

#[derive(Debug, PartialEq, Default)]
pub struct Grid {
    pub cells: Vec<Cell>,
}

impl Grid {
    pub fn valid_part_number(&self, p: &PartNumber) -> bool {
        let mut result = false;
        for cell in p.cells.iter() {
            let neighbours = self.get_neighbours(&cell);
            for n in neighbours {
                if !n.value.is_numeric() && n.value != '.' {
                    result = true;
                }
            }
        }
        result
    }

    pub fn from_lines(lines: &Vec<&str>) -> Grid {
        let mut cells = vec![];
        let mut x = 0;
        let mut y = 0;
        for line in lines {
            for cell in line.chars() {
                cells.push(Cell {
                    value: cell,
                    coord: Coord { x, y },
                });
                x += 1;
            }
            x = 0;
            y += 1;
        }
        Grid { cells }
    }

    pub fn get_neighbours(&self, cell: &Cell) -> Vec<&Cell> {
        self.cells
            .iter()
            .filter(|c| cell.coord.adjacent(&c.coord))
            .collect()
    }
}
#[test]
fn test_check_part_number() {
    let example_grid = Grid {
        cells: vec![
            Cell {
                value: '1',
                coord: Coord { x: 0, y: 0 },
            },
            Cell {
                value: '+',
                coord: Coord { x: 0, y: 1 },
            },
        ],
    };

    assert_eq!(
        example_grid.valid_part_number(&&PartNumber {
            value: 1,
            cells: vec![Cell {
                value: '1',
                coord: Coord { x: 0, y: 0 },
            }],
        }),
        true
    );

    assert_eq!(
        example_grid.valid_part_number(&&PartNumber {
            value: 1,
            cells: vec![Cell {
                value: '1',
                coord: Coord { x: 2, y: 2 },
            }],
        }),
        false
    )
}

#[test]
fn test_empty_grid_neighbours() {
    assert_eq!(
        Grid { cells: Vec::new() }.get_neighbours(&Default::default()),
        Vec::<&Cell>::new()
    )
}

#[test]
fn test_one_cell_neighbours() {
    assert_eq!(
        Grid {
            cells: vec![Default::default()]
        }
        .get_neighbours(&Default::default()),
        Vec::<&Cell>::new()
    )
}

#[test]
fn test_first_cell_right_neighbour() {
    assert_eq!(
        Grid {
            cells: vec![
                Cell {
                    value: 'a',
                    coord: Coord { x: 0, y: 0 }
                },
                Cell {
                    value: 'b',
                    coord: Coord { x: 1, y: 0 }
                }
            ]
        }
        .get_neighbours(&Cell {
            value: 'a',
            coord: Coord { x: 0, y: 0 }
        }),
        vec![&Cell {
            value: 'b',
            coord: Coord { x: 1, y: 0 }
        }]
    )
}

#[test]
fn test_first_cell_left_neighbour() {
    assert_eq!(
        Grid {
            cells: vec![
                Cell {
                    value: 'a',
                    coord: Coord { x: 0, y: 0 }
                },
                Cell {
                    value: 'b',
                    coord: Coord { x: 1, y: 0 }
                }
            ]
        }
        .get_neighbours(&Cell {
            value: 'b',
            coord: Coord { x: 1, y: 0 }
        }),
        vec![&Cell {
            value: 'a',
            coord: Coord { x: 0, y: 0 }
        }]
    )
}
#[test]
fn test_first_cell_upper_neighbour() {
    assert_eq!(
        Grid {
            cells: vec![
                Cell {
                    value: 'a',
                    coord: Coord { x: 0, y: 1 }
                },
                Cell {
                    value: 'b',
                    coord: Coord { x: 0, y: 0 }
                }
            ]
        }
        .get_neighbours(&Cell {
            value: 'a',
            coord: Coord { x: 0, y: 1 }
        }),
        vec![&Cell {
            value: 'b',
            coord: Coord { x: 0, y: 0 }
        }]
    )
}
#[test]
fn test_first_cell_lower_neighbour() {
    assert_eq!(
        Grid {
            cells: vec![
                Cell {
                    value: 'a',
                    coord: Coord { x: 0, y: 0 }
                },
                Cell {
                    value: 'b',
                    coord: Coord { x: 0, y: 1 }
                }
            ]
        }
        .get_neighbours(&Cell {
            value: 'a',
            coord: Coord { x: 0, y: 0 }
        }),
        vec![&Cell {
            value: 'b',
            coord: Coord { x: 0, y: 1 }
        }]
    )
}

#[test]
fn test_empty_grid() {
    assert_eq!(Grid::from_lines(&vec![]), Grid { cells: vec![] });
}

#[test]
fn test_single_cell() {
    assert_eq!(
        Grid::from_lines(&vec!["5"]),
        Grid {
            cells: vec![Cell {
                value: '5',
                coord: Default::default()
            }]
        }
    );
}

#[test]
fn test_single_line() {
    assert_eq!(
        Grid::from_lines(&vec!["54"]),
        Grid {
            cells: vec![
                Cell {
                    value: '5',
                    coord: Coord { x: 0, y: 0 }
                },
                Cell {
                    value: '4',
                    coord: Coord { x: 1, y: 0 }
                }
            ]
        }
    );
}

#[test]
fn test_multiple_lines() {
    assert_eq!(
        Grid::from_lines(&vec!["54", "32"]),
        Grid {
            cells: vec![
                Cell {
                    value: '5',
                    coord: Coord { x: 0, y: 0 }
                },
                Cell {
                    value: '4',
                    coord: Coord { x: 1, y: 0 }
                },
                Cell {
                    value: '3',
                    coord: Coord { x: 0, y: 1 }
                },
                Cell {
                    value: '2',
                    coord: Coord { x: 1, y: 1 }
                }
            ]
        }
    );
}

#[test]
fn test_diagonal_neighbours() {
    assert_eq!(
        Grid {
            cells: vec![
                Cell {
                    value: 'a',
                    coord: Coord { x: 0, y: 0 }
                },
                Cell {
                    value: 'b',
                    coord: Coord { x: 1, y: 1 }
                }
            ]
        }
        .get_neighbours(&Cell {
            value: 'a',
            coord: Coord { x: 0, y: 0 }
        }),
        vec![&Cell {
            value: 'b',
            coord: Coord { x: 1, y: 1 }
        }]
    );
    assert_eq!(
        Grid {
            cells: vec![
                Cell {
                    value: 'a',
                    coord: Coord { x: 1, y: 1 }
                },
                Cell {
                    value: 'b',
                    coord: Coord { x: 0, y: 0 }
                }
            ]
        }
        .get_neighbours(&Cell {
            value: 'a',
            coord: Coord { x: 1, y: 1 }
        }),
        vec![&Cell {
            value: 'b',
            coord: Coord { x: 0, y: 0 }
        }]
    );
}
