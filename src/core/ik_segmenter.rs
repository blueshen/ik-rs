use crate::core::char_util::char_type_of;
use crate::core::char_util::regularize_str;
use crate::core::char_util::CharType;
use crate::core::cjk_segmenter::CJKSegmenter;
use crate::core::cn_quantifier_segmenter::CnQuantifierSegmenter;
use crate::core::ik_arbitrator::IKArbitrator;
use crate::core::letter_segmentor::LetterSegmenter;
use crate::core::lexeme::{Lexeme, LexemeType};
use crate::core::lexeme_path::LexemePath;
use crate::core::ordered_linked_list::OrderedLinkedList;
use crate::core::segmentor::Segmenter;
use crate::dict::dictionary::GLOBAL_DICT;
use std::collections::{HashMap, LinkedList};

#[derive(Clone, Copy, PartialEq)]
pub enum TokenMode {
    INDEX,
    SEARCH,
}
// ik main class
pub struct IKSegmenter {
    segmenters: Vec<Box<dyn Segmenter>>,
    arbitrator: IKArbitrator,
}

unsafe impl Send for IKSegmenter {}
unsafe impl Sync for IKSegmenter {}

impl IKSegmenter {
    pub fn new() -> Self {
        let ik = IKSegmenter {
            arbitrator: IKArbitrator::new(),
            segmenters: vec![
                Box::new(LetterSegmenter::new()),
                Box::new(CnQuantifierSegmenter::new()),
                Box::new(CJKSegmenter::new()),
            ],
        };
        ik
    }

    pub fn tokenize(&mut self, text: &str, mode: TokenMode) -> Vec<Lexeme> {
        let regular_str = regularize_str(text);
        let input = regular_str.as_str();
        let mut origin_lexemes = OrderedLinkedList::<Lexeme>::new();
        for (cursor, curr_char) in input.chars().enumerate() {
            let curr_char_type = char_type_of(curr_char);
            for segmenter in self.segmenters.iter_mut() {
                segmenter.analyze(input, cursor, curr_char_type, &mut origin_lexemes);
            }
        }

        let mut path_map = self.arbitrator.process(&mut origin_lexemes, mode);
        let mut results = self.output_to_result(&mut path_map, input);
        let mut final_results = Vec::new();
        // remove stop word
        let mut result = results.pop_front();
        while let Some(ref mut result_value) = result {
            if mode == TokenMode::SEARCH {
                self.compound(&mut results, result_value);
            }
            if !GLOBAL_DICT.lock().unwrap().is_stop_word(
                input,
                result_value.get_begin_position(),
                result_value.get_length(),
            ) {
                result_value.parse_lexeme_text(input);
                final_results.push(result_value.clone())
            }
            result = results.pop_front();
        }
        final_results
    }

    fn output_to_result(
        &mut self,
        path_map: &mut HashMap<usize, LexemePath>,
        input: &str,
    ) -> LinkedList<Lexeme> {
        let mut results = LinkedList::new();
        let mut index = 0usize;
        let char_count = input.chars().count();
        while index < char_count {
            let curr_char = input.chars().nth(index).unwrap();
            let cur_char_type = char_type_of(curr_char);
            if CharType::USELESS == cur_char_type {
                index += 1;
                continue;
            }
            let path = path_map.get_mut(&index);
            if let Some(p) = path {
                let mut cur_lexeme = p.poll_first();
                while let Some(ref lexeme) = cur_lexeme {
                    results.push_back(lexeme.clone());
                    index = lexeme.get_end_position();
                    cur_lexeme = p.poll_first();
                    if let Some(ref lexeme) = cur_lexeme {
                        while index < lexeme.get_begin_position() {
                            let curr_char = input.chars().nth(index).unwrap();
                            let cur_char_type = char_type_of(curr_char);
                            self.add_single_lexeme(&mut results, cur_char_type, index);
                            index += 1;
                        }
                    }
                }
            } else {
                self.add_single_lexeme(&mut results, cur_char_type, index);
                index += 1;
            }
        }
        results
    }

    fn add_single_lexeme(
        &self,
        results: &mut LinkedList<Lexeme>,
        cur_char_type: CharType,
        index: usize,
    ) {
        match cur_char_type {
            CharType::CHINESE => {
                let single_char_lexeme = Lexeme::new(index..index + 1, LexemeType::CNCHAR);
                results.push_back(single_char_lexeme);
            }
            CharType::OtherCjk => {
                let single_char_lexeme = Lexeme::new(index..index + 1, LexemeType::OtherCJK);
                results.push_back(single_char_lexeme);
            }
            _ => {}
        }
    }

    fn compound(&mut self, results: &mut LinkedList<Lexeme>, result: &mut Lexeme) {
        if !results.is_empty() {
            if LexemeType::ARABIC == result.lexeme_type {
                let mut append_ok = false;
                let next_lexeme = results.front();
                if let Some(next) = next_lexeme {
                    if LexemeType::CNUM == next.lexeme_type {
                        append_ok = result.append(next, LexemeType::CNUM);
                    } else if LexemeType::COUNT == next.lexeme_type {
                        append_ok = result.append(next, LexemeType::CQUAN);
                    }
                }
                if append_ok {
                    results.pop_front();
                }
            }

            if LexemeType::CNUM == result.lexeme_type && !results.is_empty() {
                let mut append_ok = false;
                let next_lexeme = results.front();
                if let Some(next) = next_lexeme {
                    if LexemeType::COUNT == next.lexeme_type {
                        append_ok = result.append(next, LexemeType::CQUAN);
                    }
                }
                if append_ok {
                    results.pop_front();
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use log;

    #[test]
    fn test_index_segment() {
        let mut ik = IKSegmenter::new();
        let texts = _get_input_texts();
        for text in texts {
            let tokens = ik.tokenize(text, TokenMode::INDEX);
            for token in tokens.iter() {
                log::info!("{:?}", token);
            }
            log::info!("{}", "----------------------")
        }
    }

    #[test]
    fn test_search_segment() {
        let mut ik = IKSegmenter::new();
        let texts = _get_input_texts();
        for text in texts {
            let tokens = ik.tokenize(text, TokenMode::SEARCH);
            for token in tokens.iter() {
                log::info!("{:?}", token);
            }
            log::info!("{}", "----------------------")
        }
    }

    fn _get_input_texts() -> Vec<&'static str> {
        let texts = vec![
            "张三说的确实在理",
            "中华人民共和国",
            "zhiyi.shen@gmail.com",
            "我感觉很happy,并且不悲伤!",
            "结婚的和尚未结婚的",
            "中国有960万平方公里的国土",
            "我的年纪是十八",
        ];
        texts
    }
}
