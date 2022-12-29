use crate::core::lexeme::Lexeme;
use crate::core::ordered_linked_list::{Node, OrderedLinkedList};
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::ptr::NonNull;

pub struct LexemePath {
    pub path_begin: i32,
    pub path_end: i32,
    pub payload_length: usize,
    pub lexeme_list: OrderedLinkedList<Lexeme>,
}

impl LexemePath {
    pub fn new() -> Self {
        LexemePath {
            path_begin: -1,
            path_end: -1,
            payload_length: 0,
            lexeme_list: OrderedLinkedList::new(),
        }
    }

    pub fn add_cross_lexeme(&mut self, lexeme: &Lexeme) -> bool {
        return if self.lexeme_list.is_empty() {
            self.lexeme_list.insert(lexeme.clone());
            self.path_begin = lexeme.get_begin() as i32;
            self.path_end = lexeme.get_end_position() as i32;
            self.payload_length += lexeme.get_length();
            true
        } else if self.check_cross(&lexeme) {
            self.lexeme_list.insert(lexeme.clone());
            if lexeme.get_end_position() as i32 > self.path_end {
                self.path_end = lexeme.get_end_position() as i32;
            }
            self.payload_length = (self.path_end - self.path_begin) as usize;
            true
        } else {
            false
        };
    }

    pub fn add_not_cross_lexeme(&mut self, lexeme: &Lexeme) -> bool {
        return if self.lexeme_list.is_empty() {
            self.lexeme_list.insert(lexeme.clone());
            self.path_begin = lexeme.get_begin() as i32;
            self.path_end = lexeme.get_end_position() as i32;
            self.payload_length += lexeme.get_length();
            true
        } else if self.check_cross(lexeme) {
            false
        } else {
            self.lexeme_list.insert(lexeme.clone());
            self.payload_length += lexeme.get_length();
            let head = self.lexeme_list.peek_front(); //  peekFirst();
            if let Some(h) = head {
                self.path_begin = h.get_begin() as i32;
            }
            let tail = self.lexeme_list.peek_back(); //  peekLast();
            if let Some(t) = tail {
                self.path_end = t.get_end_position() as i32;
            }
            true
        };
    }

    pub fn remove_tail(&mut self) -> Option<Lexeme> {
        let tail = self.lexeme_list.pop_back();
        if self.lexeme_list.is_empty() {
            self.path_begin = -1;
            self.path_end = -1;
            self.payload_length = 0;
        } else {
            self.payload_length -= tail.as_ref().unwrap().get_length();
            let new_tail = self.lexeme_list.peek_back();
            if let Some(new) = new_tail {
                self.path_end = new.get_begin() as i32 + new.get_length() as i32;
            }
        }
        return tail;
    }

    pub fn check_cross(&self, lexeme: &Lexeme) -> bool {
        let l_begin = lexeme.get_begin() as i32;
        let l_length = lexeme.get_length() as i32;
        let cross = (l_begin >= self.path_begin && l_begin < self.path_end)
            || (self.path_begin >= l_begin && self.path_begin < l_begin + l_length);
        cross
    }

    pub fn get_path_begin(&self) -> i32 {
        self.path_begin
    }

    // pub fn get_path_end(&self) -> i32 {
    //     self.path_end
    // }

    // pub fn get_payload_length(&self) -> usize {
    //     self.payload_length
    // }

    pub fn get_path_length(&self) -> usize {
        (self.path_end - self.path_begin) as usize
    }

    pub fn get_xweight(&self) -> i32 {
        let mut product = 1;
        for lexeme in self.lexeme_list.iter() {
            product *= lexeme.get_length();
        }
        return product as i32;
    }

    pub fn get_pweight(&self) -> i32 {
        let mut p_weight = 0;
        let mut p = 0;
        for lexeme in self.lexeme_list.iter() {
            p += 1;
            p_weight += p * lexeme.get_length();
        }
        return p_weight as i32;
    }

    pub fn size(&self) -> usize {
        self.lexeme_list.length()
    }

    pub fn poll_first(&mut self) -> Option<Lexeme> {
        self.lexeme_list.pop_front()
    }

    pub fn get_head(&self) -> Option<&NonNull<Node<Lexeme>>> {
        self.lexeme_list.head_node()
    }
}

impl Display for LexemePath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "path_begin:{}, path_end:{}, payload_length:{}, lexeme_list:{}",
            self.path_begin, self.path_end, self.payload_length, self.lexeme_list
        )
    }
}

impl Clone for LexemePath {
    fn clone(&self) -> Self {
        let mut the_copy = LexemePath::new();
        the_copy.path_begin = self.path_begin;
        the_copy.path_end = self.path_end;
        the_copy.payload_length = self.payload_length;
        for lexeme in self.lexeme_list.iter() {
            the_copy.lexeme_list.insert(lexeme.clone());
        }
        return the_copy;
    }
}

impl Ord for LexemePath {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd<Self> for LexemePath {
    fn partial_cmp(&self, o: &Self) -> Option<Ordering> {
        if self.payload_length > o.payload_length {
            return Some(Ordering::Less);
        } else if self.payload_length < o.payload_length {
            return Some(Ordering::Greater);
        } else {
            if self.size() < o.size() {
                return Some(Ordering::Less);
            } else if self.size() > o.size() {
                return Some(Ordering::Greater);
            } else {
                if self.get_path_length() > o.get_path_length() {
                    return Some(Ordering::Less);
                } else if self.get_path_length() < o.get_path_length() {
                    return Some(Ordering::Greater);
                } else {
                    if self.path_end > o.path_end {
                        return Some(Ordering::Less);
                    } else if self.path_end < o.path_end {
                        return Some(Ordering::Greater);
                    } else {
                        if self.get_xweight() > o.get_xweight() {
                            return Some(Ordering::Less);
                        } else if self.get_xweight() < o.get_xweight() {
                            return Some(Ordering::Greater);
                        } else {
                            if self.get_pweight() > o.get_pweight() {
                                return Some(Ordering::Less);
                            } else if self.get_pweight() < o.get_pweight() {
                                return Some(Ordering::Greater);
                            }
                        }
                    }
                }
            }
        }
        return Some(Ordering::Equal);
    }
}

impl Eq for LexemePath {}
impl PartialEq for LexemePath {
    fn eq(&self, other: &Self) -> bool {
        return if self.path_begin == other.path_begin
            && self.path_end == other.path_end
            && self.payload_length == other.payload_length
            && self.lexeme_list.length() == other.lexeme_list.length()
        {
            for _ in 0..self.lexeme_list.length() {
                let a = self.lexeme_list.iter().next().unwrap();
                let b = other.lexeme_list.iter().next().unwrap();
                if !a.eq(b) {
                    return false;
                }
            }
            true
        } else {
            false
        };
    }
}
