use anyhow::Result;
use day01::*;
use day02::*;
use std::fs::File;
use std::ops::Deref;

type Function = Box<dyn Fn(&File) -> Result<String>>;

struct Challenge {
    name: String,
    input: File,
    challenge_fn: ChallengeFn,
}

impl Challenge {
    fn new(name: &str, input: File, challenge_fn: Option<Function>) -> Self {
        Self {
            name: name.to_string(),
            input,
            challenge_fn: match challenge_fn {
                Some(function) => ChallengeFn(function),
                None => ChallengeFn(Box::new(|_| Ok("did not run".to_string()))),
            },
        }
    }

    fn run(&self) -> String {
        self.challenge_fn.as_ref()(&self.input).unwrap_or_else(|err| err.to_string())
    }
}

struct ChallengeFn(Function);

impl Deref for ChallengeFn {
    type Target = Function;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() -> Result<()> {
    let challenges: [Challenge; 50] = [
        Challenge::new(
            "day01_a",
            File::open("inputs/day01.txt")?,
            Some(Box::new(day01_a)),
        ),
        Challenge::new(
            "day01_b",
            File::open("inputs/day01.txt")?,
            Some(Box::new(day01_b)),
        ),
        Challenge::new(
            "day02_a",
            File::open("inputs/day02.txt")?,
            Some(Box::new(day02_a)),
        ),
        Challenge::new("day02_b", File::open("inputs/dummy.txt")?, None),
        Challenge::new("day03_a", File::open("inputs/dummy.txt")?, None),
        Challenge::new("day03_b", File::open("inputs/dummy.txt")?, None),
        Challenge::new("day04_a", File::open("inputs/dummy.txt")?, None),
        Challenge::new("day04_b", File::open("inputs/dummy.txt")?, None),
        Challenge::new("day05_a", File::open("inputs/dummy.txt")?, None),
        Challenge::new("day05_b", File::open("inputs/dummy.txt")?, None),
        Challenge::new("day06_a", File::open("inputs/dummy.txt")?, None),
        Challenge::new("day06_b", File::open("inputs/dummy.txt")?, None),
        Challenge::new("day07_a", File::open("inputs/dummy.txt")?, None),
        Challenge::new("day07_b", File::open("inputs/dummy.txt")?, None),
        Challenge::new("day08_a", File::open("inputs/dummy.txt")?, None),
        Challenge::new("day08_b", File::open("inputs/dummy.txt")?, None),
        Challenge::new("day09_a", File::open("inputs/dummy.txt")?, None),
        Challenge::new("day09_b", File::open("inputs/dummy.txt")?, None),
        Challenge::new("day10_a", File::open("inputs/dummy.txt")?, None),
        Challenge::new("day10_b", File::open("inputs/dummy.txt")?, None),
        Challenge::new("day11_a", File::open("inputs/dummy.txt")?, None),
        Challenge::new("day11_b", File::open("inputs/dummy.txt")?, None),
        Challenge::new("day12_a", File::open("inputs/dummy.txt")?, None),
        Challenge::new("day12_b", File::open("inputs/dummy.txt")?, None),
        Challenge::new("day13_a", File::open("inputs/dummy.txt")?, None),
        Challenge::new("day13_b", File::open("inputs/dummy.txt")?, None),
        Challenge::new("day14_a", File::open("inputs/dummy.txt")?, None),
        Challenge::new("day14_b", File::open("inputs/dummy.txt")?, None),
        Challenge::new("day15_a", File::open("inputs/dummy.txt")?, None),
        Challenge::new("day15_b", File::open("inputs/dummy.txt")?, None),
        Challenge::new("day16_a", File::open("inputs/dummy.txt")?, None),
        Challenge::new("day16_b", File::open("inputs/dummy.txt")?, None),
        Challenge::new("day17_a", File::open("inputs/dummy.txt")?, None),
        Challenge::new("day17_b", File::open("inputs/dummy.txt")?, None),
        Challenge::new("day18_a", File::open("inputs/dummy.txt")?, None),
        Challenge::new("day18_b", File::open("inputs/dummy.txt")?, None),
        Challenge::new("day19_a", File::open("inputs/dummy.txt")?, None),
        Challenge::new("day19_b", File::open("inputs/dummy.txt")?, None),
        Challenge::new("day20_a", File::open("inputs/dummy.txt")?, None),
        Challenge::new("day20_b", File::open("inputs/dummy.txt")?, None),
        Challenge::new("day21_a", File::open("inputs/dummy.txt")?, None),
        Challenge::new("day21_b", File::open("inputs/dummy.txt")?, None),
        Challenge::new("day22_a", File::open("inputs/dummy.txt")?, None),
        Challenge::new("day22_b", File::open("inputs/dummy.txt")?, None),
        Challenge::new("day23_a", File::open("inputs/dummy.txt")?, None),
        Challenge::new("day23_b", File::open("inputs/dummy.txt")?, None),
        Challenge::new("day24_a", File::open("inputs/dummy.txt")?, None),
        Challenge::new("day24_b", File::open("inputs/dummy.txt")?, None),
        Challenge::new("day25_a", File::open("inputs/dummy.txt")?, None),
        Challenge::new("day25_b", File::open("inputs/dummy.txt")?, None),
    ];

    for challenge in challenges {
        println!("{}: {}", challenge.name, challenge.run());
    }

    Ok(())
}
