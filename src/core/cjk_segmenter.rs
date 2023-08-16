use crate::core::char_util::utf8_len;
use crate::core::char_util::CharType;
use crate::core::lexeme::{Lexeme, LexemeType};
use crate::core::ordered_linked_list::OrderedLinkedList;
use crate::core::segmentor::Segmenter;
use crate::dict::dictionary::GLOBAL_DICT;

const SEGMENTER_NAME: &str = "CJK_SEGMENTER";

#[derive(Default, Debug)]
pub struct CJKSegmenter {}

impl Segmenter for CJKSegmenter {
    fn analyze(
        &mut self,
        input: &str,
        cursor: usize,
        curr_char_type: &CharType,
        origin_lexemes: &mut OrderedLinkedList<Lexeme>,
    ) {
        match curr_char_type {
            CharType::USELESS => {}
            _ => {
                let char_count = utf8_len(input);
                let lock_guard = {cfg_if::cfg_if! {
                    if #[cfg(feature="use-parking-lot")] {GLOBAL_DICT.read()}
                    else /*if #[cfg(feature="use-std-sync")]*/ {
                        match GLOBAL_DICT.read() {
                            Err(_err) => return,
                            Ok(lck) => lck
                        }
                    }
                }};
                let hits = lock_guard.match_in_main_dict_with_offset(
                    input,
                    cursor,
                    char_count - cursor
                );
                for hit in hits.iter() {
                    if hit.is_match() {
                        let new_lexeme = Lexeme::new(hit.pos(), LexemeType::CNWORD);
                        origin_lexemes.insert(new_lexeme);
                    }
                }
            }
        }
    }

    fn name(&self) -> &str {
        return SEGMENTER_NAME;
    }
}
