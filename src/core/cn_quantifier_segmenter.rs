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
    start: Option<usize>,
    end: Option<usize>,
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

impl Default for CnQuantifierSegmenter {
    fn default() -> Self {
        CnQuantifierSegmenter {
            start: None,
            end: None,
            chn_number_chars: HashSet::from([
                '一', '二', '两', '三', '四', '五', '六', '七', '八', '九', '十', '零', '壹', '贰',
                '叁', '肆', '伍', '陆', '柒', '捌', '玖', '拾', '百', '千', '万', '亿', '拾', '佰',
                '仟', '萬', '億', '兆', '卅', '廿',
            ]),
        }
    }
}

impl CnQuantifierSegmenter {
    fn process_cnumber(
        &mut self,
        input: &str,
        cursor: usize,
        curr_char_type: CharType,
        origin_lexemes: &mut OrderedLinkedList<Lexeme>,
    ) {
        let curr_char = &input.chars().nth(cursor).unwrap();
        let char_count = utf8_len(input);
        if self.initial_state() {
            match curr_char_type {
                CharType::CHINESE if self.chn_number_chars.contains(&curr_char) => {
                    self.start = Some(cursor);
                    self.end = Some(cursor);
                }
                _ => {}
            }
        } else {
            if curr_char_type == CharType::CHINESE && self.chn_number_chars.contains(&curr_char) {
                self.end = Some(cursor);
            } else {
                let new_lexeme = Lexeme::new(
                    (self.start.unwrap())..(self.end.unwrap() + 1),
                    LexemeType::CNUM,
                );
                origin_lexemes.insert(new_lexeme);
                self.reset_state();
            }
        }
        if let Some(index) = self.end {
            if index == (char_count - 1) {
                let new_lexeme = Lexeme::new((self.start.unwrap())..index + 1, LexemeType::CNUM);
                origin_lexemes.insert(new_lexeme);
                self.reset_state();
            }
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
            match curr_char_type {
                CharType::CHINESE => {
                    let hits = GLOBAL_DICT.lock().unwrap().match_in_quantifier_dict(
                        input,
                        cursor,
                        char_count - cursor,
                    );
                    for hit in hits.iter() {
                        if hit.is_match() {
                            let new_lexeme = Lexeme::new(hit.pos(), LexemeType::COUNT);
                            origin_lexemes.insert(new_lexeme);
                        }
                    }
                }
                _ => {}
            }
        }
    }

    fn need_count_scan(
        &self,
        cursor: usize,
        origin_lexemes: &mut OrderedLinkedList<Lexeme>,
    ) -> bool {
        if self.start.is_some() && self.end.is_some() {
            return true;
        }
        if origin_lexemes.empty() {
            return false;
        }
        let last = origin_lexemes.peek_back();
        if let Some(lexeme) = last {
            if lexeme.lexeme_type == LexemeType::ARABIC || lexeme.lexeme_type == LexemeType::CNUM {
                if lexeme.end_pos() == cursor {
                    return true;
                }
            }
        }
        return false;
    }

    fn initial_state(&self) -> bool {
        self.start.is_none() && self.end.is_none()
    }

    fn reset_state(&mut self) {
        self.start = None;
        self.end = None;
    }
}
