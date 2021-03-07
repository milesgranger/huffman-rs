
use mrg::{compress};
use criterion::{black_box, criterion_group, criterion_main, Criterion};


pub fn criterion_benchmark(c: &mut Criterion) {
    let data = (0..1000000)
        .map(|_| b"oh what a beautiful morning, oh what a beautiful day!!".to_vec())
        .flat_map(|v| v)
        .collect::<Vec<u8>>();
    c.bench_function("block_compress", |b| b.iter(|| compress(black_box(&data))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
