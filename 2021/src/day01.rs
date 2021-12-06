pub fn get_input(path: &str) -> anyhow::Result<Vec<i64>> {
    Ok(std::fs::read_to_string(path)?
        .lines()
        .map(|l| l.parse())
        .collect::<Result<Vec<i64>, _>>()?)
}

pub fn question_one(values: &Vec<i64>) -> anyhow::Result<i64> {
    let mut count = 0;

    for i in 1..values.len(){
        if values[i] > values[i - 1] {
            count += 1;
        }
    }

    return Ok(count)
}

pub fn question_two(values: &Vec<i64>) -> anyhow::Result<i64> {
    let mut count = 0;

    for i in 3..values.len(){
        if values[i] > values[i - 3] {
            count += 1;
        }
    }

    return Ok(count)
}

fn run() -> anyhow::Result<()> {
    let input = get_input("input/day01.txt")?;
    println!("D1Q1: {}", question_one(&input)?);
    println!("D1Q2: {}", question_two(&input)?);

    Ok(())
}

pub fn main() {
    if let Err(e) = run() {
        panic!("{:?}", e);
    }
}
