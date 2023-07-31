use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn vec_insert() {
    let file = File::open("words.txt").expect("unable to open file");
    let reader = BufReader::new(file);
    let mut word_list = Vec::new();

    for line in reader.lines() {
        if let Ok(words) = line {
            word_list.extend(words.split(',').map(|s| s.trim().to_string()))
        }
    }
}

fn vec_lookup() -> String {
    let file = File::open("words.txt").expect("unable to open file");
    let reader = BufReader::new(file);
    let mut word_list = Vec::new();

    for line in reader.lines() {
        if let Ok(words) = line {
            word_list.extend(words.split(',').map(|s| s.trim().to_string()))
        }
    }
    let mut rng = thread_rng();
    word_list.choose(&mut rng).unwrap().to_string()
}

fn n_vec_lookup(n: u32) {
    let file = File::open("words.txt").expect("unable to open file");
    let reader = BufReader::new(file);
    let mut word_list = Vec::new();

    for line in reader.lines() {
        if let Ok(words) = line {
            word_list.extend(words.split(',').map(|s| s.trim().to_string()))
        }
    }
    for _ in 0..n {
        let mut rng = thread_rng();
        word_list.choose(&mut rng).unwrap().to_string();
    }
}

// TODO: index map and hashmap
// https://crates.io/crates/indexmap
// https://github.com/bluss/indexmap/blob/master/benches/bench.rs#L277

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("vec insert", |b| b.iter(|| vec_insert()));
    c.bench_function("vec lookup", |b| b.iter(|| vec_lookup()));
    c.bench_function("n vec lookup", |b| b.iter(|| n_vec_lookup(black_box(1000))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
