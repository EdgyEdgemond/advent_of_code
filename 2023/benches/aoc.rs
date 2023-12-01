use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    #[cfg(feature = "day01")]
    {
        let input = aoc2023::day01::get_input("input/day01.txt").unwrap();

        c.bench_function("d01p01", |b| {
            b.iter(|| aoc2023::day01::question_one(black_box(&input)))
        });

        c.bench_function("d01p02", |b| {
            b.iter(|| aoc2023::day01::question_two(black_box(&input)))
        });
    }

}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
