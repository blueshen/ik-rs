#[cfg(test)]
mod test {
    use ik_rs::core::ik_segmenter::{IKSegmenter, TokenMode};
    use once_cell::sync::Lazy;
    use std::sync::Mutex;

    pub static GLOBAL_IK: Lazy<Mutex<IKSegmenter>> = Lazy::new(|| {
        let ik = IKSegmenter::new();
        Mutex::new(ik)
    });

    #[test]
    fn test_index_segment() {
        assert_index_token(
            "张三说的确实在理",
            vec!["张三", "三", "说的", "的确", "的", "确实", "实在", "在理"],
        );
        assert_index_token(
            "中华人民共和国",
            vec![
                "中华人民共和国",
                "中华人民",
                "中华",
                "华人",
                "人民共和国",
                "人民",
                "共和国",
                "共和",
                "国",
            ],
        );
        assert_index_token(
            "zhiyi.shen@gmail.com",
            vec!["zhiyi.shen@gmail.com", "zhiyi", "shen", "gmail", "com"],
        );
        assert_index_token(
            "我感觉很happy,并且不悲伤!",
            vec!["我", "感觉", "很", "happy", "并且", "且不", "悲伤"],
        );
        assert_index_token(
            "结婚的和尚未结婚的",
            vec!["结婚", "的", "和尚", "尚未", "未结", "结婚", "的"],
        );
        assert_index_token(
            "中国有960万平方公里的国土",
            vec![
                "中国",
                "国有",
                "有",
                "960",
                "万",
                "平方公里",
                "平方",
                "方公里",
                "公里",
                "的",
                "国土",
            ],
        );
    }

    #[test]
    fn test_search_segment() {
        assert_search_token("张三说的确实在理", vec!["张三", "说的", "确实", "在理"]);
        assert_search_token("中华人民共和国", vec!["中华人民共和国"]);
        assert_search_token("zhiyi.shen@gmail.com", vec!["zhiyi.shen@gmail.com"]);
        // ik is not perfect
        assert_search_token(
            "我感觉很happy,并且不悲伤!",
            vec!["我", "感觉", "很", "happy", "并", "且不", "悲伤"],
        );
        assert_search_token(
            "结婚的和尚未结婚的",
            vec!["结婚", "的", "和尚", "未", "结婚", "的"],
        );
        // quantifier token
        assert_search_token(
            "中国有960万平方公里的国土",
            vec!["中国", "有", "960万平方公里", "的", "国土"],
        );
    }
    // SEARCH Mode
    fn assert_search_token(text: &str, expect: Vec<&str>) {
        let tokens = GLOBAL_IK.lock().unwrap().tokenize(text, TokenMode::SEARCH);
        let mut token_texts = Vec::new();
        for token in tokens.iter() {
            // println!("{:?}", token);
            token_texts.push(token.get_lexeme_text());
        }
        assert_eq!(expect, token_texts);
    }

    // INDEX Mode
    fn assert_index_token(text: &str, expect: Vec<&str>) {
        let tokens = GLOBAL_IK.lock().unwrap().tokenize(text, TokenMode::INDEX);
        let mut token_texts = Vec::new();
        for token in tokens.iter() {
            // println!("{:?}", token);
            token_texts.push(token.get_lexeme_text());
        }
        assert_eq!(expect, token_texts);
    }
}
