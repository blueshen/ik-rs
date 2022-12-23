use crate::core::char_util::CharType;
use crate::core::lexeme::Lexeme;
use crate::core::ordered_linked_list::OrderedLinkedList;

pub trait Segmenter {
    fn analyze(
        &mut self,
        input: &str,
        cursor: usize,
        curr_char_type: CharType,
        origin_lexemes: &mut OrderedLinkedList<Lexeme>,
    );
    fn name(&self) -> &str;
}
