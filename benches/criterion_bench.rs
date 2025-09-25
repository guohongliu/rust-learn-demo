use criterion::{criterion_group, criterion_main, Bencher, Criterion};
use rust_learn_demo::algorithm_learn::longest_substring_without_repeating_characters::Solution;
fn v1(b: &mut Bencher) {
    b.iter(|| Solution::length_of_longest_substring_v1("abcdefg".parse().unwrap()))
}

fn v2(b: &mut Bencher) {
    b.iter(|| Solution::length_of_longest_substring_v2("abcdefg".parse().unwrap()))
}

fn v3(b: &mut Bencher) {
    b.iter(|| Solution::length_of_longest_substring("abcdefg".parse().unwrap()))
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("v1", v1);
    c.bench_function("v2", v2);
    c.bench_function("v3", v3);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);