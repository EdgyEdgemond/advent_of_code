use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    #[cfg(feature = "day01")]
    {
        let input = aoc2021::day01::get_input("input/day01.txt").unwrap();

        c.bench_function("d01p01", |b| {
            b.iter(|| aoc2021::day01::question_one(black_box(&input)))
        });

        c.bench_function("d01p02", |b| {
            b.iter(|| aoc2021::day01::question_two(black_box(&input)))
        });
    }

    #[cfg(feature = "day02")]
    {
        let input = aoc2021::day02::get_input("input/day02.txt").unwrap();

        c.bench_function("d02p01", |b| {
            b.iter(|| aoc2021::day02::question_one(black_box(&input)))
        });

        c.bench_function("d02p02", |b| {
            b.iter(|| aoc2021::day02::question_two(black_box(&input)))
        });
    }

    #[cfg(feature = "day03")]
    {
        let input = aoc2021::day03::get_input("input/day03.txt").unwrap();

        c.bench_function("d03p01", |b| {
            b.iter(|| aoc2021::day03::question_one(black_box(&input)))
        });
        c.bench_function("d03p02", |b| {
            b.iter(|| aoc2021::day03::question_two(black_box(&input)))
        });
    }

    #[cfg(feature = "day04")]
    {
        let input = aoc2021::day04::get_input("input/day04.txt").unwrap();
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

        c.bench_function("d04p01", |b| {
            b.iter(|| aoc2021::day04::question_one(black_box(&numbers), black_box(&boards)))
        });
        c.bench_function("d04p02", |b| {
            b.iter(|| aoc2021::day04::question_two(black_box(&numbers), black_box(&boards)))
        });
    }

    #[cfg(feature = "day05")]
    {
        let input = aoc2021::day05::get_input("input/day05.txt").unwrap();

        c.bench_function("d05p01", |b| {
            b.iter(|| aoc2021::day05::question_one(black_box(&input)))
        });
        c.bench_function("d05p02", |b| {
            b.iter(|| aoc2021::day05::question_two(black_box(&input)))
        });
    }

    #[cfg(feature = "day06")]
    {
        let input = aoc2021::day06::get_input("input/day06.txt").unwrap();

        c.bench_function("d06p01", |b| {
            b.iter(|| aoc2021::day06::question_one(black_box(&input)))
        });
        c.bench_function("d06p02", |b| {
            b.iter(|| aoc2021::day06::question_two(black_box(&input)))
        });
    }

    #[cfg(feature = "day07")]
    {
        let input = aoc2021::day07::get_input("input/day07.txt").unwrap();

        c.bench_function("d07p01", |b| {
            b.iter(|| aoc2021::day07::question_one(black_box(&input)))
        });
        c.bench_function("d07p02", |b| {
            b.iter(|| aoc2021::day07::question_two(black_box(&input)))
        });
    }

    #[cfg(feature = "day08")]
    {
        let input = aoc2021::day08::get_input("input/day08.txt").unwrap();

        c.bench_function("d08p01", |b| {
            b.iter(|| aoc2021::day08::question_one(black_box(&input)))
        });
        c.bench_function("d08p02", |b| {
            b.iter(|| aoc2021::day08::question_two(black_box(&input)))
        });
    }

    #[cfg(feature = "day09")]
    {
        let input = aoc2021::day09::get_input("input/day09.txt").unwrap();

        c.bench_function("d09p01", |b| {
            b.iter(|| aoc2021::day09::question_one(black_box(&input)))
        });
        c.bench_function("d09p02", |b| {
            b.iter(|| aoc2021::day09::question_two(black_box(&input)))
        });
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
