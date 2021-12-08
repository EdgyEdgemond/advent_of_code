use anyhow::anyhow;
use std::collections::HashMap;

pub fn question_one(crabs: &Vec<i32>) -> anyhow::Result<i32> {
    let mut positions = crabs.to_vec();
    positions.sort_unstable();

    let median = positions
        .get(positions.len() / 2)
        .ok_or_else(|| anyhow!("Couldn't find median."))?;

    let mut map: HashMap<i32, i32> = HashMap::new();
    for x in crabs {
        *map.entry(*x).or_default() += 1;
    }

    let mut total = 0;
    for (x, count) in map.clone().into_iter() {
        total += (x as i32 - *median as i32).abs() * count;
    }

    Ok(total as i32)
}

pub fn question_two(crabs: &Vec<i32>) -> anyhow::Result<i32> {
    let avg: i32 = crabs.iter().fold(0, |acc, &val| acc + val) / crabs.len() as i32;

    let mut total = 0;
    for c in crabs {
        let d = (avg - c).abs();
        total += (d + d * d) / 2;
    }

    Ok(total)
}

pub fn get_input(path: &str) -> anyhow::Result<Vec<i32>> {
    Ok(std::fs::read_to_string(path)?
        .lines()
        .next()
        .ok_or_else(|| anyhow!("Could not fetch crab positions"))
        .unwrap()
        .split(',')
        .map(|n| n.parse())
        .collect::<Result<Vec<i32>, _>>()?)
}

fn run() -> anyhow::Result<()> {
    let input = get_input("input/day07.txt")?;
    println!("D7Q1: {}", question_one(&input)?);
    println!("D7Q2: {}", question_two(&input)?);

    Ok(())
}

pub fn main() {
    if let Err(e) = run() {
        panic!("{:?}", e);
    }
}
