use anyhow::anyhow;

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

pub fn question_two(inputs: &Vec<Vec<i32>>) -> anyhow::Result<i32> {
    Ok(0)
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
