
use criterion::*;
use ik_rs::dict::trie::Trie;

fn trie_build() -> Trie {
    let mut trie = Trie::new();
    trie.insert("Test");
    trie.insert("Tea");
    trie.insert("Background");
    trie.insert("Back");
    trie.insert("Brown");
    trie
}

fn trie_match() {
    let mut  trie = trie_build();
    trie.match_word("Back");
    trie.match_word("Tea");
}

fn trie_benchmark(c: &mut Criterion) {
    c.bench_function("trie match", |b| {
        b.iter(||trie_match())
    });
}

criterion_group!(benches, trie_benchmark);
criterion_main!(benches);