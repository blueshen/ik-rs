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
use crate::dict::dictionary::GLOBAL_DICT;
use std::collections::{HashMap, LinkedList};

// ik main class
pub struct IKSegmenter {
    input: String,
    segmenters: Vec<Box<dyn Segmenter>>,
    // 分词歧义裁决器
    arbitrator: IKArbitrator,
    use_smart: bool,
    // 原始分词结果集合，未经歧义处理
    origin_lexemes: OrderedLinkedList<Lexeme>,
    // LexemePath位置索引表
    path_map: HashMap<usize, LexemePath>,
    // 最终分词结果集
    results: LinkedList<Lexeme>,
    seg: bool,
}

impl IKSegmenter {
    /*
     *   非智能分词：细粒度输出所有可能的切分结果
     *   智能分词： 合并数词和量词，对分词结果进行歧义判断
     */
    pub fn new(input: &str, use_smart: bool) -> Self {
        let regular_input = regularize_str(input);
        let ik = IKSegmenter {
            input: regular_input,
            arbitrator: IKArbitrator::new(),
            segmenters: vec![
                Box::new(LetterSegmenter::new()),
                Box::new(CnQuantifierSegmenter::new()),
                Box::new(CJKSegmenter::new()),
            ],
            use_smart,
            origin_lexemes: OrderedLinkedList::new(),
            path_map: HashMap::new(),
            results: LinkedList::new(),
            seg: false,
        };
        ik
    }

    /**
     * 分词，获取下一个词元
     * make sync todo!
     */
    pub fn next(&mut self) -> Option<Lexeme> {
        if !self.seg {
            //遍历子分词器
            for segmenter in self.segmenters.iter_mut() {
                println!("sub segmenter->{}", segmenter.name());
                let lexemes = segmenter.analyze(self.input.as_str());
                for lexeme in lexemes {
                    self.origin_lexemes.insert(lexeme).expect("error!");
                }
            }
            //对分词进行歧义处理
            unsafe {
                self.path_map = self
                    .arbitrator
                    .process(&mut self.origin_lexemes, self.use_smart);
            }
            // for (k,v) in self.path_map.iter(){
            //     println!("k={}, v={}", k, v);
            // }
            //将分词结果输出到结果集，并处理未切分的单个CJK字符
            self.output_to_result();
            self.seg = true;
        }
        let l = self.get_next_lexeme();
        l
    }

    /**
     * 推送分词结果到结果集合
     * 1.从buff头部遍历到self.cursor已处理位置
     * 2.将map中存在的分词结果推入results
     * 3.将map中不存在的CJDK字符以单字方式推入results
     */
    pub fn output_to_result(&mut self) {
        let mut index = 0;
        let char_count = self.input.chars().count();
        while index < char_count {
            let curr_char = self.input.chars().nth(index).unwrap();
            let cur_char_type = char_type_of(curr_char);
            //跳过非CJK字符
            if CharType::USELESS == cur_char_type {
                index += 1;
                continue;
            }
            //从pathMap找出对应index位置的LexemePath
            let mut path = self.path_map.get_mut(&index);
            if path.is_some() {
                //输出LexemePath中的lexeme到results集合
                let mut l = path.as_mut().unwrap().poll_first();
                while l.is_some() {
                    let l_value = l.as_ref().unwrap();
                    self.results.push_back(l_value.clone());
                    //将index移至lexeme后
                    index = l_value.get_begin() + l_value.get_length();
                    l = path.as_mut().unwrap().poll_first();
                    if l.is_some() {
                        let new_l_value = l.as_ref().unwrap();
                        //输出path内部，词元间遗漏的单字
                        while index < new_l_value.get_begin() {
                            let curr_char = self.input.chars().nth(index).unwrap();
                            let cur_char_type = char_type_of(curr_char);
                            if CharType::CHINESE == cur_char_type {
                                let single_char_lexeme =
                                    Lexeme::new(0, index, 1, LexemeType::CNCHAR);
                                self.results.push_back(single_char_lexeme);
                            } else if CharType::OtherCjk == cur_char_type {
                                let single_char_lexeme =
                                    Lexeme::new(0, index, 1, LexemeType::OtherCJK);
                                self.results.push_back(single_char_lexeme);
                            }
                            index += 1;
                        }
                    }
                }
            } else {
                //pathMap中找不到index对应的LexemePath, 单字输出
                let curr_char = self.input.chars().nth(index).unwrap();
                let cur_char_type = char_type_of(curr_char);
                if CharType::CHINESE == cur_char_type {
                    let single_char_lexeme = Lexeme::new(0, index, 1, LexemeType::CNCHAR);
                    self.results.push_back(single_char_lexeme);
                } else if CharType::OtherCjk == cur_char_type {
                    let single_char_lexeme = Lexeme::new(0, index, 1, LexemeType::OtherCJK);
                    self.results.push_back(single_char_lexeme);
                }
                index += 1;
            }
        }
        self.path_map.clear();
    }

    // 返回lexeme, 同时处理合并
    pub fn get_next_lexeme(&mut self) -> Option<Lexeme> {
        //从结果集取出，并移除第一个Lexeme
        let mut result = self.results.pop_front();
        let mut result_value;
        while result.is_some() {
            //数量词合并
            result_value = result.as_mut().unwrap();
            self.compound(result_value);
            if GLOBAL_DICT.lock().unwrap().is_stop_word(
                &self.input,
                result_value.get_begin(),
                result_value.get_length(),
            ) {
                //是停止词继续取列表的下一个
                result = self.results.pop_front();
            } else {
                //不是停止词, 生成lexeme的词元文本,输出
                result_value.parse_lexeme_text(&self.input);
                return Some(result_value.clone());
            }
        }
        None
    }

    // 组合词元
    pub fn compound(&mut self, result: &mut Lexeme) {
        if !self.use_smart {
            return;
        }
        //数量词合并处理
        if !self.results.is_empty() {
            if LexemeType::ARABIC == result.lexeme_type {
                let next_lexeme = self.results.front();
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
                    self.results.pop_front();
                }
            }
            //可能存在第二轮合并
            if LexemeType::CNUM == result.lexeme_type && !self.results.is_empty() {
                let next_lexeme = self.results.front(); // p peekFirst();
                let mut append_ok = false;
                if LexemeType::COUNT == next_lexeme.unwrap().lexeme_type {
                    //合并中文数词+中文量词
                    append_ok = result.append(next_lexeme.unwrap(), LexemeType::CQUAN);
                }
                if append_ok {
                    self.results.pop_front();
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_segment() {
        let smart = false;
        let texts = _get_input_texts();
        for text in texts {
            let mut ik = IKSegmenter::new(text, smart);
            let mut token = ik.next();
            while token.is_some() {
                println!("{:?}", token.unwrap());
                token = ik.next();
            }
            println!("{}", "----------------------")
        }
    }

    #[test]
    fn test_smart_segment() {
        let smart = true;
        let texts = _get_input_texts();
        for text in texts {
            let mut ik = IKSegmenter::new(text, smart);
            let mut token = ik.next();
            while token.is_some() {
                println!("{:?}", token.unwrap());
                token = ik.next();
            }
            println!("{}", "----------------------")
        }
    }

    fn _get_input_texts() -> Vec<&'static str> {
        let texts = vec![
            "张三说的确实在理",
            "中华人民共和国",
            "zhiyi.shen@gmail.com",
            "我感觉很happy,并且不悲伤!",
        ];
        texts
    }
}
