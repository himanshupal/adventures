use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rustlings::codewars::vowel_count;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Vowel Count", |b| {
        b.iter(|| vowel_count::func(black_box("abracadabra")))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
