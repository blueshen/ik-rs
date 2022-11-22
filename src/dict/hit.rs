const UNMATCH: u32 = 0x00000000;
const MATCH: u32 = 0x00000001;
const PREFIX: u32 = 0x00000010;

#[derive(Debug, Default, Clone)]
pub struct Hit {
    pub hit_state: u32,
    pub begin: usize,
    pub end: usize,
}

impl Hit {
    pub fn new() -> Self {
        Hit {
            hit_state: UNMATCH,
            begin: 0,
            end: 0,
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
    pub fn is_prefix(&self) -> bool {
        (self.hit_state & PREFIX) > 0
    }

    pub fn set_unmatch(&mut self) {
        self.hit_state = UNMATCH;
    }
    pub fn is_unmatch(&self) -> bool {
        self.hit_state == UNMATCH
    }
    pub fn length(&self) -> usize {
        self.end - self.begin + 1
    }
}
