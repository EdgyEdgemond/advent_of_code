use anyhow::anyhow;
use std::collections::HashMap;

pub fn question_one(crabs: &Vec<u32>) -> anyhow::Result<i32> {
    let max_pos = crabs.iter().fold(0, |max, &val| if val > max{ val } else{ max });
    let mut map: HashMap<u32, i32> = HashMap::new();
    for x in crabs {
        *map.entry(*x).or_default() += 1;
    }

    let mut min = i32::MAX;
    for i in (0..max_pos).rev() {
        let mut total = 0;
        for (x, count) in map.clone().into_iter() {
            total += (x as i32 - i as i32).abs() * count;
        }
        if total < min {
            min = total;
        }
    }

    Ok(min)
}

pub fn question_two(crabs: &Vec<u32>) -> anyhow::Result<u32> {
    let max_pos: u32 = crabs.iter().fold(0, |max, &val| if val > max{ val } else{ max });
    let mut map: HashMap<u32, u32> = HashMap::new();
    let mut tri_map: HashMap<u32, u32> = HashMap::new();
    for x in crabs {
        *map.entry(*x).or_default() += 1;
    }
    let mut t = 0;
    for i in 0..=max_pos {
        t += i;
        tri_map.insert(i,  t);
    }

    let mut min = u32::MAX;
    for i in (0..=max_pos).rev() {
        let mut total = 0;
        for (x, count) in map.clone().into_iter() {
            let d = (i as i32 - x as i32).abs();
            let t = tri_map.get(&(d as u32)).ok_or_else(|| anyhow!("Triangular not pre calculated for {}", d)).unwrap();
            total += t * count;
        }
        if total < min {
            min = total;
        }
    }

    Ok(min)
}

pub fn get_input(path: &str) -> anyhow::Result<Vec<u32>> {
    Ok(std::fs::read_to_string(path)?
        .lines()
        .next()
        .ok_or_else(|| anyhow!("Could not fetch crab positions"))
        .unwrap()
        .split(',')
        .map(|n| n.parse())
        .collect::<Result<Vec<u32>, _>>()?)
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
