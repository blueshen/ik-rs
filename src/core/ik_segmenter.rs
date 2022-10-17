use crate::core::char_util::char_type_of;
use crate::core::char_util::CharType;
use crate::core::char_util::regularize_str;
use crate::core::cjk_segmenter::CJKSegmenter;
use crate::core::cn_quantifier_segmenter::CnQuantifierSegmenter;
use crate::core::ik_arbitrator::IKArbitrator;
use crate::core::letter_segmentor::LetterSegmenter;
use crate::core::lexeme::{Lexeme, LexemeType};
use crate::core::lexeme_path::LexemePath;
use crate::core::ordered_linked_list::OrderedLinkedList;
use crate::core::segmentor::Segmenter;
use crate::dict::dictionary::{GLOBAL_DICT};
use std::collections::{HashMap, LinkedList};

#[derive(Clone, Copy, PartialEq)]
pub enum TokenMode{
    INDEX,
    SEARCH
}
// ik main class
pub struct IKSegmenter {
    segmenters: Vec<Box<dyn Segmenter>>,
    arbitrator: IKArbitrator,
}

unsafe impl Sync for IKSegmenter {}
unsafe impl Send for IKSegmenter {}

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
        //遍历子分词器
        let mut origin_lexemes = OrderedLinkedList::new();
        for segmenter in self.segmenters.iter_mut() {
            println!("sub segmenter->{}", segmenter.name());
            let lexemes = segmenter.analyze(input);
            for lexeme in lexemes {
                origin_lexemes.insert(lexeme).expect("error!");
            }
        }
        let mut path_map = HashMap::new();
        //对分词进行歧义处理
        unsafe {
            path_map = self
                .arbitrator
                .process(&mut origin_lexemes, mode);
        }
        //将分词结果输出到结果集，并处理未切分的单个CJK字符
        let mut results = self.output_to_result(&mut path_map, input);
        let mut final_results = Vec::new();
        // remove stop word
        let mut result = results.pop_front();
        let mut result_value;
        while result.is_some() {
            //数量词合并
            result_value = result.as_mut().unwrap();
            if mode == TokenMode::SEARCH {
                self.compound(&mut results, result_value);
            }
            if !GLOBAL_DICT.lock().unwrap().is_stop_word(
                input,
                result_value.get_begin(),
                result_value.get_length(),
            ) {
                //不是停止词, 生成lexeme的词元文本,输出
                result_value.parse_lexeme_text(input);
                final_results.push(result_value.clone())
            }
            result = results.pop_front();
        }
        final_results
    }

    /**
     * 推送分词结果到结果集合
     * 1.从buff头部遍历到self.cursor已处理位置
     * 2.将map中存在的分词结果推入results
     * 3.将map中不存在的CJDK字符以单字方式推入results
     */
    pub fn output_to_result(&mut self, path_map: &mut HashMap<usize, LexemePath>, input: &str) -> LinkedList<Lexeme> {
        let mut results = LinkedList::new();
        let mut index = 0usize;
        let char_count = input.chars().count();
        while index < char_count {
            let curr_char = input.chars().nth(index).unwrap();
            let cur_char_type = char_type_of(curr_char);
            //跳过非CJK字符
            if CharType::USELESS == cur_char_type {
                index += 1;
                continue;
            }
            //从pathMap找出对应index位置的LexemePath
            let mut path = path_map.get_mut(&index);
            if path.is_some() {
                //输出LexemePath中的lexeme到results集合
                let mut l = path.as_mut().unwrap().poll_first();
                while l.is_some() {
                    let l_value = l.as_ref().unwrap();
                    results.push_back(l_value.clone());
                    //将index移至lexeme后
                    index = l_value.get_begin() + l_value.get_length();
                    l = path.as_mut().unwrap().poll_first();
                    if l.is_some() {
                        let new_l_value = l.as_ref().unwrap();
                        //输出path内部，词元间遗漏的单字
                        while index < new_l_value.get_begin() {
                            let curr_char = input.chars().nth(index).unwrap();
                            let cur_char_type = char_type_of(curr_char);
                            if CharType::CHINESE == cur_char_type {
                                let single_char_lexeme =
                                    Lexeme::new(0, index, 1, LexemeType::CNCHAR);
                                results.push_back(single_char_lexeme);
                            } else if CharType::OtherCjk == cur_char_type {
                                let single_char_lexeme =
                                    Lexeme::new(0, index, 1, LexemeType::OtherCJK);
                                results.push_back(single_char_lexeme);
                            }
                            index += 1;
                        }
                    }
                }
            } else {
                //pathMap中找不到index对应的LexemePath, 单字输出
                let curr_char = input.chars().nth(index).unwrap();
                let cur_char_type = char_type_of(curr_char);
                if CharType::CHINESE == cur_char_type {
                    let single_char_lexeme = Lexeme::new(0, index, 1, LexemeType::CNCHAR);
                    results.push_back(single_char_lexeme);
                } else if CharType::OtherCjk == cur_char_type {
                    let single_char_lexeme = Lexeme::new(0, index, 1, LexemeType::OtherCJK);
                    results.push_back(single_char_lexeme);
                }
                index += 1;
            }
        }
        results
    }

    // 组合词元
    pub fn compound(&mut self, results: &mut LinkedList<Lexeme>, result: &mut Lexeme) {
        //数量词合并处理
        if !results.is_empty() {
            if LexemeType::ARABIC == result.lexeme_type {
                let next_lexeme = results.front();
                let mut append_ok = false;
                if LexemeType::CNUM == next_lexeme.unwrap().lexeme_type {
                    //合并英文数词+中文数词
                    append_ok = result.append(next_lexeme.unwrap(), LexemeType::CNUM);
                } else if LexemeType::COUNT == next_lexeme.unwrap().lexeme_type {
                    //合并英文数词+中文量词
                    append_ok = result.append(next_lexeme.unwrap(), LexemeType::CQUAN);
                }
                if append_ok {
                    //弹出
                    results.pop_front();
                }
            }
            //可能存在第二轮合并
            if LexemeType::CNUM == result.lexeme_type && !results.is_empty() {
                let next_lexeme = results.front(); // p peekFirst();
                let mut append_ok = false;
                if LexemeType::COUNT == next_lexeme.unwrap().lexeme_type {
                    //合并中文数词+中文量词
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
            let mut tokens = ik.tokenize(text, TokenMode::INDEX);
            for token in tokens {
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
            for token in tokens {
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
            "结婚的和尚未结婚的",
        ];
        texts
    }
}
