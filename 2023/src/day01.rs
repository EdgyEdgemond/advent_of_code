pub fn get_input(path: &str) -> anyhow::Result<Vec<String>> {
    Ok(std::fs::read_to_string(path)?
        .lines()
        .map(|l| l.parse())
        .collect::<Result<Vec<String>, _>>()?)
}

pub fn question_one(values: &Vec<String>) -> anyhow::Result<i64> {
    let mut sum = 0;

    for input in values.iter(){
        let mut val = "".to_string();
        for c in input.chars(){
            if c.is_digit(10) {
                val.push(c);
                break;
            }
        }
        for c in input.chars().rev(){
            if c.is_digit(10) {
                val.push(c);
                break;
            }
        }
        sum += val.parse::<i64>()?;
    }

    return Ok(sum)
}

const NUMBERS: [(&str, char); 9] = [
    ("one", '1'),
    ("two", '2'),
    ("three", '3'),
    ("four", '4'),
    ("five", '5'),
    ("six", '6'),
    ("seven", '7'),
    ("eight", '8'),
    ("nine", '9'),
];

pub fn question_two(values: &Vec<String>) -> anyhow::Result<i64> {
    let mut sum = 0;
    for input in values.iter(){
        let mut val = "".to_string();
        for i in 0..input.chars().count() {
            // we've checked line length, unwrap doesn't risk out of bounds
            let c = input.chars().nth(i).unwrap();
            if c.is_digit(10) {
                val.push(c);
                break;
            }
            for (word, value) in NUMBERS {
                if input[i..].starts_with(word) {
                    val.push(value);
                    break;
                }
            }
            if val.chars().count() == 1 {
                break;
            }
        }
        for i in (0..input.chars().count()).rev() {
            let c = input.chars().nth(i).unwrap();
            if c.is_digit(10) {
                val.push(c);
                break;
            }
            for (word, value) in NUMBERS {
                if input[i..].starts_with(word) {
                    val.push(value);
                    break;
                }
            }
            if val.chars().count() == 2 {
                break;
            }
        }
        sum += val.parse::<i64>()?;
    }

    return Ok(sum)
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

