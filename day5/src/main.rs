use utils::file::read_lines;

use crate::map::Map;

mod map;

fn main() {
    let mut input_lines = read_lines("input").expect("Could not read input file");

    let seeds = parse_seeds(
        input_lines
            .next()
            .expect("First line of input did not match expectations")
            .unwrap(),
    );
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

    let processed_seeds = seeds
        .iter()
        .map(|s| run_seed(vec![*s], &maps))
        .collect::<Vec<Vec<i64>>>();
    println!("processed seeds: {:?}", processed_seeds);

    let destinations = processed_seeds
        .iter()
        .map(|p| *p.last().unwrap())
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

fn run_seed(seed_journey: Vec<i64>, maps: &Vec<Map>) -> Vec<i64> {
    maps.iter().fold(seed_journey, |acc, m| {
        let last_seed = acc.last().expect("Didn't expect zero elements").clone();
        [acc, vec![m.src_to_dst(last_seed)]].concat()
    })
}
