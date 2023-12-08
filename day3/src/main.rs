use crate::gear::Gear;
use crate::grid::Grid;
use crate::part_number::PartNumber;
use utils::file::read_lines;

mod cell;
mod coord;
mod gear;
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

    let mut part_numbers = vec![];
    let mut total_of_part_numbers = 0;
    for part_number in potential_part_numbers {
        if grid.valid_part_number(&part_number) {
            total_of_part_numbers += part_number.value;
            part_numbers.push(part_number)
        }
    }

    println!("Valid part numbers: {:?}", part_numbers);
    println!("Total is {:?}", total_of_part_numbers);

    let potential_gears = Gear::from_lines(&borrowed_grid_lines);

    let mut total_of_gear_ratios = 0;
    for potential_gear in potential_gears {
        if let Some(gear) = grid.get_gear(&potential_gear, &part_numbers) {
            total_of_gear_ratios += gear.ratio
        }
    }

    println!("Total gear ratio is {:?}", total_of_gear_ratios)
}
