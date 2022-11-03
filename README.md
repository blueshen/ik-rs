<!-- Badges section here. -->
[![Open Source Love](https://badges.frapsoft.com/os/v1/open-source.svg?v=103)](https://github.com/blueshen/ik-rs/releases)
[![Crates.io](https://img.shields.io/badge/license-lgpl__2__1-blue)](./LICENSE)
[![Crates.io](https://img.shields.io/badge/ik--rs-0.1.1-green)](https://crates.io/crates/ik-rs)
[![Build Status](https://travis-ci.org/blueshen/ik-rs.svg)](https://travis-ci.org/blueshen/ik-rs)

[![GitHub forks](https://img.shields.io/github/forks/blueshen/ik-rs.svg?style=social&label=Fork)](https://github.com/blueshen/ik-rs/network/members)
[![GitHub stars](https://img.shields.io/github/stars/blueshen/ik-rs.svg?style=social&label=Star)](https://github.com/blueshen/ik-rs/stargazers)
<!-- /Badges section end. -->

# ik-rs

[ik-analyzer](https://github.com/blueshen/ik-analyzer) for Rust

support [Tantivy](https://github.com/quickwit-oss/tantivy)


# Usage

## Chinese Segment
```rust
    let mut ik = IKSegmenter::new();
    let text = "中华人民共和国";
    let tokens = ik.tokenize(text, TokenMode::SEARCH); // TokenMode::INDEX
    for token in tokens {
        println!("{:?}", token);
    }
```

# Usage for Tantivy
todo
```rust

mod tests {
    use ik_rs::core::ik_segmenter::TokenMode;
    use ik_rs::IkTokenizer;
    use tantivy::Index;
    use tantivy::schema::{IndexRecordOption, Schema, TextFieldIndexing, TextOptions};

    #[test]
    fn it_works() {
        let mut schema_builder = Schema::builder();
        let text_field_indexing = TextFieldIndexing::default()
            .set_tokenizer("ik-index")
            .set_index_option(IndexRecordOption::WithFreqsAndPositions);
        let text_options = TextOptions::default()
            .set_indexing_options(text_field_indexing)
            .set_stored();
        schema_builder.add_text_field("title", text_options);
        let schema = schema_builder.build();
        let index = Index::create_in_ram(schema.clone());
        index
            .tokenizers()
            .register("ik-index", IkTokenizer::new(TokenMode::INDEX));
        index
            .tokenizers()
            .register("ik-search", IkTokenizer::new(TokenMode::SEARCH));
    }
}
```