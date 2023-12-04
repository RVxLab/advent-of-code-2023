use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Seek};

struct Number {
    offset: usize,
    code: String,
}

pub fn day03_a(file: &File) -> Result<String> {
    let mut read_buffer = BufReader::new(file);
    let lines = read_buffer
        .lines()
        .filter_map(|line| line.ok())
        .collect::<Vec<_>>();

    let line_length = lines.get(0).unwrap().len();
    let line_count = lines.len();
    let input = lines.join("");

    let mut start_offset: Option<usize> = None;

    for (offset, char) in input.char_indices() {
        if char.is_digit(10) {
            if start_offset.is_none() {
                start_offset = Some(offset);
            }

            continue;
        }

        if start_offset.is_some() {
            let start = start_offset.unwrap();
            let number = &input[start..offset];

            let row = offset / line_length;

            let mut code = String::new();

            if row > 0 {
                let upper_start = offset - number.len() - line_length - 1;
                let upper_end = upper_start + number.len() + 2;
                let upper_code = &input[upper_start..upper_end];
                code.push_str(upper_code);
            }

            let mid_start = offset - number.len();
            let mid_end = mid_start + number.len() + 2;
            let mid_code = &input[mid_start..mid_end];
            code.push_str(mid_code);

            if row < line_count {
                let lower_start = offset - number.len() + line_length - 1;
                let lower_end = lower_start + number.len() + 2;
                let lower_code = &input[lower_start..lower_end];
                code.push_str(lower_code);
            }

            if code
                .chars()
                .filter(|char| char != &'.' && !char.is_digit(10))
                .count()
                > 0
            {
                dbg!(code);
            }

            start_offset = None;
        }
    }

    Ok("".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day03_a() {
        let file = File::open("../../inputs/day03.txt").unwrap();
        let solution = day03_a(&file).unwrap();
        println!("day03 a: {}", solution);
    }
}
