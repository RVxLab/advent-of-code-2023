use anyhow::Result;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn day01_a(input: &File) -> Result<String> {
    let read_buffer = BufReader::new(input);

    let code: u32 = read_buffer
        .lines()
        .filter_map(|line| line.ok())
        .filter_map(remove_non_digits)
        .filter_map(line_to_code)
        .sum();

    Ok(code.to_string())
}

pub fn day01_b(input: &File) -> Result<String> {
    let read_buffer = BufReader::new(input);

    let code: u32 = read_buffer
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            let mut digits = String::new();
            let mut chars = line.chars();

            loop {
                if let Some(digit) = chars.clone().peekable().peek().filter(|c| c.is_digit(10)) {
                    digits.push(digit.to_owned().to_owned());
                } else {
                    let mut spelled_digits = HashMap::new();
                    spelled_digits.insert("one", '1');
                    spelled_digits.insert("two", '2');
                    spelled_digits.insert("three", '3');
                    spelled_digits.insert("four", '4');
                    spelled_digits.insert("five", '5');
                    spelled_digits.insert("six", '6');
                    spelled_digits.insert("seven", '7');
                    spelled_digits.insert("eight", '8');
                    spelled_digits.insert("nine", '9');

                    let current = chars.as_str();

                    for (key, digit) in spelled_digits {
                        if current.starts_with(key) {
                            digits.push(digit);

                            continue;
                        }
                    }
                }

                if chars.next().is_none() {
                    break;
                }
            }

            digits
        })
        .filter_map(remove_non_digits)
        .filter_map(line_to_code)
        .sum();

    Ok(code.to_string())
}

fn line_to_code(line: String) -> Option<u32> {
    let mut chars = line.chars();
    let first = chars.nth(0).unwrap();
    let last = chars.last().unwrap_or(first);

    let mut code = first.to_string();
    code.push(last);
    code.parse::<u32>().ok()
}

fn remove_non_digits(line: String) -> Option<String> {
    let digits = line
        .chars()
        .filter(|char| char.is_digit(10))
        .collect::<String>();

    match digits.is_empty() {
        true => None,
        false => Some(digits),
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_day01_a() {
        let file = File::open("../../inputs/day01.txt").unwrap();
        let solution = day01_a(&file).unwrap();
        println!("day01 a: {}", solution);
    }

    #[test]
    fn test_day01_b() {
        let file = File::open("../../inputs/day01.txt").unwrap();
        let solution = day01_b(&file).unwrap();
        println!("day01 b: {}", solution);
    }
}
