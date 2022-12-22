use crate::core::char_util::char_type_of;
use crate::core::char_util::utf8_len;
use crate::core::char_util::CharType;
use crate::core::lexeme::{Lexeme, LexemeType};
use crate::core::segmentor::Segmenter;
use crate::dict::dictionary::GLOBAL_DICT;
use std::collections::HashSet;

const SEGMENTER_NAME: &str = "QUAN_SEGMENTER";

#[derive(Debug)]
pub struct CnQuantifierSegmenter {
    start: i32,
    end: i32,
    chn_number_chars: HashSet<char>,
}

impl Segmenter for CnQuantifierSegmenter {
    fn analyze(&mut self, input: &str) -> Vec<Lexeme> {
        let mut new_lexemes: Vec<Lexeme> = Vec::new();
        let mut cnum_lexemes = self.process_cnumber(input);
        new_lexemes.append(&mut cnum_lexemes);
        let mut cquan_lexemes = self.process_count(input);
        new_lexemes.append(&mut cquan_lexemes);
        new_lexemes
    }
    fn name(&self) -> &str {
        return SEGMENTER_NAME;
    }
}

impl CnQuantifierSegmenter {
    pub fn new() -> Self {
        CnQuantifierSegmenter {
            start: -1,
            end: -1,
            chn_number_chars: HashSet::from([
                '一', '二', '两', '三', '四', '五', '六', '七', '八', '九', '十', '零', '壹', '贰',
                '叁', '肆', '伍', '陆', '柒', '捌', '玖', '拾', '百', '千', '万', '亿', '拾', '佰',
                '仟', '萬', '億', '兆', '卅', '廿',
            ]),
        }
    }

    fn process_cnumber(&mut self, input: &str) -> Vec<Lexeme> {
        let mut new_lexemes = Vec::new();
        for (cursor, curr_char) in input.chars().enumerate() {
            let curr_char_type = char_type_of(curr_char);
            if self.initial_state() {
                if CharType::CHINESE == curr_char_type && self.chn_number_chars.contains(&curr_char)
                {
                    self.start = cursor as i32;
                    self.end = cursor as i32;
                }
            } else {
                if CharType::CHINESE == curr_char_type && self.chn_number_chars.contains(&curr_char)
                {
                    self.end = cursor as i32;
                } else {
                    let new_lexeme = Lexeme::new(
                        0,
                        self.start as usize,
                        (self.end - self.start + 1) as usize,
                        LexemeType::CNUM,
                    );
                    new_lexemes.push(new_lexeme);
                    self.reset_state();
                }
            }

            if self.start != -1 && self.end != -1 {
                let new_lexeme = Lexeme::new(
                    0,
                    self.start as usize,
                    (self.end - self.start + 1) as usize,
                    LexemeType::CNUM,
                );
                new_lexemes.push(new_lexeme);
                self.reset_state();
            }
        }
        new_lexemes
    }

    fn process_count(&mut self, input: &str) -> Vec<Lexeme> {
        let mut new_lexemes = Vec::new();
        if self.need_count_scan() {
            let char_count = utf8_len(input);
            for (cursor, curr_char) in input.chars().enumerate() {
                let curr_char_type = char_type_of(curr_char);
                if CharType::CHINESE == curr_char_type {
                    let hit_options = GLOBAL_DICT.lock().unwrap().match_in_quantifier_dict(
                        input,
                        cursor,
                        char_count - cursor,
                    );
                    for hit in hit_options.iter() {
                        if hit.is_match() {
                            let new_lexeme = Lexeme::new(
                                0,
                                hit.begin,
                                hit.end - hit.begin + 1,
                                LexemeType::COUNT,
                            );
                            new_lexemes.push(new_lexeme);
                        }
                    }
                }
            }
        }
        new_lexemes
    }

    fn need_count_scan(&self) -> bool {
        if self.initial_state() {
            return false;
        }
        // TODO(blueshen) check if previous lexeme is CNUM or ARABIC
        // maybe should merge letter_segmentor + cn_quantifier segmentor
        return true;
    }

    fn initial_state(&self) -> bool {
        self.start == -1 && self.end == -1
    }

    fn reset_state(&mut self) {
        self.start = -1;
        self.end = -1;
    }
}
