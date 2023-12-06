#[derive(Debug, PartialEq, Default)]
pub struct Coord {
    pub x: u32,
    pub y: u32,
}

impl Coord {
    pub fn adjacent(&self, other: &Coord) -> bool {
        let dx = self.x.abs_diff(other.x);
        let dy = self.y.abs_diff(other.y);
        return match (dx, dy) {
            (1, 0) => true,
            (0, 1) => true,
            (1, 1) => true,
            _ => false,
        };
    }
}

#[test]
fn test_adjacency() {
    let test_cell = Coord { x: 5, y: 5 };

    assert_eq!(test_cell.adjacent(&Coord { x: 3, y: 3 }), false);
    assert_eq!(test_cell.adjacent(&Coord { x: 4, y: 3 }), false);
    assert_eq!(test_cell.adjacent(&Coord { x: 5, y: 3 }), false);
    assert_eq!(test_cell.adjacent(&Coord { x: 6, y: 3 }), false);
    assert_eq!(test_cell.adjacent(&Coord { x: 7, y: 3 }), false);

    assert_eq!(test_cell.adjacent(&Coord { x: 3, y: 4 }), false);
    assert_eq!(test_cell.adjacent(&Coord { x: 4, y: 4 }), true);
    assert_eq!(test_cell.adjacent(&Coord { x: 5, y: 4 }), true);
    assert_eq!(test_cell.adjacent(&Coord { x: 6, y: 4 }), true);
    assert_eq!(test_cell.adjacent(&Coord { x: 7, y: 4 }), false);

    assert_eq!(test_cell.adjacent(&Coord { x: 3, y: 5 }), false);
    assert_eq!(test_cell.adjacent(&Coord { x: 4, y: 5 }), true);
    assert_eq!(test_cell.adjacent(&Coord { x: 5, y: 5 }), false);
    assert_eq!(test_cell.adjacent(&Coord { x: 6, y: 5 }), true);
    assert_eq!(test_cell.adjacent(&Coord { x: 7, y: 5 }), false);

    assert_eq!(test_cell.adjacent(&Coord { x: 3, y: 6 }), false);
    assert_eq!(test_cell.adjacent(&Coord { x: 4, y: 6 }), true);
    assert_eq!(test_cell.adjacent(&Coord { x: 5, y: 6 }), true);
    assert_eq!(test_cell.adjacent(&Coord { x: 6, y: 6 }), true);
    assert_eq!(test_cell.adjacent(&Coord { x: 7, y: 6 }), false);

    assert_eq!(test_cell.adjacent(&Coord { x: 3, y: 7 }), false);
    assert_eq!(test_cell.adjacent(&Coord { x: 4, y: 7 }), false);
    assert_eq!(test_cell.adjacent(&Coord { x: 5, y: 7 }), false);
    assert_eq!(test_cell.adjacent(&Coord { x: 6, y: 7 }), false);
    assert_eq!(test_cell.adjacent(&Coord { x: 7, y: 7 }), false);
}
