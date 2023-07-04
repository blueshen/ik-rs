<!-- Badges section here. -->
[![Crates.io](https://img.shields.io/badge/crates.io-0.5.0-green)](https://crates.io/crates/ik-rs)
[![License](https://img.shields.io/badge/license-LGPL--2.1-blue)](./LICENSE)
[![Open Source Love](https://badges.frapsoft.com/os/v1/open-source.svg?v=103)](https://github.com/blueshen/ik-rs/releases)
[![Build Status](https://app.travis-ci.com/blueshen/ik-rs.svg?branch=main)](https://app.travis-ci.com/github/blueshen/ik-rs)

[![GitHub forks](https://img.shields.io/github/forks/blueshen/ik-rs.svg?style=social&label=Fork)](https://github.com/blueshen/ik-rs/network/members)
[![GitHub stars](https://img.shields.io/github/stars/blueshen/ik-rs.svg?style=social&label=Star)](https://github.com/blueshen/ik-rs/stargazers)
<!-- /Badges section end. -->

# ik-rs

[ik-analyzer](https://github.com/blueshen/ik-analyzer) for Rust



# Usage
## add to Cargo.toml
```toml
[dependencies]
ik-rs = "0.5.0"
```

## Chinese Segment
```rust
#[cfg(test)]
mod test {
    use ik_rs::core::ik_segmenter::{IKSegmenter, TokenMode};

    #[test]
    pub fn test_ik() {
        let mut ik = IKSegmenter::new();
        let text = "中华人民共和国";
        let tokens = ik.tokenize(text, TokenMode::INDEX); // TokenMode::SEARCH
        let mut token_texts = Vec::new();
        for token in tokens.iter() {
            println!("{:?}", token);
            token_texts.push(token.get_lexeme_text());
        }
        assert_eq!(
            token_texts,
            vec![
                "中华人民共和国",
                "中华人民",
                "中华",
                "华人",
                "人民共和国",
                "人民",
                "共和国",
                "共和",
                "国"
            ]
        )
    }
}

```
# BenchMark

High performance
```shell
cargo bench
ik_tokenize_benchmark   time:   [19.366 µs 19.572 µs 19.850 µs]
                        change: [-1.5364% -0.4029% +0.7357%] (p = 0.51 > 0.05)

```
# Usage for Tantivy

use [tantivy-ik](https://github.com/blueshen/tantivy-ik) project


---
Welcome to rust developer and search engine developer join us, and maintain this project together!

you can PR or submit issue...

and star⭐️ or fork this project to support me!