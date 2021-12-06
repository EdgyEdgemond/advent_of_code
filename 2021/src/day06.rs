use anyhow::anyhow;

fn solution(ages: &Vec<u32>, days: u32) -> anyhow::Result<u64> {
    let mut day_counts = vec![0 as u64; 9];
    for age in ages {
        day_counts[*age as usize] += 1;
    }
    for _ in 1..=days {
        let new_fish = day_counts[0];
        for i in 0..8 {
            day_counts[i] = day_counts[i + 1]
        }
        day_counts[6] += new_fish;
        day_counts[8] = new_fish;
    }
    return Ok(day_counts.iter().fold(0, |sum, val| sum + val))
}

pub fn question_one(ages: &Vec<u32>) -> anyhow::Result<u64> {
    return solution(ages, 80)
}

pub fn question_two(ages: &Vec<u32>) -> anyhow::Result<u64> {
    return solution(ages, 256)
}

pub fn get_input(path: &str) -> anyhow::Result<Vec<u32>> {
    Ok(std::fs::read_to_string(path)?
        .lines()
        .next()
        .ok_or_else(|| anyhow!("Could not fetch ages"))
        .unwrap()
        .split(',')
        .map(|n| n.parse())
        .collect::<Result<Vec<u32>, _>>()?)
}

fn run() -> anyhow::Result<()> {
    let input = get_input("input/day06.txt")?;
    println!("D6Q1: {}", question_one(&input)?);
    println!("D6Q2: {}", question_two(&input)?);

    Ok(())
}

pub fn main() {
    if let Err(e) = run() {
        panic!("{:?}", e);
    }
}
