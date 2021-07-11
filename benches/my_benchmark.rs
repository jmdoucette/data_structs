use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::Rng;

extern crate data_structs;
use data_structs::disjoint_set::DisjointSet;
use data_structs::disjoint_set_no_compression::DisjointSetNoCompression;

fn add_and_union(n: u64) {
    let mut rng = rand::thread_rng();
    let mut ds = DisjointSet::new();
    for i in 0..n {
        ds.add(i);
    }

    for _ in 0..n {
        let left = rng.gen_range(0..n);
        let right = rng.gen_range(0..n);
        ds.union(left, right);
    }
}

fn add_and_union_no_compression(n: u64) {
    let mut rng = rand::thread_rng();
    let mut ds = DisjointSetNoCompression::new();
    for i in 0..n {
        ds.add(i);
    }

    for _ in 0..n {
        let left = rng.gen_range(0..n);
        let right = rng.gen_range(0..n);
        ds.union(left, right);
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("add_and_union 100000", |b| {
        b.iter(|| add_and_union(black_box(100000)))
    });
    c.bench_function("add_and_union_no_compression 100000", |b| {
        b.iter(|| add_and_union_no_compression(black_box(100000)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
