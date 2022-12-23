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
                result_value.get_begin(),
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
                let mut l = p.poll_first();
                while let Some(ref l_value) = l {
                    results.push_back(l_value.clone());
                    index = l_value.get_begin() + l_value.get_length();
                    l = p.poll_first();
                    if let Some(ref new_l_value) = l {
                        while index < new_l_value.get_begin() {
                            let curr_char = input.chars().nth(index).unwrap();
                            let cur_char_type = char_type_of(curr_char);
                            match cur_char_type {
                                CharType::CHINESE => {
                                    let single_char_lexeme =
                                        Lexeme::new(0, index, 1, LexemeType::CNCHAR);
                                    results.push_back(single_char_lexeme);
                                }
                                CharType::OtherCjk => {
                                    let single_char_lexeme =
                                        Lexeme::new(0, index, 1, LexemeType::OtherCJK);
                                    results.push_back(single_char_lexeme);
                                }
                                _ => {}
                            }
                            index += 1;
                        }
                    }
                }
            } else {
                match cur_char_type {
                    CharType::CHINESE => {
                        let single_char_lexeme = Lexeme::new(0, index, 1, LexemeType::CNCHAR);
                        results.push_back(single_char_lexeme);
                    }
                    CharType::OtherCjk => {
                        let single_char_lexeme = Lexeme::new(0, index, 1, LexemeType::OtherCJK);
                        results.push_back(single_char_lexeme);
                    }
                    _ => {}
                }
                index += 1;
            }
        }
        results
    }

    fn compound(&mut self, results: &mut LinkedList<Lexeme>, result: &mut Lexeme) {
        if !results.is_empty() {
            if LexemeType::ARABIC == result.lexeme_type {
                let next_lexeme = results.front();
                let mut append_ok = false;
                if LexemeType::CNUM == next_lexeme.unwrap().lexeme_type {
                    append_ok = result.append(next_lexeme.unwrap(), LexemeType::CNUM);
                } else if LexemeType::COUNT == next_lexeme.unwrap().lexeme_type {
                    append_ok = result.append(next_lexeme.unwrap(), LexemeType::CQUAN);
                }
                if append_ok {
                    results.pop_front();
                }
            }

            if LexemeType::CNUM == result.lexeme_type && !results.is_empty() {
                let next_lexeme = results.front();
                let mut append_ok = false;
                if LexemeType::COUNT == next_lexeme.unwrap().lexeme_type {
                    append_ok = result.append(next_lexeme.unwrap(), LexemeType::CQUAN);
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

    #[test]
    fn test_index_segment() {
        let mut ik = IKSegmenter::new();
        let texts = _get_input_texts();
        for text in texts {
            let tokens = ik.tokenize(text, TokenMode::INDEX);
            for token in tokens.iter() {
                println!("{:?}", token);
            }
            println!("{}", "----------------------")
        }
    }

    #[test]
    fn test_search_segment() {
        let mut ik = IKSegmenter::new();
        let texts = _get_input_texts();
        for text in texts {
            let tokens = ik.tokenize(text, TokenMode::SEARCH);
            for token in tokens.iter() {
                println!("{:?}", token);
            }
            println!("{}", "----------------------")
        }
    }

    fn _get_input_texts() -> Vec<&'static str> {
        let texts = vec![
            // "张三说的确实在理",
            // "中华人民共和国",
            // "zhiyi.shen@gmail.com",
            // "我感觉很happy,并且不悲伤!",
            // "结婚的和尚未结婚的",
            // "中国有960万平方公里的国土",
            "我的年纪是十八",
        ];
        texts
    }
}
