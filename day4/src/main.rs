mod scratchcard;

use crate::scratchcard::Scratchcard;
use utils::file::read_lines;

fn main() {
    let mut total = 0;
    // let mut sum_of_powers = 0;

    if let Ok(lines) = read_lines("input") {
        for line_result in lines {
            if let Ok(line) = line_result {
                let scratchcard = Scratchcard::from_string(&line);

                println!("Scratchcard {:?}", scratchcard);

                total += scratchcard.points_value()
            }
        }
    }
    println!("total points = {total}");
    // println!("sum of powers = {sum_of_powers}")
}
