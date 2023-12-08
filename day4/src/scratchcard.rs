use regex::{Match, Regex};

#[derive(Debug, PartialEq, Default)]
pub struct Scratchcard {
    pub number: u32,
    pub winners: Vec<u32>,
    pub have: Vec<u32>,
}

impl Scratchcard {
    pub(crate) fn points_value(&self) -> u64 {
        let matched_numbers = self
            .have
            .iter()
            .filter(|h| self.winners.contains(&h))
            .count() as u32;
        match matched_numbers {
            0 => 0,
            _ => 2u64.pow(matched_numbers - 1),
        }
    }

    fn collect_captures(captures: Option<Match>) -> Vec<&str> {
        captures.map_or(vec![], |m| {
            m.as_str()
                .trim()
                .split(" ")
                .filter(|&x| !x.is_empty())
                .collect()
        })
    }

    pub(crate) fn from_string(input: &str) -> Scratchcard {
        let re =
            Regex::new(r"^Card +(?<number>\d+): +(?<winners>[\d\s]+)\|(?<have>[\d\s]+)$").unwrap();
        let caps = re.captures(input).unwrap();

        let number = caps
            .name("number")
            .map_or(0u32, |m| m.as_str().parse::<u32>().unwrap());
        let winners: Vec<&str> = Self::collect_captures(caps.name("winners"));
        let have: Vec<&str> = Self::collect_captures(caps.name("have"));

        Scratchcard {
            number,
            winners: winners.iter().map(|w| w.parse::<u32>().unwrap()).collect(),
            have: have.iter().map(|h| h.parse::<u32>().unwrap()).collect(),
        }
    }
}
