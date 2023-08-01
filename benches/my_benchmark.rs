use benchmark::utils::*;
use criterion::{criterion_group, criterion_main, Criterion};

fn bench_lookup(c: &mut Criterion) {
    let mut group = c.benchmark_group("lookup");
    let v = vec_init();
    let h = hash_init();
    let t = trie_init();
    group.bench_function("vec", |b| b.iter(|| v.vec_lookup()));
    group.bench_function("hashmap", |b| b.iter(|| h.hash_lookup()));
    group.bench_function("trie", |b| b.iter(|| t.get_random_word()));
    group.finish();
}

criterion_group!(benches, bench_lookup);
criterion_main!(benches);
