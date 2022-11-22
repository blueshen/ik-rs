mod tests {
    use ik_rs::core::ik_segmenter::TokenMode;
    use ik_rs::IkTokenizer;
    use tantivy::schema::{IndexRecordOption, Schema, TextFieldIndexing, TextOptions};
    use tantivy::Index;

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
