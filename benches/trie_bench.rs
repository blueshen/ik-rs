// extern crate test;
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use test::Bencher;
//     use ik_rs::dict::trie::Trie;
//
//     #[bench]
//     fn benchmark_trie(b: &mut Bencher) {
//         let mut trie = Trie::new();
//         trie.insert("Test");
//         trie.insert("Tea");
//         trie.insert("Background");
//         trie.insert("Back");
//         trie.insert("Brown");
//         b.iter(||trie.match_word("Back"));
//     }
//
// }