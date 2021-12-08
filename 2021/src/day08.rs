use anyhow::anyhow;
use itertools::Itertools;
use std::collections::HashMap;

pub fn question_one(inputs: &Vec<(Vec<String>, Vec<String>)>) -> anyhow::Result<i32> {
    let mut total: i32 = 0;
    for input in inputs {
        let c = input.1.iter().map(|o| o.len() as i32).filter(|l| [2, 3, 4, 7].contains(&l)).count();
        total += c as i32;
    }
    Ok(total)
}

pub fn question_two(inputs: &Vec<(Vec<String>, Vec<String>)>) -> anyhow::Result<i32> {
    let mut total: i32 = 0;
    for input in inputs {
        let mut digits: Vec<String> = input.0.iter().map(|d| d.chars().sorted().collect::<String>()).collect();
        digits.sort_unstable();
        digits.sort_by(|a, b| a.len().cmp(&b.len()));

        let mut map: HashMap<&str, char> = HashMap::new();
        let one = digits[0].as_str();
        let four = digits[2].as_str();
        let seven = digits[1].as_str();
        let eight = digits[9].as_str();
        map.insert(one, '1');
        map.insert(four, '4');
        map.insert(seven, '7');
        map.insert(eight, '8');

        // 3 is the five char code that contains digit 7
        let three = digits
            .iter()
            .filter(|d| d.len() == 5 && seven.chars().map(|c|d.contains(c)).all(|x| x))
            .next()
            .ok_or_else(|| anyhow!("can not find 3"))?
            .as_str();
        map.insert(three, '3');

        // 9 is the combo of 4 and 3
        let mut nine = String::new();
        nine.push_str(three);
        nine.push_str(four);
        nine = nine
            .chars()
            .sorted()
            .dedup()
            .collect::<String>();
        map.insert(nine.as_str(), '9');

        // 2 is the five char code missing the same segment as 9
        let lower_left = eight
            .chars()
            .filter(|c| !nine.contains(*c))
            .next()
            .ok_or_else(|| anyhow!("can not find lower left segment."))?;
        let two = digits
            .iter()
            .filter(|d| d.len() == 5 && d.contains(lower_left))
            .next()
            .ok_or_else(|| anyhow!("can not find 2"))?
            .as_str();
        map.insert(two, '2');

        // 5 is the only remaining 5 char code
        let five = digits
            .iter()
            .filter(|d| d.len() == 5 && !map.contains_key((*d.clone()).as_str()))
            .next()
            .ok_or_else(|| anyhow!("can not find 5"))?
            .as_str();
        map.insert(five, '5');

        // 6 is the six char code missing upper right segment
        let upper_right = one
            .chars()
            .filter(|c| !five.contains(*c))
            .next()
            .ok_or_else(|| anyhow!("can not find upper right segment."))?;
        let six = digits
            .iter()
            .filter(|d| d.len() == 6 && !d.contains(upper_right))
            .next()
            .ok_or_else(|| anyhow!("can not find 6"))?
            .as_str();
        map.insert(six, '6');

        // Only 0 left
        let zero = digits
            .iter()
            .filter(|d| !map.contains_key((*d.clone()).as_str()))
            .next()
            .ok_or_else(|| anyhow!("can not find 0"))?
            .as_str();
        map.insert(zero, '0');

        let output = input.1
            .iter()
            .map(|o| o.chars().sorted().collect::<String>())
            .map(|o| map[o.as_str()])
            .collect::<String>();
        let value: i32 = output.parse()?;
        total += value;
    }
    Ok(total)
}

pub fn get_input(path: &str) -> anyhow::Result<Vec<(Vec<String>, Vec<String>)>> {
    Ok(std::fs::read_to_string(path)?
        .lines()
        .map(|l| l.split('|').collect::<Vec<&str>>())
        .map(|p| (
                p[0].to_string().split_whitespace().map(|p| p.to_string()).collect::<Vec<String>>(),
                p[1].to_string().split_whitespace().map(|p| p.to_string()).collect::<Vec<String>>(),
            )
        )
        .collect::<Vec<(Vec<String>, Vec<String>)>>())
}

fn run() -> anyhow::Result<()> {
    let input = get_input("input/day08.txt")?;
    println!("D7Q1: {}", question_one(&input)?);
    println!("D7Q2: {}", question_two(&input)?);

    Ok(())
}

pub fn main() {
    if let Err(e) = run() {
        panic!("{:?}", e);
    }
}
