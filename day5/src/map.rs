use std::ops::Range;

#[derive(Debug)]
struct Mapping {
    range: Range<i64>,
    offset: i64,
}

#[derive(Debug)]
pub(crate) struct Map {
    mappings: Vec<Mapping>,
}

fn read_ranges(line: &str) -> Mapping {
    let split: Vec<&str> = line.split(' ').collect();
    let dst = split[0]
        .parse::<i64>()
        .expect("Failed to parse numbers in map");
    let src = split[1]
        .parse::<i64>()
        .expect("Failed to parse numbers in map");
    let len = split[2]
        .parse::<i64>()
        .expect("Failed to parse numbers in map");
    Mapping {
        range: src..src + len,
        offset: dst - src,
    }
}

fn read_mappings(lines: &[String]) -> Vec<Mapping> {
    let mut mappings = vec![];

    for line in lines {
        mappings.push(read_ranges(&line));
    }
    mappings
}

impl Map {
    pub(crate) fn src_to_dst(&self, src: i64) -> i64 {
        for mapping in &self.mappings {
            if mapping.range.contains(&src) {
                return src + mapping.offset;
            }
        }
        src
    }

    pub(crate) fn from_lines(lines: Vec<String>) -> Map {
        let mappings = read_mappings(&lines[1..]);

        Map { mappings }
    }
}

#[test]
fn tdd() {
    let lines = vec![
        "seed-to-soil map:".to_string(),
        "50 98 2".to_string(),
        "52 50 48".to_string(),
    ];
    let map = Map::from_lines(lines);

    assert_eq!(map.src_to_dst(79), 81);
    assert_eq!(map.src_to_dst(14), 14);
    assert_eq!(map.src_to_dst(55), 57);
    assert_eq!(map.src_to_dst(13), 13);
}
