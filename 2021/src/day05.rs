use std::convert::TryFrom;
use std::str::FromStr;
use std::num::ParseIntError;

#[derive(Debug)]
pub struct Range {
    x1: u32,
    x2: u32,
    y1: u32,
    y2: u32,
}


impl FromStr for Range {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let new_s = s.replace(" -> ", ",");
        let pieces: Vec<&str> = new_s.split(',').collect();

        let x1 = pieces[0].parse()?;
        let y1 = pieces[1].parse()?;
        let x2 = pieces[2].parse()?;
        let y2 = pieces[3].parse()?;

        Ok(Range { x1: x1, x2: x2, y1: y1, y2: y2 })
    }
}

fn solution(input: &[Range], allow_diag: bool) -> anyhow::Result<u32> {
    let mut max_x = input.iter().fold(0, |max, val| if val.x1 > max{ val.x1 } else{ max });
    max_x = input.iter().fold(max_x, |max, val| if val.x2 > max{ val.x2 } else{ max });
    let mut max_y = input.iter().fold(0, |max, val| if val.y1 > max{ val.y1 } else{ max });
    max_y = input.iter().fold(max_y, |max, val| if val.y2 > max{ val.y2 } else{ max });
    max_x += 1;
    max_y += 1;

    let mut grid_raw = vec![0; usize::try_from(max_x * max_y).unwrap() ];
    let mut grid_base: Vec<_> = grid_raw.as_mut_slice().chunks_mut(usize::try_from(max_x).unwrap()).collect();

    let it = if allow_diag {
        input
            .iter()
            .collect::<Vec<&Range>>()
    } else {
        input
            .iter()
            .filter(|r| r.x1 == r.x2 || r.y1 == r.y2)
            .collect::<Vec<&Range>>()
    };
    for range in it {
        if range.x1 == range.x2 {
            let r: Vec<u32> = match range.y1 < range.y2 {
                true => (range.y1..=range.y2).collect(),
                false => (range.y2..=range.y1).rev().collect(),
            };
            for i in r {
                grid_base[i as usize][range.x1 as usize] += 1
            }
        } else if range.y1 == range.y2 {
            let r: Vec<u32> = match range.x1 < range.x2 {
                true => (range.x1..=range.x2).collect(),
                false => (range.x2..=range.x1).rev().collect(),
            };
            for i in r {
                grid_base[range.y1 as usize][i as usize] += 1
            }
        } else {
            let xr: Vec<u32> = match range.x1 < range.x2 {
                true => (range.x1..=range.x2).collect(),
                false => (range.x2..=range.x1).rev().collect(),
            };
            let yr: Vec<u32> = match range.y1 < range.y2 {
                true => (range.y1..=range.y2).collect(),
                false => (range.y2..=range.y1).rev().collect(),
            };

            for (i, v) in yr.iter().enumerate() {
                grid_base[*v as usize][xr[i] as usize] += 1
            }
        }
    }
    
    let total = grid_base
        .iter()
        .map(|r| r.iter().filter(|e| **e > 1).collect::<Vec<_>>().len())
        .fold(0, |sum, val| sum + val);

    Ok(total as u32)
}

pub fn question_one(input: &[Range]) -> anyhow::Result<u32> {
    return solution(input, false)
}


pub fn question_two(input: &[Range]) -> anyhow::Result<u32> {
    return solution(input, true)
}

pub fn get_input(path: &str) -> anyhow::Result<Vec<Range>> {
    Ok(std::fs::read_to_string(path)?
        .lines()
        .map(|l| l.parse())
        .collect::<Result<Vec<Range>, _>>()?)
}

fn run() -> anyhow::Result<()> {
    let input = get_input("input/day05.txt")?;
    println!("D5Q1: {}", question_one(&input)?);
    println!("D5Q2: {}", question_two(&input)?);

    Ok(())
}

pub fn main() {
    if let Err(e) = run() {
        panic!("{:?}", e);
    }
}
