use crate::dict::hit::Hit;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct TrieNode {
    value: Option<char>,
    final_state: bool,
    child_nodes: HashMap<char, TrieNode>,
}

impl Display for TrieNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "TrieNode[value:{:?}, final_state:{}, childs:{}]",
            self.value,
            self.final_state,
            self.child_nodes.len()
        )
    }
}

impl TrieNode {
    pub fn new(c: char, final_state: bool) -> Self {
        TrieNode {
            value: Some(c),
            final_state,
            child_nodes: HashMap::new(),
        }
    }

    pub fn new_root() -> TrieNode {
        TrieNode {
            value: None,
            final_state: false,
            child_nodes: HashMap::new(),
        }
    }

    pub fn has_childs(&self) -> bool {
        self.child_nodes.len() > 0
    }

    pub fn is_final_state(&self) -> bool {
        self.final_state
    }

    pub fn check_value(self, c: char) -> bool {
        self.value == Some(c)
    }

    pub fn add_child(&mut self, c: char, final_state: bool) {
        self.child_nodes.insert(c, TrieNode::new(c, final_state));
    }

    pub fn exist(&self, string_val: &str) -> bool {
        let mut current_node = self;
        let char_list: Vec<char> = string_val.chars().collect();
        for counter in 0..char_list.len() {
            if !current_node.child_nodes.contains_key(&char_list[counter]) {
                return false;
            }
            current_node = current_node.child_nodes.get(&char_list[counter]).unwrap();
        }
        if current_node.final_state == true {
            return true;
        } else {
            return false;
        }
    }

    pub fn delete(&mut self, string_val: &str) -> bool {
        let mut current_node = self;
        let char_list: Vec<char> = string_val.chars().collect();
        for counter in 0..char_list.len() {
            if !current_node.child_nodes.contains_key(&char_list[counter]) {
                return true;
            }
            current_node = current_node
                .child_nodes
                .get_mut(&char_list[counter])
                .unwrap();
        }
        current_node.final_state = false;
        return true;
    }

    pub fn insert(&mut self, string_val: &str) {
        let mut current_node = self;
        let char_list: Vec<char> = string_val.chars().collect();
        let mut final_state = false;
        for counter in 0..char_list.len() {
            if !current_node.child_nodes.contains_key(&char_list[counter]) {
                if counter == char_list.len() - 1 {
                    final_state = true;
                }
                current_node.add_child(char_list[counter], final_state);
            }
            current_node = current_node
                .child_nodes
                .get_mut(&char_list[counter])
                .unwrap();
        }
    }

    pub fn match_with_offset(&self, string_val: &str, offset: usize, length: usize) -> Vec<Hit> {
        let mut hits = Vec::new();
        let mut current_node = self;
        let char_list: Vec<char> = string_val.chars().collect();
        if offset + length <= char_list.len() {
            let mut end = offset;
            for counter in offset..offset + length {
                if !current_node.child_nodes.contains_key(&char_list[counter]) {
                    break;
                }
                if current_node.final_state {
                    let mut hit = Hit::new();
                    hit.begin = offset;
                    hit.end = end;
                    hit.set_match();
                    if current_node.has_childs() {
                        hit.set_prefix();
                    }
                    hits.push(hit);
                }
                current_node = current_node.child_nodes.get(&char_list[counter]).unwrap();
                end = counter;
            }
            if current_node.value.is_some() {
                let mut hit = Hit::new();
                hit.begin = offset;
                hit.end = end;
                if current_node.final_state {
                    hit.set_match();
                }
                if current_node.has_childs() {
                    hit.set_prefix();
                }
                hits.push(hit);
            }
        }
        hits
    }
}

#[derive(Debug)]
pub struct Trie {
    root: TrieNode,
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            root: TrieNode::new_root(),
        }
    }

    pub fn insert(&mut self, string_val: &str) {
        let current_node = &mut self.root;
        current_node.insert(string_val)
    }

    pub fn delete(&mut self, string_val: &str) -> bool {
        let current_node = &mut self.root;
        current_node.delete(string_val)
    }

    pub fn exist(&mut self, string_val: &str) -> bool {
        let current_node = &mut self.root;
        current_node.exist(string_val)
    }

    pub fn match_word(&mut self, string_val: &str) -> Vec<Hit> {
        let root_node = &mut self.root;
        let v: Vec<char> = string_val.chars().collect();
        let len = v.len();
        root_node.match_with_offset(string_val, 0, len)
    }

    pub fn match_word_with_offset(
        &mut self,
        string_val: &str,
        offset: usize,
        length: usize,
    ) -> Vec<Hit> {
        let root_node = &mut self.root;
        root_node.match_with_offset(string_val, offset, length)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn trie_exist() {
        let mut trie = Trie::new();
        trie.insert("Test");
        trie.insert("Tea");
        trie.insert("Background");
        trie.insert("Back");
        trie.insert("Brown");
        trie.insert("申艳超");
        trie.insert("blues小站");

        assert_eq!(false, trie.exist("Testing"));
        assert_eq!(true, trie.exist("Brown"));
        assert_eq!(true, trie.exist("申艳超"));
        assert_eq!(false, trie.exist("申超"));
    }

    #[test]
    fn trie_search() {
        let mut trie = Trie::new();
        trie.insert("Test");
        trie.insert("Tea");
        trie.insert("Background");
        trie.insert("Back");
        trie.insert("Brown");

        let hits = trie.match_word(&String::from("申艳超"));
        assert_eq!(0, hits.len());
        let hits = trie.match_word(&String::from("Tea"));
        for hit in hits.iter() {
            println!("{:?}", hit);
        }
    }
}
