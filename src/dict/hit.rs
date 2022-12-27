use std::ops::Range;

const UNMATCH: u32 = 0x00000000;
const MATCH: u32 = 0x00000001;
const PREFIX: u32 = 0x00000010;

#[derive(Debug, Default, Clone)]
pub struct Hit {
    pub hit_state: u32,
    pub pos: Range<usize>,
}

impl Hit {
    pub fn new() -> Self {
        Hit {
            hit_state: UNMATCH,
            pos: 0..0,
        }
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
    pub fn set_unmatch(&mut self) {
        self.hit_state = UNMATCH;
    }
    #[allow(dead_code)]
    pub fn is_unmatch(&self) -> bool {
        self.hit_state == UNMATCH
    }
    #[allow(dead_code)]
    pub fn length(&self) -> usize {
        self.pos.len()
    }
}
