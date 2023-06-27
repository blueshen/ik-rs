use crate::core::char_util::utf8_slice;
use std::cmp::Ordering;
use std::ops::Range;

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

impl LexemeType {
    pub fn as_str(&self) -> &str {
        match self {
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
}

#[derive(Debug)]
pub struct Lexeme {
    offset: usize, // maybe use later, current default = 0
    pos: Range<usize>,
    lexeme_text: String,
    pub(crate) lexeme_type: LexemeType,
}

impl Clone for Lexeme {
    fn clone(&self) -> Self {
        Self {
            offset: self.offset,
            pos: self.pos.clone(),
            lexeme_text: self.lexeme_text.clone(),
            lexeme_type: self.lexeme_type.clone(),
        }
    }
}

impl PartialEq for Lexeme {
    fn eq(&self, other: &Self) -> bool {
        self.offset == other.offset && self.pos == other.pos
    }
}

impl PartialOrd for Lexeme {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return if self.begin_pos() < other.begin_pos() {
            Some(Ordering::Less)
        } else if self.begin_pos() == other.begin_pos() {
            if self.pos.len() > other.pos.len() {
                Some(Ordering::Less)
            } else if self.pos.len() == other.pos.len() {
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
    pub fn new(pos: Range<usize>, lexeme_type: LexemeType) -> Self {
        Lexeme {
            offset: 0,
            pos,
            lexeme_type,
            lexeme_text: String::from(""),
        }
    }

    pub fn begin_pos(&self) -> usize {
        self.offset + self.pos.start
    }

    pub fn end_pos(&self) -> usize {
        self.offset + self.pos.end
    }

    pub fn len(&self) -> usize {
        self.pos.len()
    }

    pub fn lexeme_text(&self) -> &str {
        &self.lexeme_text
    }

    pub fn parse_lexeme_text(&mut self, input: &str) {
        let sub_text = utf8_slice(input, self.begin_pos(), self.end_pos());
        self.lexeme_text = sub_text.to_string();
    }

    pub fn append(&mut self, l: &Lexeme, lexeme_type: LexemeType) -> bool {
        if self.end_pos() == l.begin_pos() {
            self.pos.end = l.pos.end;
            self.lexeme_type = lexeme_type;
            return true;
        }
        return false;
    }
}
