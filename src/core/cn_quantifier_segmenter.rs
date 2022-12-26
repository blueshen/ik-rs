use crate::core::char_util::utf8_len;
use crate::core::char_util::CharType;
use crate::core::lexeme::{Lexeme, LexemeType};
use crate::core::ordered_linked_list::OrderedLinkedList;
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
    fn analyze(
        &mut self,
        input: &str,
        cursor: usize,
        curr_char_type: CharType,
        origin_lexemes: &mut OrderedLinkedList<Lexeme>,
    ) {
        self.process_cnumber(input, cursor, curr_char_type, origin_lexemes);
        self.process_count(input, cursor, curr_char_type, origin_lexemes);
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

    fn process_cnumber(
        &mut self,
        input: &str,
        cursor: usize,
        curr_char_type: CharType,
        origin_lexemes: &mut OrderedLinkedList<Lexeme>,
    ) {
        let curr_char = input.chars().nth(cursor).unwrap();
        let char_count = utf8_len(input);
        if self.initial_state() {
            if CharType::CHINESE == curr_char_type && self.chn_number_chars.contains(&curr_char) {
                self.start = cursor as i32;
                self.end = cursor as i32;
            }
        } else {
            if CharType::CHINESE == curr_char_type && self.chn_number_chars.contains(&curr_char) {
                self.end = cursor as i32;
            } else {
                let new_lexeme = Lexeme::new(
                    (self.start as usize)..(self.end + 1) as usize,
                    LexemeType::CNUM,
                );
                origin_lexemes.insert(new_lexeme);
                self.reset_state();
            }
        }
        if self.end == (char_count - 1) as i32 {
            let new_lexeme = Lexeme::new(
                (self.start as usize)..(self.end + 1) as usize,
                LexemeType::CNUM,
            );
            origin_lexemes.insert(new_lexeme);
            self.reset_state();
        }
    }

    fn process_count(
        &mut self,
        input: &str,
        cursor: usize,
        curr_char_type: CharType,
        origin_lexemes: &mut OrderedLinkedList<Lexeme>,
    ) {
        if self.need_count_scan(cursor, origin_lexemes) {
            let char_count = utf8_len(input);
            if CharType::CHINESE == curr_char_type {
                let hit_options = GLOBAL_DICT.lock().unwrap().match_in_quantifier_dict(
                    input,
                    cursor,
                    char_count - cursor,
                );
                for hit in hit_options.iter() {
                    if hit.is_match() {
                        let new_lexeme = Lexeme::new(hit.pos.clone(), LexemeType::COUNT);
                        origin_lexemes.insert(new_lexeme);
                    }
                }
            }
        }
    }

    fn need_count_scan(
        &self,
        cursor: usize,
        origin_lexemes: &mut OrderedLinkedList<Lexeme>,
    ) -> bool {
        if self.start != -1 && self.end != -1 {
            return true;
        }
        if origin_lexemes.is_empty() {
            return false;
        } else {
            let last = origin_lexemes.peek_back();
            if let Some(lexeme) = last {
                if lexeme.lexeme_type == LexemeType::ARABIC
                    || lexeme.lexeme_type == LexemeType::CNUM
                {
                    if lexeme.get_begin() + lexeme.get_length() == cursor {
                        return true;
                    }
                }
            }
            return false;
        }
    }

    fn initial_state(&self) -> bool {
        self.start == -1 && self.end == -1
    }

    fn reset_state(&mut self) {
        self.start = -1;
        self.end = -1;
    }
}
