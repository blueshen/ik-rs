use crate::core::lexeme::Lexeme;
use crate::core::ordered_linked_list::{Link, OrderedLinkedList};
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};

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
            payload_length: 0usize,
            lexeme_list: OrderedLinkedList::new(),
        }
    }

    pub fn add_cross_lexeme(&mut self, lexeme: &Lexeme) -> bool {
        return if self.lexeme_list.empty() {
            self.lexeme_list.insert(lexeme.clone());
            self.path_begin = lexeme.begin_position() as i32;
            self.path_end = lexeme.end_position() as i32;
            self.payload_length += lexeme.length();
            true
        } else if self.check_cross(&lexeme) {
            self.lexeme_list.insert(lexeme.clone());
            if lexeme.end_position() > self.path_end() as usize {
                self.path_end = lexeme.end_position() as i32;
            }
            self.payload_length = self.path_length();
            true
        } else {
            false
        };
    }

    pub fn add_not_cross_lexeme(&mut self, lexeme: &Lexeme) -> bool {
        return if self.lexeme_list.empty() {
            self.lexeme_list.insert(lexeme.clone());
            self.path_begin = lexeme.begin_position() as i32;
            self.path_end = lexeme.end_position() as i32;
            self.payload_length += lexeme.length();
            true
        } else if self.check_cross(lexeme) {
            false
        } else {
            self.lexeme_list.insert(lexeme.clone());
            self.payload_length += lexeme.length();
            let head = self.lexeme_list.peek_front(); //  peekFirst();
            if let Some(h) = head {
                self.path_begin = h.begin_position() as i32;
            }
            let tail = self.lexeme_list.peek_back(); //  peekLast();
            if let Some(t) = tail {
                self.path_end = t.end_position() as i32;
            }
            true
        };
    }

    pub fn remove_tail(&mut self) -> Option<Lexeme> {
        let tail = self.lexeme_list.pop_back();
        if self.lexeme_list.empty() {
            self.path_begin = -1;
            self.path_end = -1;
            self.payload_length = 0usize;
        } else {
            self.payload_length -= tail.as_ref().unwrap().length();
            let new_tail = self.lexeme_list.peek_back();
            if let Some(new) = new_tail {
                self.path_end = new.end_position() as i32;
            }
        }
        return tail;
    }

    pub fn check_cross(&self, lexeme: &Lexeme) -> bool {
        let l_begin = lexeme.begin_position();
        let l_length = lexeme.length();
        let cross = (l_begin >= self.path_begin() as usize && l_begin < self.path_end() as usize)
            || (self.path_begin() as usize >= l_begin
                && (self.path_begin() as usize) < (l_begin + l_length));
        cross
    }

    pub fn path_begin(&self) -> i32 {
        self.path_begin
    }

    pub fn path_end(&self) -> i32 {
        self.path_end
    }

    pub fn path_length(&self) -> usize {
        (self.path_end - self.path_begin) as usize
    }

    pub fn payload_length(&self) -> usize {
        self.payload_length
    }

    pub fn x_weight(&self) -> usize {
        let mut product = 1;
        for lexeme in self.lexeme_list.iter() {
            product *= lexeme.length();
        }
        return product;
    }

    pub fn p_weight(&self) -> usize {
        let mut p_weight = 0;
        let mut p = 0;
        for lexeme in self.lexeme_list.iter() {
            p += 1;
            p_weight += p * lexeme.length();
        }
        return p_weight;
    }

    pub fn size(&self) -> usize {
        self.lexeme_list.length()
    }

    pub fn poll_first(&mut self) -> Option<Lexeme> {
        self.lexeme_list.pop_front()
    }

    pub fn head_node(&self) -> Option<&Link<Lexeme>> {
        self.lexeme_list.head_node()
    }
}

impl Display for LexemePath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "path_begin:{}, path_end:{}, payload_length:{}, lexeme_list:{}",
            self.path_begin(),
            self.path_end(),
            self.payload_length(),
            self.lexeme_list
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
        if self.payload_length() > o.payload_length() {
            return Some(Ordering::Less);
        } else if self.payload_length() < o.payload_length() {
            return Some(Ordering::Greater);
        } else {
            if self.size() < o.size() {
                return Some(Ordering::Less);
            } else if self.size() > o.size() {
                return Some(Ordering::Greater);
            } else {
                if self.path_length() > o.path_length() {
                    return Some(Ordering::Less);
                } else if self.path_length() < o.path_length() {
                    return Some(Ordering::Greater);
                } else {
                    if self.path_end() > o.path_end() {
                        return Some(Ordering::Less);
                    } else if self.path_end() < o.path_end() {
                        return Some(Ordering::Greater);
                    } else {
                        if self.x_weight() > o.x_weight() {
                            return Some(Ordering::Less);
                        } else if self.x_weight() < o.x_weight() {
                            return Some(Ordering::Greater);
                        } else {
                            if self.p_weight() > o.p_weight() {
                                return Some(Ordering::Less);
                            } else if self.p_weight() < o.p_weight() {
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
        return if self.path_begin() == other.path_begin()
            && self.path_end() == other.path_end()
            && self.payload_length == other.payload_length
            && self.lexeme_list.length() == other.lexeme_list.length()
        {
            for (a, b) in self.lexeme_list.iter().zip(other.lexeme_list.iter()) {
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
