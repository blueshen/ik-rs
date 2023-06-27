use crate::core::lexeme::Lexeme;
use crate::core::ordered_linked_list::{Link, OrderedLinkedList};
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};

pub struct LexemePath {
    begin: i32,
    end: i32,
    payload_len: usize,
    pub lexeme_list: OrderedLinkedList<Lexeme>,
}

impl LexemePath {
    pub fn new() -> Self {
        LexemePath {
            begin: -1,
            end: -1,
            payload_len: 0usize,
            lexeme_list: OrderedLinkedList::new(),
        }
    }

    pub fn add_cross_lexeme(&mut self, lexeme: &Lexeme) -> bool {
        return if self.lexeme_list.empty() {
            self.lexeme_list.insert(lexeme.clone());
            self.begin = lexeme.begin_pos() as i32;
            self.end = lexeme.end_pos() as i32;
            self.payload_len += lexeme.len();
            true
        } else if self.check_cross(&lexeme) {
            self.lexeme_list.insert(lexeme.clone());
            if lexeme.end_pos() > self.end() as usize {
                self.end = lexeme.end_pos() as i32;
            }
            self.payload_len = self.path_len();
            true
        } else {
            false
        };
    }

    pub fn add_not_cross_lexeme(&mut self, lexeme: &Lexeme) -> bool {
        return if self.lexeme_list.empty() {
            self.lexeme_list.insert(lexeme.clone());
            self.begin = lexeme.begin_pos() as i32;
            self.end = lexeme.end_pos() as i32;
            self.payload_len += lexeme.len();
            true
        } else if self.check_cross(lexeme) {
            false
        } else {
            self.lexeme_list.insert(lexeme.clone());
            self.payload_len += lexeme.len();
            let head = self.lexeme_list.peek_front(); //  peekFirst();
            if let Some(h) = head {
                self.begin = h.begin_pos() as i32;
            }
            let tail = self.lexeme_list.peek_back(); //  peekLast();
            if let Some(t) = tail {
                self.end = t.end_pos() as i32;
            }
            true
        };
    }

    pub fn remove_tail(&mut self) -> Option<Lexeme> {
        let tail = self.lexeme_list.pop_back();
        if self.lexeme_list.empty() {
            self.begin = -1;
            self.end = -1;
            self.payload_len = 0usize;
        } else {
            self.payload_len -= tail.as_ref().unwrap().len();
            let new_tail = self.lexeme_list.peek_back();
            if let Some(new) = new_tail {
                self.end = new.end_pos() as i32;
            }
        }
        return tail;
    }

    pub fn check_cross(&self, lexeme: &Lexeme) -> bool {
        let begin = lexeme.begin_pos();
        let length = lexeme.len();
        let cross = (begin >= self.begin() as usize && begin < self.end() as usize)
            || (self.begin() as usize >= begin && (self.begin() as usize) < (begin + length));
        cross
    }

    pub fn begin(&self) -> i32 {
        self.begin
    }

    pub fn end(&self) -> i32 {
        self.end
    }

    pub fn path_len(&self) -> usize {
        (self.end - self.begin) as usize
    }

    pub fn payload_len(&self) -> usize {
        self.payload_len
    }

    pub fn x_weight(&self) -> usize {
        (self
            .lexeme_list
            .iter()
            .map(|l| l.len())
            .collect::<Vec<usize>>())
        .iter()
        .product()
    }

    pub fn p_weight(&self) -> usize {
        (self
            .lexeme_list
            .iter()
            .enumerate()
            .map(|(i, lexeme)| (i + 1) * lexeme.len())
            .collect::<Vec<usize>>())
        .iter()
        .sum()
    }

    pub fn len(&self) -> usize {
        self.lexeme_list.len()
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
            self.begin(),
            self.end(),
            self.payload_len(),
            self.lexeme_list
        )
    }
}

impl Clone for LexemePath {
    fn clone(&self) -> Self {
        let mut the_copy = LexemePath::new();
        the_copy.begin = self.begin;
        the_copy.end = self.end;
        the_copy.payload_len = self.payload_len;
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
        if self.payload_len() > o.payload_len() {
            return Some(Ordering::Less);
        } else if self.payload_len() < o.payload_len() {
            return Some(Ordering::Greater);
        } else {
            if self.len() < o.len() {
                return Some(Ordering::Less);
            } else if self.len() > o.len() {
                return Some(Ordering::Greater);
            } else {
                if self.path_len() > o.path_len() {
                    return Some(Ordering::Less);
                } else if self.path_len() < o.path_len() {
                    return Some(Ordering::Greater);
                } else {
                    if self.end() > o.end() {
                        return Some(Ordering::Less);
                    } else if self.end() < o.end() {
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
        return if self.begin() == other.begin()
            && self.end() == other.end()
            && self.payload_len == other.payload_len
            && self.lexeme_list.len() == other.lexeme_list.len()
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
