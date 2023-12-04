use crate::set_of_cubes::SetOfCubes;

#[derive(Debug, PartialEq)]
pub struct Game {
    pub id: i32,
    pub sets_of_cubes: Vec<SetOfCubes>,
}

impl Game {
    pub fn valid(&self, limiting_set: SetOfCubes) -> bool {
        self.sets_of_cubes
            .iter()
            .filter(|s| {
                s.red > limiting_set.red
                    || s.green > limiting_set.green
                    || s.blue > limiting_set.blue
            })
            .count()
            == 0
    }

    pub fn from_string(line: &str) -> Game {
        let line_parts: Vec<&str> = line.split(": ").collect();
        let game_id_parts: Vec<&str> = line_parts[0].split(" ").collect();
        let id = game_id_parts[1].parse::<i32>();
        let set_of_cubes = line_parts[1];

        match id {
            Ok(id) => Game {
                id,
                sets_of_cubes: set_of_cubes
                    .split("; ")
                    .map(|s| SetOfCubes::from_string(s))
                    .collect(),
            },
            Err(_) => {
                panic!("Couldn't read game ID")
            }
        }
    }

    pub fn power(&self) -> i64 {
        let min_set = self.calc_minimum_set();
        min_set.red * min_set.green * min_set.blue
    }

    fn calc_minimum_set(&self) -> SetOfCubes {
        let min_red_needed = self.sets_of_cubes.iter().map(|s| s.red).max();
        let min_green_needed = self.sets_of_cubes.iter().map(|s| s.green).max();
        let min_blue_needed = self.sets_of_cubes.iter().map(|s| s.blue).max();
        SetOfCubes {
            red: match min_red_needed {
                None => 0,
                Some(m) => m,
            },
            green: match min_green_needed {
                None => 0,
                Some(m) => m,
            },
            blue: match min_blue_needed {
                None => 0,
                Some(m) => m,
            },
        }
    }
}

#[test]
fn test_power() {
    assert_eq!(
        Game {
            id: 42,
            sets_of_cubes: vec![SetOfCubes {
                red: 1,
                green: 1,
                blue: 1,
            }],
        }
        .power(),
        1
    );
    assert_eq!(
        Game {
            id: 42,
            sets_of_cubes: vec![SetOfCubes {
                red: 4,
                green: 5,
                blue: 6,
            }],
        }
        .power(),
        120
    );
}

#[test]
fn test_valid() {
    assert_eq!(
        Game {
            id: 42,
            sets_of_cubes: vec![SetOfCubes {
                red: 1,
                green: 1,
                blue: 1,
            }],
        }
        .valid(SetOfCubes {
            red: 1,
            green: 1,
            blue: 1,
        }),
        true
    );
    assert_eq!(
        Game {
            id: 42,
            sets_of_cubes: vec![SetOfCubes {
                red: 1,
                green: 1,
                blue: 1,
            }],
        }
        .valid(SetOfCubes {
            red: 0,
            green: 1,
            blue: 1,
        }),
        false
    );
    assert_eq!(
        Game {
            id: 42,
            sets_of_cubes: vec![SetOfCubes {
                red: 1,
                green: 1,
                blue: 1,
            }],
        }
        .valid(SetOfCubes {
            red: 1,
            green: 0,
            blue: 1,
        }),
        false
    );
}

#[test]
fn test_from_string() {
    assert_eq!(
        Game::from_string("Game 42: 6 red, 5 green, 16 blue; 30 green"),
        Game {
            id: 42,
            sets_of_cubes: vec![
                SetOfCubes {
                    red: 6,
                    green: 5,
                    blue: 16,
                },
                SetOfCubes {
                    red: 0,
                    green: 30,
                    blue: 0,
                },
            ],
        }
    );
}

#[test]
fn test_calc_minimum_set() {
    assert_eq!(
        Game::from_string("Game 3: 1 red, 2 green, 3 blue; 4 red; 2 green, 6 blue; 1 red")
            .calc_minimum_set(),
        SetOfCubes {
            red: 4,
            green: 2,
            blue: 6,
        }
    );
}
