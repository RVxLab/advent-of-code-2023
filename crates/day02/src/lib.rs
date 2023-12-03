use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn day02_a(file: &File) -> Result<String> {
    let read_buffer = BufReader::new(file);

    let code: u32 = read_buffer
        .lines()
        .filter_map(|line| line.ok())
        .filter_map(|line| -> Option<u32> {
            let parts = line.split(":").map(str::trim).collect::<Vec<_>>();

            let is_possible = parts
                .get(1)?
                .split(";")
                .map(str::trim)
                .flat_map(|turn| turn.split(",").map(str::trim))
                .all(|cube| {
                    let parts = cube.split_whitespace().collect::<Vec<_>>();
                    let amount: u8 = parts.get(0).unwrap().parse().unwrap();
                    let color = parts.get(1).unwrap();

                    match *color {
                        "red" => amount <= 12,
                        "green" => amount <= 13,
                        "blue" => amount <= 14,
                        _ => false,
                    }
                });

            if is_possible {
                let game = parts
                    .get(0)?
                    .split_whitespace()
                    .map(str::trim)
                    .collect::<Vec<_>>()
                    .get(1)?
                    .parse()
                    .ok()?;

                return Some(game);
            }

            None::<u32>
        })
        .sum();

    Ok(code.to_string())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_day02_a() {
        let file = File::open("../../inputs/day02.txt").unwrap();
        let solution = day02_a(&file).unwrap();
        println!("day02 a: {}", solution);
    }
}
