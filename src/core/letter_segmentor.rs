use crate::core::char_util::char_type_of;
use crate::core::char_util::CharType;
use crate::core::lexeme::{Lexeme, LexemeType};
use crate::core::segmentor::Segmenter;
use crate::core::char_util::utf8_len;

//子分词器标签
const SEGMENTER_NAME: &str = "LETTER_SEGMENTER";
//链接符号
const LETTER_CONNECTOR: [char; 7] = ['#', '&', '+', '-', '.', '@', '_'];

//数字符号
const NUM_CONNECTOR: [char; 2] = [',', '.'];

// 英文字符及阿拉伯数字子分词器
pub struct LetterSegmenter {
    /**
     * 词元的开始位置，
     * 同时作为子分词器状态标识
     * 当start > -1 时，标识当前的分词器正在处理字符
     */
    start: i32,
    /**
     * 记录词元结束位置
     * end记录的是在词元中最后一个出现的Letter但非Sign_Connector的字符的位置
     */
    end: i32,

    // 字母起始位置
    english_start: i32,
    // 字母结束位置
    english_end: i32,

    // 阿拉伯数字起始位置
    arabic_start: i32,
    // 阿拉伯数字结束位置
    arabic_end: i32,
}

impl Segmenter for LetterSegmenter {
    fn analyze(&mut self, input: &str) -> Vec<Lexeme> {
        let mut new_lexemes = Vec::new();
        //处理英文字母
        let mut a = self.process_english_letter(input);
        //处理阿拉伯字母
        let mut b = self.process_arabic_letter(input);
        //处理混合字母(这个要放最后处理，可以通过QuickSortSet排除重复)
        let mut c = self.process_mix_letter(input);
        new_lexemes.append(&mut a);
        new_lexemes.append(&mut b);
        new_lexemes.append(&mut c);
        new_lexemes
    }
    fn name(&self) -> &str {
        return SEGMENTER_NAME;
    }
}

impl LetterSegmenter {
    pub fn new() -> Self {
        LetterSegmenter {
            start: -1,
            end: -1,
            english_start: -1,
            english_end: -1,
            arabic_start: -1,
            arabic_end: -1,
        }
    }

    /// 处理数字字母混合输出
    /// 如：windos2000 | zhiyi.shen@gmail.com
    pub fn process_mix_letter(&mut self, input: &str) -> Vec<Lexeme> {
        let mut new_lexemes = Vec::new();
        let char_count = utf8_len(input);
        for cursor in 0..char_count {
            let curr_char = input.char_indices().nth(cursor).unwrap().1;
            let curr_char_type = char_type_of(curr_char);
            if self.start == -1 {
                //当前的分词器尚未开始处理字符
                if CharType::ARABIC == curr_char_type || CharType::ENGLISH == curr_char_type {
                    //记录起始指针的位置,标明分词器进入处理状态
                    self.start = cursor as i32;
                    self.end = self.start;
                }
            } else {
                //当前的分词器正在处理字符
                if CharType::ARABIC == curr_char_type || CharType::ENGLISH == curr_char_type {
                    //记录下可能的结束位置
                    self.end = cursor as i32;
                } else if CharType::USELESS == curr_char_type && self.is_letter_connector(curr_char)
                {
                    //记录下可能的结束位置
                    self.end = cursor as i32;
                } else {
                    //遇到非Letter字符，输出词元
                    let new_lexeme = Lexeme::new(
                        0,
                        self.start as usize,
                        (self.end - self.start + 1) as usize,
                        LexemeType::LETTER,
                    );
                    new_lexemes.push(new_lexeme);
                    self.start = -1;
                    self.end = -1;
                }
            }
        }

        if self.end == (char_count - 1) as i32 {
            let new_lexeme = Lexeme::new(
                0,
                self.start as usize,
                (self.end - self.start + 1) as usize,
                LexemeType::LETTER,
            );
            new_lexemes.push(new_lexeme);
            self.start = -1;
            self.end = -1;
        }
        new_lexemes
    }

    // 处理纯英文字母输出
    fn process_english_letter(&mut self, input: &str) -> Vec<Lexeme> {
        let mut new_lexemes = Vec::new();
        let char_count = utf8_len(input);
        for cursor in 0..char_count {
            let curr_char = input.char_indices().nth(cursor).unwrap().1;
            let curr_char_type = char_type_of(curr_char);
            if self.english_start == -1 {
                //当前的分词器尚未开始处理英文字符
                if CharType::ENGLISH == curr_char_type {
                    //记录起始指针的位置,标明分词器进入处理状态
                    self.english_start = cursor as i32;
                    self.english_end = self.english_start;
                }
            } else {
                //当前的分词器正在处理英文字符
                if CharType::ENGLISH == curr_char_type {
                    //记录当前指针位置为结束位置
                    self.english_end = cursor as i32;
                } else {
                    //遇到非English字符,输出词元
                    let new_lexeme = Lexeme::new(
                        0,
                        self.english_start as usize,
                        (self.english_end - self.english_start + 1) as usize,
                        LexemeType::ENGLISH,
                    );
                    new_lexemes.push(new_lexeme);
                    self.english_start = -1;
                    self.english_end = -1;
                }
            }
        }
        // 结束了
        if self.english_end == (char_count - 1) as i32 {
            let new_lexeme = Lexeme::new(
                0,
                self.english_start as usize,
                (self.english_end - self.english_start + 1) as usize,
                LexemeType::ENGLISH,
            );
            new_lexemes.push(new_lexeme);
            self.english_start = -1;
            self.english_end = -1;
        }
        new_lexemes
    }

    /**
     * 处理阿拉伯数字输出
     */
    fn process_arabic_letter(&mut self, input: &str) -> Vec<Lexeme> {
        let mut new_lexemes = Vec::new();
        let char_count = utf8_len(input);
        for cursor in 0..char_count {
            let curr_char = input.chars().nth(cursor).unwrap();
            let curr_char_type = char_type_of(curr_char);
            if self.arabic_start == -1 {
                //当前的分词器尚未开始处理数字字符
                if CharType::ARABIC == curr_char_type {
                    //记录起始指针的位置,标明分词器进入处理状态
                    self.arabic_start = cursor as i32;
                    self.arabic_end = self.arabic_start;
                }
            } else {
                //当前的分词器正在处理数字字符
                if CharType::ARABIC == curr_char_type {
                    //记录当前指针位置为结束位置
                    self.arabic_end = cursor as i32;
                } else if CharType::USELESS == curr_char_type && self.is_num_connector(curr_char) {
                    //不输出数字，但不标记结束
                } else {
                    // 遇到非Arabic字符,输出词元
                    let new_lexeme = Lexeme::new(
                        0,
                        self.arabic_start as usize,
                        (self.arabic_end - self.arabic_start + 1) as usize,
                        LexemeType::ARABIC,
                    );
                    new_lexemes.push(new_lexeme);
                    self.arabic_start = -1;
                    self.arabic_end = -1;
                }
            }
        }
        if self.arabic_end == (char_count - 1) as i32 {
            let new_lexeme = Lexeme::new(
                0,
                self.arabic_start as usize,
                (self.arabic_end - self.arabic_start + 1) as usize,
                LexemeType::ARABIC,
            );
            new_lexemes.push(new_lexeme);
            self.arabic_start = -1;
            self.arabic_end = -1;
        }
        new_lexemes
    }

    // 判断是否是字母连接符号
    pub fn is_letter_connector(&self, input: char) -> bool {
        LETTER_CONNECTOR.contains(&input)
    }

    // 判断是否是数字连接符号
    pub fn is_num_connector(&self, input: char) -> bool {
        NUM_CONNECTOR.contains(&input)
    }
}
