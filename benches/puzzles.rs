mod perf;

use std::time::Duration;

use advent_of_code_2021::puzzles::{day04, day05, day07, day15};
use criterion::{criterion_group, criterion_main, Criterion};

fn day04(c: &mut Criterion) {
    c.benchmark_group("Day 04")
        .bench_function("Part One", |b| b.iter(|| day04::part_one()))
        .bench_function("Part Two", |b| b.iter(|| day04::part_two()));
}

fn day05(c: &mut Criterion) {
    c.benchmark_group("Day 05")
        .bench_function("Part One", |b| b.iter(|| day05::part_one()))
        .bench_function("Part Two", |b| b.iter(|| day05::part_two()));
}

fn day07(c: &mut Criterion) {
    c.benchmark_group("Day 07")
        .measurement_time(Duration::from_secs_f32(7.5))
        .bench_function("Part One", |b| b.iter(|| day07::part_one()))
        .bench_function("Part Two", |b| b.iter(|| day07::part_two()));
}

fn day15(c: &mut Criterion) {
    c.benchmark_group("Day 15")
        .bench_function("Part One", |b| b.iter(|| day15::part_one()));
}

fn profiled_config() -> Criterion {
    Criterion::default().with_profiler(perf::FlamegraphProfiler::new(100))
}

criterion_group! {
    name = all_benches;
    config = profiled_config();
    targets = day04, day05, day07, day15
}
criterion_main!(all_benches);
