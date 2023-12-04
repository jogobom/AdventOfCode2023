#[derive(Debug, PartialEq)]
pub struct SetOfCubes {
    pub red: i64,
    pub blue: i64,
    pub green: i64,
}

impl SetOfCubes {
    pub fn from_string(set_str: &str) -> SetOfCubes {
        let colours: Vec<&str> = set_str.split(",").collect();
        SetOfCubes {
            red: count_colour(&colours, "red"),
            green: count_colour(&colours, "green"),
            blue: count_colour(&colours, "blue"),
        }
    }
}

#[test]
fn test_from_string() {
    assert_eq!(
        SetOfCubes::from_string("6 red, 5 green, 16 blue"),
        SetOfCubes {
            red: 6,
            green: 5,
            blue: 16
        }
    );
}

fn count_colour(colours: &Vec<&str>, colour_name: &str) -> i64 {
    if let Some(this_colour) = colours.iter().find(|c| c.contains(&colour_name)) {
        let split_colour: Vec<&str> = this_colour.trim().split(" ").collect();
        return match split_colour[0].parse::<i64>() {
            Ok(n) => n,
            Err(_) => 0,
        };
    }
    0
}

#[test]
fn test_count_colour() {
    let colours = vec!["6 red", "42 banana"];
    assert_eq!(count_colour(&colours, "red"), 6);
    assert_eq!(count_colour(&colours, "banana"), 42);
}
