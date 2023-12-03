use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut total = 0;

    if let Ok(lines) = read_lines("input") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(line_str) = line {
                let first_digit =
                    get_digit_from_line(&line_str, find_numeric, find_name, Ordering::Less);
                let last_digit =
                    get_digit_from_line(&line_str, rfind_numeric, rfind_name, Ordering::Greater);

                let num = match (first_digit, last_digit) {
                    (Some(f), Some(l)) => Some(format!("{f}{l}")),
                    (Some(f), None) => Some(format!("{f}{f}")),
                    (None, Some(l)) => Some(format!("{l}{l}")),
                    (None, None) => None,
                };

                println!(
                    "{line_str} -> {:?} {:?} -> {:?}",
                    first_digit, last_digit, num
                );

                if let Some(n) = num {
                    let result = n.parse::<i32>();
                    if let Ok(r) = result {
                        total += r;
                    }
                }
            }
        }
    }
    println!("Total: {total}")
}

const NUM_STRINGS: [(i32, &str); 9] = [
    (1, "one"),
    (2, "two"),
    (3, "three"),
    (4, "four"),
    (5, "five"),
    (6, "six"),
    (7, "seven"),
    (8, "eight"),
    (9, "nine"),
];

fn get_digit_from_line(
    line_str: &String,
    find_numeric: fn(&String) -> Option<usize>,
    find_name: fn(&String, &str) -> Option<usize>,
    ordering: Ordering,
) -> Option<i32> {
    let mut digit_position = None;
    let mut digit = None;

    if let Some(fp) = find_numeric(line_str) {
        digit_position = Some(fp);
        if let Some(f) = line_str.get(fp..fp + 1) {
            if let Ok(x) = f.parse::<i32>() {
                digit = Some(x)
            }
        }
    }

    for num_str in NUM_STRINGS {
        let first_match = find_name(line_str, num_str.1);
        match (first_match, digit_position) {
            (Some(fm), Some(dp)) => {
                if fm.cmp(&dp) == ordering {
                    digit_position = first_match;
                    digit = Some(num_str.0);
                }
            }
            (Some(_fm), None) => {
                digit_position = first_match;
                digit = Some(num_str.0);
            }
            (None, Some(_dp)) => {}
            (None, None) => {}
        };
    }

    return digit;
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn find_numeric(str: &String) -> Option<usize> {
    return str.find(char::is_numeric);
}

fn rfind_numeric(str: &String) -> Option<usize> {
    return str.rfind(char::is_numeric);
}

fn find_name(str: &String, name: &str) -> Option<usize> {
    return str.find(name);
}

fn rfind_name(str: &String, name: &str) -> Option<usize> {
    return str.rfind(name);
}
