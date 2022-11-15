use criterion::{criterion_group, criterion_main, Criterion};

use utility::AocDay;

pub fn criterion_benchmark(c: &mut Criterion) {
    bench::<aoc2019::day01::Day01>(c, 2019);
    bench::<aoc2019::day02::Day02>(c, 2019);
    bench::<aoc2019::day03::Day03>(c, 2019);
    bench::<aoc2019::day04::Day04>(c, 2019);
    bench::<aoc2019::day05::Day05>(c, 2019);
}

fn bench<Day: for<'a> AocDay<'a>>(c: &mut Criterion, year: usize) {
    let input = Day::read_input().unwrap();
    c.bench_function(&format!("{} {} Loading input", year, Day::day()), |b| {
        b.iter(|| Day::load(&input))
    });
    let input = Day::load(&input).unwrap();
    c.bench_function(&format!("{} {} Part 1", year, Day::day()), |b| {
        b.iter(|| Day::part_1(&input))
    });
    c.bench_function(&format!("{} {} Part 2", year, Day::day()), |b| {
        b.iter(|| Day::part_2(&input))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
