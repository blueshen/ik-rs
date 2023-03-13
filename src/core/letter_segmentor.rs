use crate::core::char_util::utf8_len;
use crate::core::char_util::CharType;
use crate::core::lexeme::{Lexeme, LexemeType};
use crate::core::ordered_linked_list::OrderedLinkedList;
use crate::core::segmentor::Segmenter;

const SEGMENTER_NAME: &str = "LETTER_SEGMENTER";

const LETTER_CONNECTOR: [char; 7] = ['#', '&', '+', '-', '.', '@', '_'];

const NUM_CONNECTOR: [char; 2] = [',', '.'];

pub struct LetterSegmenter {
    start: Option<usize>,
    end: Option<usize>,

    english_start: Option<usize>,
    english_end: Option<usize>,

    arabic_start: Option<usize>,
    arabic_end: Option<usize>,
}

impl Segmenter for LetterSegmenter {
    fn analyze(
        &mut self,
        input: &str,
        cursor: usize,
        curr_char_type: CharType,
        origin_lexemes: &mut OrderedLinkedList<Lexeme>,
    ) {
        self.process_english_letter(input, cursor, curr_char_type, origin_lexemes);
        self.process_arabic_letter(input, cursor, curr_char_type, origin_lexemes);
        self.process_mix_letter(input, cursor, curr_char_type, origin_lexemes);
    }
    fn name(&self) -> &str {
        return SEGMENTER_NAME;
    }
}

impl LetterSegmenter {
    pub fn new() -> Self {
        LetterSegmenter {
            start: None,
            end: None,
            english_start: None,
            english_end: None,
            arabic_start: None,
            arabic_end: None,
        }
    }

    /// mix letter
    /// windows2000 | zhiyi.shen@gmail.com
    fn process_mix_letter(
        &mut self,
        input: &str,
        cursor: usize,
        curr_char_type: CharType,
        origin_lexemes: &mut OrderedLinkedList<Lexeme>,
    ) {
        let curr_char = input.chars().nth(cursor).unwrap();
        let char_count = utf8_len(input);
        match self.start {
            None => {
                if CharType::ARABIC == curr_char_type || CharType::ENGLISH == curr_char_type {
                    self.start = Some(cursor);
                    self.end = Some(cursor);
                }
            }
            Some(_) => {
                if CharType::ARABIC == curr_char_type || CharType::ENGLISH == curr_char_type {
                    self.end = Some(cursor);
                } else if CharType::USELESS == curr_char_type && self.is_letter_connector(curr_char)
                {
                    self.end = Some(cursor);
                } else {
                    let new_lexeme = Lexeme::new(
                        (self.start.unwrap())..(self.end.unwrap() + 1),
                        LexemeType::LETTER,
                    );
                    origin_lexemes.insert(new_lexeme);
                    self.reset_mix_state();
                }
            }
        }

        if let Some(index) = self.end {
            if index == (char_count - 1) {
                let new_lexeme = Lexeme::new(
                    (self.start.unwrap())..(self.end.unwrap() + 1),
                    LexemeType::LETTER,
                );
                origin_lexemes.insert(new_lexeme);
                self.reset_mix_state();
            }
        }
    }

    // english
    fn process_english_letter(
        &mut self,
        input: &str,
        cursor: usize,
        curr_char_type: CharType,
        origin_lexemes: &mut OrderedLinkedList<Lexeme>,
    ) {
        let char_count = utf8_len(input);
        match self.english_start {
            None => match curr_char_type {
                CharType::ENGLISH => {
                    self.english_start = Some(cursor);
                    self.english_end = Some(cursor);
                }
                _ => {}
            },
            Some(_) => match curr_char_type {
                CharType::ENGLISH => {
                    self.english_end = Some(cursor);
                }
                _ => {
                    let new_lexeme = Lexeme::new(
                        (self.english_start.unwrap())..(self.english_end.unwrap() + 1),
                        LexemeType::ENGLISH,
                    );
                    origin_lexemes.insert(new_lexeme);
                    self.reset_english_state();
                }
            },
        }

        if let Some(index) = self.english_end {
            if index == (char_count - 1) {
                let new_lexeme = Lexeme::new(
                    (self.english_start.unwrap())..(self.english_end.unwrap() + 1),
                    LexemeType::ENGLISH,
                );
                origin_lexemes.insert(new_lexeme);
                self.reset_english_state();
            }
        }
    }

    // arabic
    fn process_arabic_letter(
        &mut self,
        input: &str,
        cursor: usize,
        curr_char_type: CharType,
        origin_lexemes: &mut OrderedLinkedList<Lexeme>,
    ) {
        let curr_char = input.chars().nth(cursor).unwrap();
        match self.arabic_start {
            None => match curr_char_type {
                CharType::ARABIC => {
                    self.arabic_start = Some(cursor);
                    self.arabic_end = Some(cursor);
                }
                _ => {}
            },
            Some(_) => {
                if CharType::ARABIC == curr_char_type {
                    self.arabic_end = Some(cursor);
                } else if CharType::USELESS == curr_char_type && self.is_num_connector(curr_char) {
                    // do nothing
                } else {
                    let new_lexeme = Lexeme::new(
                        (self.arabic_start.unwrap())..(self.arabic_end.unwrap() + 1),
                        LexemeType::ARABIC,
                    );
                    origin_lexemes.insert(new_lexeme);
                    self.reset_arabic_state();
                }
            }
        }
        let char_count = utf8_len(input);
        if let Some(index) = self.arabic_end {
            if index == (char_count - 1) {
                let new_lexeme = Lexeme::new(
                    (self.arabic_start.unwrap())..(self.arabic_end.unwrap() + 1),
                    LexemeType::ARABIC,
                );
                origin_lexemes.insert(new_lexeme);
                self.reset_arabic_state();
            }
        }
    }
    fn reset_mix_state(&mut self) {
        self.start = None;
        self.end = None;
    }

    fn reset_english_state(&mut self) {
        self.english_start = None;
        self.english_end = None;
    }

    fn reset_arabic_state(&mut self) {
        self.arabic_start = None;
        self.arabic_end = None;
    }

    fn is_letter_connector(&self, input: char) -> bool {
        LETTER_CONNECTOR.contains(&input)
    }

    fn is_num_connector(&self, input: char) -> bool {
        NUM_CONNECTOR.contains(&input)
    }
}
