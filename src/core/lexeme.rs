use crate::core::char_util::utf8_slice;
use std::cmp::Ordering;

#[derive(Debug, PartialEq, Clone)]
pub enum LexemeType {
    UNKNOWN,
    ENGLISH,
    ARABIC,
    LETTER,
    CNWORD,
    CNCHAR,
    OtherCJK,
    CNUM,
    COUNT,
    CQUAN,
}

#[derive(Debug)]
pub struct Lexeme {
    // TODO(blueshen) maybe use later, current default = 0
    offset: usize,
    begin: usize,
    length: usize,
    lexeme_text: String,
    pub(crate) lexeme_type: LexemeType,
}

impl Clone for Lexeme {
    fn clone(&self) -> Self {
        Self {
            offset: self.offset,
            begin: self.begin,
            length: self.length,
            lexeme_text: self.lexeme_text.clone(),
            lexeme_type: self.lexeme_type.clone(),
        }
    }
}

impl PartialEq for Lexeme {
    fn eq(&self, other: &Self) -> bool {
        self.offset == other.offset && self.begin == other.begin && self.length == other.length
    }
}

impl PartialOrd for Lexeme {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return if self.begin < other.begin {
            Some(Ordering::Less)
        } else if self.begin == other.begin {
            if self.length > other.length {
                Some(Ordering::Less)
            } else if self.length == other.length {
                Some(Ordering::Equal)
            } else {
                Some(Ordering::Greater)
            }
        } else {
            Some(Ordering::Greater)
        };
    }
}

impl Lexeme {
    pub fn new(offset: usize, begin: usize, length: usize, lexeme_type: LexemeType) -> Self {
        Lexeme {
            offset,
            begin,
            length,
            lexeme_type,
            lexeme_text: String::from(""),
        }
    }

    pub fn get_begin(&self) -> usize {
        self.begin
    }

    pub fn get_begin_position(&self) -> usize {
        self.offset + self.begin
    }

    pub fn get_end_position(&self) -> usize {
        self.get_begin_position() + self.length
    }

    pub fn get_length(&self) -> usize {
        self.length
    }

    pub fn set_length(&mut self, length: usize) {
        self.length = length;
    }

    pub fn get_lexeme_text(&self) -> &str {
        &self.lexeme_text
    }

    pub fn parse_lexeme_text(&mut self, input: &str) {
        let sub_text = utf8_slice(input, self.begin, self.begin + self.length);
        self.lexeme_text = sub_text.to_string();
    }

    pub fn get_lexeme_type_string(&self) -> &str {
        match &self.lexeme_type {
            LexemeType::ENGLISH => "ENGLISH",
            LexemeType::ARABIC => "ARABIC",
            LexemeType::LETTER => "LETTER",
            LexemeType::CNWORD => "CN_WORD",
            LexemeType::CNCHAR => "CN_CHAR",
            LexemeType::OtherCJK => "OtherCjk",
            LexemeType::COUNT => "COUNT",
            LexemeType::CNUM => "TYPE_CNUM",
            LexemeType::CQUAN => "TYPE_CQUAN",
            _ => "UNKNOW",
        }
    }

    pub fn append(&mut self, l: &Lexeme, lexeme_type: LexemeType) -> bool {
        if self.get_end_position() == l.get_begin_position() {
            self.length += l.get_length();
            self.lexeme_type = lexeme_type;
            return true;
        }
        return false;
    }
}
