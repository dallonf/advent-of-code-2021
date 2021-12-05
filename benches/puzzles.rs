use advent_of_code_2021::puzzles::{day04, day05};
use criterion::{criterion_group, criterion_main, Criterion};

fn day04_part_one(c: &mut Criterion) {
    c.bench_function("Day 04 Part One", |b| b.iter(|| day04::part_one()));
}
fn day04_part_two(c: &mut Criterion) {
    c.bench_function("Day 04 Part Two", |b| b.iter(|| day04::part_two()));
}
criterion_group!(day04, day04_part_one, day04_part_two);

fn day05_part_one(c: &mut Criterion) {
    c.bench_function("Day 05 Part One", |b| b.iter(|| day05::part_one()));
}
fn day05_part_two(c: &mut Criterion) {
    c.bench_function("Day 05 Part Two", |b| b.iter(|| day05::part_two()));
}
criterion_group!(day05, day05_part_one, day05_part_two);

criterion_group!(
    all_benches,
    day04_part_one,
    day04_part_two,
    day05_part_one,
    day05_part_two
);
criterion_main!(all_benches);
