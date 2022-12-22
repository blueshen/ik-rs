use crate::config::configuration::Configuration;
use crate::config::default_config::DefaultConfig;
use crate::dict::hit::Hit;
use crate::dict::trie::Trie;
#[warn(unused_imports)]
use once_cell;
use once_cell::sync::Lazy;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::marker::Sync;
use std::rc::Rc;
use std::sync::Mutex;

pub static GLOBAL_DICT: Lazy<Mutex<Dictionary>> = Lazy::new(|| {
    let mut dict = Dictionary::new();
    dict.init();
    Mutex::new(dict)
});

/// Dictionary Manager
pub struct Dictionary {
    main_dict: Trie,
    stop_word_dict: Trie,
    quantifier_dict: Trie,
    cfg: Rc<dyn Configuration>,
}

unsafe impl Sync for Dictionary {}
unsafe impl Send for Dictionary {}

impl Dictionary {
    pub fn new() -> Self {
        Dictionary {
            main_dict: Trie::new(),
            stop_word_dict: Trie::new(),
            quantifier_dict: Trie::new(),
            cfg: Rc::new(DefaultConfig::new()),
        }
    }

    pub fn init(&mut self) -> bool {
        self.load_main_dict() && self.load_stop_word_dict() && self.load_quantifier_dict()
    }

    pub fn add_words(&mut self, words: Vec<&str>) -> () {
        for word in words {
            self.main_dict.insert(word);
        }
    }

    pub fn disable_words(&mut self, words: Vec<&str>) -> () {
        for word in words {
            self.main_dict.delete(word);
        }
    }

    pub fn match_in_main_dict(&mut self, word: &str) -> Vec<Hit> {
        self.main_dict.match_word(word)
    }

    pub fn match_in_main_dict_with_offset(
        &mut self,
        word: &str,
        offset: usize,
        length: usize,
    ) -> Vec<Hit> {
        self.main_dict.match_word_with_offset(word, offset, length)
    }

    pub fn match_in_quantifier_dict(
        &mut self,
        word: &str,
        offset: usize,
        length: usize,
    ) -> Vec<Hit> {
        self.quantifier_dict
            .match_word_with_offset(word, offset, length)
    }

    pub fn is_stop_word(&mut self, word: &str, offset: usize, length: usize) -> bool {
        let hits = self
            .stop_word_dict
            .match_word_with_offset(word, offset, length);
        for hit in hits.iter() {
            if hit.is_match() {
                return true;
            }
        }
        return false;
    }

    fn load_main_dict(&mut self) -> bool {
        let main_dict_path = self.cfg.get_main_dictionary();
        let file = File::open(main_dict_path).expect("Open main_dict error!");
        let reader = BufReader::new(file);
        let mut total: usize = 0;
        for line in reader.lines() {
            match line {
                Ok(word) => {
                    self.main_dict.insert(&word.trim());
                    total += 1;
                }
                Err(e) => {
                    panic!("main dict read error:{}", e);
                }
            }
        }
        println!("load main_dict size = {}", total);
        self.load_ext_dict()
    }

    fn load_ext_dict(&mut self) -> bool {
        let ext_dict_files = self.cfg.get_ext_dictionaries();
        let mut total = 0;
        for ext_dict_file in ext_dict_files {
            let file = File::open(ext_dict_file).expect("open error");
            let reader = BufReader::new(file);
            for line in reader.lines() {
                match line {
                    Ok(word) => {
                        self.main_dict.insert(&word.trim());
                        total += 1;
                    }
                    Err(e) => {
                        panic!("ext dict read error:{}", e);
                    }
                }
            }
        }
        println!("ext dict total size = {}", total);
        true
    }

    fn load_stop_word_dict(&mut self) -> bool {
        let ext_stop_word_dict_files = self.cfg.get_ext_stop_word_dictionaries();
        let mut total = 0usize;
        for stop_file in ext_stop_word_dict_files {
            println!("{}", stop_file);
            let file = File::open(stop_file).expect("open error");
            let reader = BufReader::new(file);
            for line in reader.lines() {
                match line {
                    Ok(word) => {
                        self.stop_word_dict.insert(&word.trim());
                        total += 1;
                    }
                    Err(e) => {
                        panic!("stop dict read error:{}", e);
                    }
                }
            }
        }
        println!("stop dict total size = {}", total);
        true
    }

    fn load_quantifier_dict(&mut self) -> bool {
        let file_path = self.cfg.get_quantifier_dictionary();
        let file = File::open(&file_path[..]).expect("open error");
        let reader = BufReader::new(file);
        let mut total = 0usize;
        for line in reader.lines() {
            match line {
                Ok(word) => {
                    self.quantifier_dict.insert(&word.trim());
                    total += 1;
                }
                Err(e) => {
                    panic!("quantifier dict read error:{}", e);
                }
            }
        }
        println!("quantifier_dict total size = {}", total);
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_dictionary() {
        let mut dictionary = Dictionary::new();
        let inited = dictionary.init();
        assert_eq!(true, inited);
        let mut words = Vec::new();
        words.push("abcd");
        words.push("blues");
        dictionary.add_words(words);

        let vec_exist = vec!["一夕之间", "ab", "万般皆下品唯有读书高", "张三", "张"];
        println!("{}", "一夕之间".to_string().len());
        for word in vec_exist {
            let hits = dictionary.match_in_main_dict(word);
            assert_eq!(true, hits.len() > 0);
        }
    }
}
