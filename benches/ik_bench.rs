use criterion::*;
use ik_rs::core::ik_segmenter::{IKSegmenter, TokenMode};
use ik_rs::dict::trie::Trie;
use once_cell::sync::Lazy;
use random_string;

cfg_if::cfg_if! {
    if #[cfg(feature="use-parking-lot")] {
        use parking_lot::RwLock;
    } else /*if #[cfg(feature="use-std-sync")]*/ {
        use std::sync::RwLock;
    }
}

pub static GLOBAL_IK: Lazy<RwLock<IKSegmenter>> = Lazy::new(|| {
    let ik = IKSegmenter::new();
    RwLock::new(ik)
});

pub static GLOBAL_TRIE: Lazy<RwLock<Trie>> = Lazy::new(|| {
    let mut trie = Trie::new();
    trie.insert("Test");
    trie.insert("Tea");
    trie.insert("Background");
    trie.insert("Back");
    trie.insert("Brown");

    let charset = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    for _ in 0..10000 {
        let r = random_string::generate(10, charset);
        trie.insert(r.as_str());
    }
    RwLock::new(trie)
});

// expect 312 ns
fn trie_match() {
    let lock_guard = {cfg_if::cfg_if!{
        if #[cfg(feature="use-parking-lot")] {GLOBAL_TRIE.read()}
        else /*if #[cfg(feature="use-std-sync")]*/ {GLOBAL_TRIE.read().unwrap()}
    }};
    lock_guard.match_word("Back");
}

// expect 17.8 µs
fn ik_tokenize() {
    let lock_guard = {cfg_if::cfg_if! {
        if #[cfg(feature="use-parking-lot")] {GLOBAL_IK.read()}
        else /*if #[cfg(feature="use-std-sync")]*/ {GLOBAL_IK.read().unwrap()}
    }};
    lock_guard.tokenize("中华人民共和国有960万平方公里土地", TokenMode::SEARCH);
}

fn ik_benchmark(c: &mut Criterion) {
    c.bench_function("ik_tokenize_benchmark", |b| b.iter(|| ik_tokenize()));
}

fn trie_benchmark(c: &mut Criterion) {
    c.bench_function("trie_match_benchmark", |b| b.iter(|| trie_match()));
}

criterion_group!(benches, ik_benchmark, trie_benchmark);
criterion_main!(benches);
