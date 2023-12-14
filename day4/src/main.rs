use std::fs::File;
use std::io::{BufReader, Lines};

use utils::file::read_lines;

use crate::scratchcard::Scratchcard;

mod scratchcard;

fn main() {
    let input_lines = read_lines("input").expect("Could not read input file");

    let scratchcards = build_scratchcards(input_lines);
    let copies_to_make = calculate_wins(&scratchcards);
    let counted_cards = scratchcards.iter().zip(copies_to_make.iter());

    println!(
        "total number of scratchcards = {:?}",
        counted_cards.fold(0, |acc, s| acc + s.1)
    );
}

fn build_scratchcards(input_lines: Lines<BufReader<File>>) -> Vec<Scratchcard> {
    let mut scratchcards = Vec::new();
    for line_result in input_lines {
        if let Ok(line) = line_result {
            scratchcards.push(Scratchcard::from_string(&line));
        }
    }
    scratchcards
}

fn calculate_wins(scratchcards: &Vec<Scratchcard>) -> Vec<usize> {
    let mut future_card_copies = vec![1; scratchcards.len()];
    for (i, scratchcard) in scratchcards.iter().enumerate() {
        let num_wins = scratchcard.number_of_wins();
        let begin = i + 1;
        let end = i + 1 + num_wins;
        for n in begin..end {
            future_card_copies[n] += future_card_copies[i]
        }
    }
    future_card_copies
}
