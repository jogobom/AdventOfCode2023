use crate::grid::Grid;
use crate::part_number::PartNumber;
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
    let borrowed_grid_lines = grid_lines_from_file.iter().map(|s| &**s).collect();

    let grid = Grid::from_lines(&borrowed_grid_lines);
    let potential_part_numbers = PartNumber::from_lines(&borrowed_grid_lines);

    let mut values = vec![];
    let mut total = 0;

    for part_number in potential_part_numbers {
        if grid.valid_part_number(&part_number) {
            values.push(part_number.value);
            total += part_number.value
        }
    }

    println!("Valid part numbers: {:?}", values);
    println!("Total is {:?}", total)
}
