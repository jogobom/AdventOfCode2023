use std::ops::Range;

use rayon::prelude::*;

use utils::file::read_lines;

use crate::map::Map;

mod map;

fn main() {
    let mut input_lines = read_lines("input").expect("Could not read input file");

    let strings = input_lines
        .next()
        .expect("First line of input did not match expectations")
        .unwrap();
    let seeds = create_ranges(parse_seeds(strings));
    input_lines.next();

    let mut maps = vec![];
    while let Some(Ok(mut l)) = input_lines.next() {
        let mut next_map = vec![];
        while !l.is_empty() {
            next_map.push(l);
            let next_line = input_lines.next();
            match next_line {
                Some(Ok(nl)) => l = nl,
                _ => break,
            }
        }
        maps.push(Map::from_lines(next_map));
    }

    let destinations = seeds
        .par_iter()
        .map(|s| run_seeds(s.clone(), &maps))
        .collect::<Vec<i64>>();
    println!("destinations: {:?}", destinations);

    println!("minimum location: {:?}", destinations.iter().min())
}

fn parse_seeds(seeds_line: String) -> Vec<i64> {
    seeds_line
        .split(" ")
        .skip(1)
        .map(|s| s.parse::<i64>().expect("Seed not in expected format"))
        .collect()
}

fn run_seeds(seeds: Range<i64>, maps: &Vec<Map>) -> i64 {
    let mut min_seed = (0, i64::MAX);

    for seed in seeds {
        let seed_journey = run_seed(seed, &maps);
        let seed_destination = seed_journey
            .last()
            .expect("Didn't expect a zero length journey!");
        if *seed_destination < min_seed.1 {
            min_seed = (seed, *seed_destination);
        }
    }
    min_seed.1
}

fn run_seed(seed: i64, maps: &Vec<Map>) -> Vec<i64> {
    let mut seed_journey = vec![seed; maps.len() + 1];
    for map in maps.iter().enumerate() {
        seed_journey[map.0 + 1] = map.1.src_to_dst(seed_journey[map.0]);
    }
    seed_journey
}

fn create_ranges(input: Vec<i64>) -> Vec<Range<i64>> {
    if input.len() % 2 != 0 {
        panic!("Input vector must have an even number of elements.");
    }

    let mut ranges = Vec::new();
    let mut iter = input.iter();

    while let Some(&start) = iter.next() {
        if let Some(&len) = iter.next() {
            ranges.push(start..start + len);
        }
    }

    ranges
}
