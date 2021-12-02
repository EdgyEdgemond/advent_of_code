use std::str::FromStr;
use std::num::ParseIntError;

#[derive(Debug)]
struct Command {
    direction: String,
    value: u32,
}

impl FromStr for Command {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pieces: Vec<&str> = s.split(' ').collect();

        let value = pieces[1].parse()?;

        Ok(Command { direction: pieces[0].to_string(), value: value })
    }
}

#[derive(Debug)]
struct Point {
    x: u32,
    y: u32,
}

impl Point {
    fn up(&mut self, value: u32) {
        self.y -= value
    }

    fn down(&mut self, value: u32) {
        self.y += value
    }

    fn forward(&mut self, value: u32) {
        self.x += value
    }
}

#[derive(Debug)]
struct AimPoint {
    x: u32,
    y: u32,
    aim: u32,
}

impl AimPoint {
    fn up(&mut self, value: u32) {
        self.aim -= value;
    }

    fn down(&mut self, value: u32) {
        self.aim += value;
    }

    fn forward(&mut self, value: u32) {
        self.x += value;
        self.y += self.aim * value;
    }
}

fn get_input(path: &str) -> anyhow::Result<Vec<Command>> {
    Ok(std::fs::read_to_string(path)?
        .lines()
        .map(|l| l.parse())
        .collect::<Result<Vec<Command>, _>>()?)
}

fn question_one(commands: &[Command]) -> anyhow::Result<u32> {
    let mut p = Point { x: 0, y: 0 };

    for c in commands {
        match c.direction.as_str() {
            "forward" => p.forward(c.value),
            "up" => p.up(c.value),
            "down" => p.down(c.value),
            _ => panic!(),
        }
    }

    Ok(p.x * p.y)
}

fn question_two(commands: &[Command]) -> anyhow::Result<u32> {
    let mut p = AimPoint { x: 0, y: 0, aim: 0 };

    for c in commands {
        match c.direction.as_str() {
            "forward" => p.forward(c.value),
            "up" => p.up(c.value),
            "down" => p.down(c.value),
            _ => panic!(),
        }
    }

    Ok(p.x * p.y)
}

fn run() -> anyhow::Result<()> {
    let input = get_input("input/day02.txt")?;
    println!("D2Q1: {}", question_one(&input)?);
    println!("D2Q2: {}", question_two(&input)?);

    Ok(())
}

pub fn main() {
    if let Err(e) = run() {
        panic!("{:?}", e);
    }
}

#[cfg(test)]
mod tests {
    extern crate test;
    use super::*;

    #[bench]
    fn benchmark_question_one(b: &mut test::Bencher) -> anyhow::Result<()> {
        let commands = get_input("input/day02.txt")?;
        b.iter(|| question_one(&commands));

        Ok(())
    }

    #[bench]
    fn benchmark_question_two(b: &mut test::Bencher) -> anyhow::Result<()> {
        let commands = get_input("input/day02.txt")?;
        b.iter(|| question_two(&commands));

        Ok(())
    }
}
