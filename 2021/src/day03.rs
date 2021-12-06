pub fn question_one(commands: &[String]) -> anyhow::Result<u32> {
    let size = commands[0].len() as u32;
    let mut vec: Vec<i32> = Vec::with_capacity(size as usize);
    for _ in 0..size {
        vec.push(0);
    }

    for command in commands {
        for (i, c) in command.chars().enumerate() {
            match c {
                '1' => vec[i] += 1,
                '0' => vec[i] -= 1,
                _ => panic!(),
            }
        }
    }

    let mut gamma_rate = String::new();
    let mut epsilon_rate = String::new();
    for v in vec.iter() {
        match v > &0 {
            true => {
                gamma_rate.push('1');
                epsilon_rate.push('0');
            },
            false => {
                gamma_rate.push('0');
                epsilon_rate.push('1');
            },
        }
    }
    let gamma_val = u32::from_str_radix(gamma_rate.as_str(), 2).unwrap();
    let epsilon_val = u32::from_str_radix(epsilon_rate.as_str(), 2).unwrap();

    Ok(gamma_val * epsilon_val)
}


pub fn question_two(commands: &[String]) -> anyhow::Result<u32> {
    let size = commands[0].len() as u32;
    let mut o_filter = String::new();
    let mut o_rating = 0;
    for i in 0..size + 1 {
        let mut char_rate = 0;
        let new_commands = commands
            .iter()
            .filter(|c| c.starts_with(o_filter.as_str()))
            .map(|c| c.parse())
            .collect::<Result<Vec<String>, _>>()?;

        if new_commands.len() == 1 {
            o_rating = u32::from_str_radix(new_commands[0].as_str(), 2).unwrap();
            break
        }
        for command in new_commands {
            let c = command.chars().nth(i as usize).unwrap();
            match c {
                '1' => char_rate += 1,
                '0' => char_rate -= 1,
                _ => panic!(),
            }
        }
        match char_rate >= 0 {
            true => o_filter.push('1'),
            false => o_filter.push('0'),
        }
    }
    
    let mut co2_filter = String::new();
    let mut co2_rating = 0;
    for i in 0..size + 1 {
        let mut char_rate = 0;
        let new_commands = commands
            .iter()
            .filter(|c| c.starts_with(co2_filter.as_str()))
            .map(|c| c.parse())
            .collect::<Result<Vec<String>, _>>()?;

        if new_commands.len() == 1 {
            co2_rating = u32::from_str_radix(new_commands[0].as_str(), 2).unwrap();
            break
        }
        for command in new_commands {
            let c = command.chars().nth(i as usize).unwrap();
            match c {
                '1' => char_rate += 1,
                '0' => char_rate -= 1,
                _ => panic!(),
            }
        }
        match char_rate >= 0 {
            true => co2_filter.push('0'),
            false => co2_filter.push('1'),
        }
    }

    Ok(o_rating * co2_rating)
}

pub fn get_input(path: &str) -> anyhow::Result<Vec<String>> {
    Ok(std::fs::read_to_string(path)?
        .lines()
        .map(|l| l.parse())
        .collect::<Result<Vec<String>, _>>()?)
}

fn run() -> anyhow::Result<()> {
    let input = get_input("input/day03.txt")?;
    println!("D3Q1: {}", question_one(&input)?);
    println!("D3Q2: {}", question_two(&input)?);

    Ok(())
}

pub fn main() {
    if let Err(e) = run() {
        panic!("{:?}", e);
    }
}
