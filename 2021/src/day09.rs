use anyhow::anyhow;
use std::collections::HashMap;

pub fn question_one(inputs: &Vec<Vec<i32>>) -> anyhow::Result<i32> {
    let mut total = 0;
    for (i, row) in inputs.iter().enumerate() {
        let d = vec![9i32; row.len()];
        let prow = inputs.get((i as i32 - 1) as usize).or(Some(&d)).unwrap();
        let nrow = inputs.get(i + 1).or(Some(&d)).unwrap();
        for (j, col) in row.iter().enumerate() {
            let left = row.get((j as i32 - 1) as usize).or(Some(&9)).unwrap();
            let right = row.get(j + 1).or(Some(&9)).unwrap();
            let up = prow[j];
            let down = nrow[j];

            let checks = [col < left, col < right, col < &up, col < &down];
            if checks.iter().all(|x| *x) {
                total += col + 1;
            }
        }
    }
    Ok(total)
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
    val: i32,
}

pub fn question_two(inputs: &Vec<Vec<i32>>) -> anyhow::Result<i32> {
    for input in inputs {
        println!("{:?}", input);
    }
    let mut total = 0;
    // Create a hashmap of all points by x, y.
    // Link list all the points
    // Determine basins (this is annoying given a basin can go down right up etc.
    // Get basin sizes order and get top 3 sum()
    // let d = vec![9i32; inputs[0].len()];
    let mut map: HashMap<(i32, i32), Point> = HashMap::new();
    for (i, row) in inputs.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if *col != 9 {
                map.insert((i as i32, j as i32), Point {y: i as i32, x: j as i32, val: *col});
            }
        }
    }
    // seen = SET
    // Basins = SET
    for (y, x) in map.keys() {
        let up = map.get(&(*y - 1, *x));
        let down = map.get(&(*y + 1, *x));
        let left = map.get(&(*y, *x - 1));
        let right = map.get(&(*y, *x + 1));

        print!("{:?}", (y, x));
        if up.is_some() {
            print!(" up");
        }
        if left.is_some() {
            print!(" left");
        }
        if down.is_some() {
            print!(" down");
        }
        if right.is_some() {
            print!(" right");
        }
        println!("");
    }
    Ok(total)
}

pub fn get_input(path: &str) -> anyhow::Result<Vec<Vec<i32>>> {
    Ok(std::fs::read_to_string(path)?
        .lines()
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap() as i32).collect::<Vec<i32>>())
        .collect::<Vec<Vec<i32>>>())
}

fn run() -> anyhow::Result<()> {
    let input = get_input("input/day09.txt")?;
    println!("D9Q1: {}", question_one(&input)?);
    println!("D9Q2: {}", question_two(&input)?);

    Ok(())
}

pub fn main() {
    if let Err(e) = run() {
        panic!("{:?}", e);
    }
}
