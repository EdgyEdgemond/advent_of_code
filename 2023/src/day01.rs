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

pub fn question_two(values: &Vec<String>) -> anyhow::Result<i64> {
    let mut sum = 0;
    for input in values.iter(){
        let update = input
            .replace("one", "o1e")
            .replace("two", "t2o")
            .replace("three", "th3ee")
            .replace("four", "fo4r")
            .replace("five", "fi5e")
            .replace("six", "s6x")
            .replace("seven", "se7en")
            .replace("eight", "ei8ht")
            .replace("nine", "ni9e");
        let mut val = "".to_string();
        for c in update.chars(){
            if c.is_digit(10) {
                val.push(c);
                break;
            }
        }
        for c in update.chars().rev(){
            if c.is_digit(10) {
                val.push(c);
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

