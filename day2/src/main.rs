mod game;
mod set_of_cubes;

use crate::game::Game;
use crate::set_of_cubes::SetOfCubes;
use utils::file::read_lines;

fn main() {
    let mut total = 0;
    let mut sum_of_powers = 0;

    if let Ok(lines) = read_lines("input") {
        for line_result in lines {
            if let Ok(line) = line_result {
                let game = Game::from_string(&line);
                if game.valid(SetOfCubes {
                    red: 12,
                    green: 13,
                    blue: 14,
                }) {
                    total += game.id
                }
                sum_of_powers += game.power()
            }
        }
    }
    println!("total of valid games = {total}");
    println!("sum of powers = {sum_of_powers}")
}
