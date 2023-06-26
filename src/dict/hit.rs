use std::ops::Range;

const UNMATCH: u8 = 0b00000000;
const MATCH: u8 = 0b00000001;
const PREFIX: u8 = 0b00000010;

#[derive(Debug, Clone)]
pub struct Hit {
    pub hit_state: u8,
    pub pos: Range<usize>,
}

impl Default for Hit {
    fn default() -> Self {
        Hit {
            hit_state: UNMATCH,
            pos: 0..0,
        }
    }
}

impl Hit {
    pub fn new_with_pos(pos: Range<usize>) -> Self {
        let mut hit = Hit::default();
        hit.pos = pos;
        hit
    }

    pub fn set_match(&mut self) {
        self.hit_state = self.hit_state | MATCH;
    }
    pub fn is_match(&self) -> bool {
        (self.hit_state & MATCH) > 0
    }

    pub fn set_prefix(&mut self) {
        self.hit_state = self.hit_state | PREFIX;
    }
    #[allow(dead_code)]
    pub fn is_prefix(&self) -> bool {
        (self.hit_state & PREFIX) > 0
    }

    #[allow(dead_code)]
    pub fn length(&self) -> usize {
        self.pos.len()
    }
}
