use crate::grid::Grid;
use utils::file::read_lines;

mod cell;
mod coord;
mod grid;
mod part_number;

fn main() {
    let mut grid_lines_from_file: Vec<String> = Vec::new();

    if let Ok(lines) = read_lines("input") {
        for line in lines {
            if let Ok(l) = line {
                grid_lines_from_file.push(l)
            }
        }
    }

    let grid = Grid::from_lines(grid_lines_from_file);

    println!("{:?}", grid)
}
