use crate::core::lexeme::Lexeme;

pub trait Segmenter {
    fn analyze(&mut self, input: &str) -> Vec<Lexeme>;
    fn name(&self) -> &str;
}
