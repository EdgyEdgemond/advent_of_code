fn check_grid(grid: &[&mut [u32]]) -> bool {
    for row in grid {
        match row
            .iter()
            .filter(|e| **e == 0)
            .collect::<Vec<&u32>>()
            .len() == 5 {
            true => {
                return true
            },
            _ => {},
        }
    }
    for i in 0..5 {
        match grid
            .iter()
            .map(|r| r[i])
            .filter(|e| *e == 0)
            .collect::<Vec<u32>>()
            .len() == 5 {
            true => {
                return true
            },
            _ => {},
        }
    }
    false
}

fn board_total(board: &[Vec<u32>], checks: &[&mut [u32]]) -> u32 {
    let mut total = 0;
    for (y, row) in board.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            total += *col * checks[y][x];
        }
    }
    total
}

fn question_one(numbers: &[u32], input: &[Vec<Vec<u32>>]) -> anyhow::Result<u32> {
    let mut grid_raw = vec![1; 25 * input.len()];
    let mut grid_base: Vec<_> = grid_raw.as_mut_slice().chunks_mut(5).collect();
    let mut grid_checks: Vec<_> = grid_base.as_mut_slice().chunks_mut(5).collect();

    for number in numbers {
        for (b, board) in input.iter().enumerate() {
            for (i, row) in board.iter().enumerate() {
                let index = row.iter().position(|e| e == number);
                match index {
                    Some(x) => {
                        grid_checks[b][i][x] = 0;
                    },
                    _ => {},
                }
            }
            match check_grid(grid_checks[b]) {
                true => {
                    let total = board_total(&board, grid_checks[b]);
                    return Ok(total * number);
                }
                _ => {},
            }
        }
    }
    Ok(0)
}


fn question_two(numbers: &[u32], input: &[Vec<Vec<u32>>]) -> anyhow::Result<u32> {
    let mut grid_raw = vec![1; 25 * input.len()];
    let mut grid_base: Vec<_> = grid_raw.as_mut_slice().chunks_mut(5).collect();
    let mut grid_checks: Vec<_> = grid_base.as_mut_slice().chunks_mut(5).collect();
    let mut completed = vec![0; input.len()];
    let mut num_left = input.len();

    for number in numbers {
        for (b, board) in input.iter().enumerate() {
            if completed[b] == 1 {
                continue
            }
            for (i, row) in board.iter().enumerate() {
                let index = row.iter().position(|e| e == number);
                match index {
                    Some(x) => {
                        grid_checks[b][i][x] = 0;
                    },
                    _ => {},
                }
            }
            match check_grid(grid_checks[b]) {
                true => {
                    completed[b] = 1;
                    num_left -= 1;
                    if num_left == 0 {
                        let total = board_total(&board, grid_checks[b]);
                        return Ok(total * number);
                    }
                }
                _ => {},
            }
        }
    }
    Ok(0)
}

fn get_input(path: &str) -> anyhow::Result<Vec<String>> {
    Ok(std::fs::read_to_string(path)?
        .lines()
        .map(|l| l.parse())
        .collect::<Result<Vec<String>, _>>()?)
}

fn run() -> anyhow::Result<()> {
    let input = get_input("input/day04.txt")?;
    let numbers = input.get(0)
        .ok_or_else(|| String::new())
        .unwrap()
        .split(',')
        .map(|n| n.parse())
        .collect::<Result<Vec<u32>, _>>()
        .unwrap();

    let boards = input.get(2..)
        .ok_or_else(|| panic!())
        .unwrap()
        .iter()
        .filter(|l| l != &&String::new())
        .map(|l| l.to_string())
        .collect::<Vec<String>>()
        .chunks(5)
        .map(|x| x.to_vec())
        .collect::<Vec<Vec<String>>>()
        .iter()
        .map(|x| x
            .iter()
            .map(|row| row
                .trim()
                .split_whitespace()
                .map(|n| n.parse())
                .collect::<Result<Vec<u32>, _>>()
                .unwrap()
            )
            .collect::<Vec<Vec<u32>>>()
        )
        .collect::<Vec<Vec<Vec<u32>>>>();

    println!("D4Q1: {}", question_one(&numbers, &boards)?);
    println!("D4Q2: {}", question_two(&numbers, &boards)?);

    Ok(())
}

pub fn main() {
    if let Err(e) = run() {
        panic!("{:?}", e);
    }
}

#[cfg(test)]
mod tests {
    extern crate test;
    use super::*;

    #[bench]
    fn benchmark_question_one(b: &mut test::Bencher) -> anyhow::Result<()> {
        let commands = get_input("input/day04.txt")?;
        b.iter(|| question_one(&commands));

        Ok(())
    }

    #[bench]
    fn benchmark_question_two(b: &mut test::Bencher) -> anyhow::Result<()> {
        let commands = get_input("input/day04.txt")?;
        b.iter(|| question_two(&commands));

        Ok(())
    }
}
