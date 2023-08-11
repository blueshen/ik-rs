use crate::config::configuration::Configuration;
use crate::config::default_config::DefaultConfig;
use crate::dict::hit::Hit;
use crate::dict::trie::Trie;
use once_cell;
use once_cell::sync::Lazy;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::RwLock;

pub static GLOBAL_DICT: Lazy<RwLock<Dictionary>> = Lazy::new(|| {
    let mut dict = Dictionary::new();
    if !dict.init() {
        panic!("dict init fatal error")
    }
    RwLock::new(dict)
});

fn load(dict: &mut Trie, file_path: &str) -> bool {
    let open_file = File::open(file_path);
    match open_file {
        Ok(file) => {
            let reader = BufReader::new(file);
            for line in reader.lines() {
                match line {
                    Ok(word) => {
                        dict.insert(&word.trim());
                    }
                    Err(e) => {
                        panic!("read dict:{} error:{}", file_path, e);
                    }
                }
            }
            log::debug!("after load dict:{}, size = {}", file_path, dict.size());
            return true;
        }
        Err(e) => {
            panic!("open dict:{} error:{}", file_path, e);
        }
    }
}
/// Dictionary Manager
pub struct Dictionary {
    main_dict: Trie,
    stop_word_dict: Trie,
    quantifier_dict: Trie,
    cfg: Box<dyn Configuration>,
}

unsafe impl Sync for Dictionary {}
unsafe impl Send for Dictionary {}

impl Dictionary {
    pub fn new() -> Self {
        Dictionary {
            main_dict: Trie::new(),
            stop_word_dict: Trie::new(),
            quantifier_dict: Trie::new(),
            cfg: Box::new(DefaultConfig::new()),
        }
    }

    fn init(&mut self) -> bool {
        self.load_main_dict() && self.load_stop_word_dict() && self.load_quantifier_dict()
    }

    #[allow(dead_code)]
    pub fn add_words(&mut self, words: Vec<&str>) -> () {
        for word in words.iter() {
            self.main_dict.insert(word);
        }
    }

    #[allow(dead_code)]
    pub fn disable_words(&mut self, words: Vec<&str>) -> () {
        for word in words.iter() {
            self.main_dict.delete(word);
        }
    }

    #[allow(dead_code)]
    pub fn match_in_main_dict(&self, word: &str) -> Vec<Hit> {
        self.main_dict.match_word(word)
    }

    pub fn match_in_main_dict_with_offset(
        &self,
        word: &str,
        offset: usize,
        length: usize,
    ) -> Vec<Hit> {
        self.main_dict.match_word_with_offset(word, offset, length)
    }

    pub fn match_in_quantifier_dict(&self, word: &str, offset: usize, length: usize) -> Vec<Hit> {
        self.quantifier_dict
            .match_word_with_offset(word, offset, length)
    }

    pub fn is_stop_word(&self, word: &str, offset: usize, length: usize) -> bool {
        let hits = self
            .stop_word_dict
            .match_word_with_offset(word, offset, length);
        for hit in hits.iter() {
            if hit.is_match() {
                return true;
            }
        }
        false
    }

    fn load_main_dict(&mut self) -> bool {
        let file_path = self.cfg.get_main_dictionary();
        if load(&mut self.main_dict, file_path.as_str()) {
            return self.load_ext_dict();
        }
        false
    }

    fn load_ext_dict(&mut self) -> bool {
        let ext_dict_files = self.cfg.get_ext_dictionaries();
        let mut ret = true;
        for ext_dict_file in ext_dict_files.iter() {
            if !load(&mut self.main_dict, ext_dict_file.as_str()) {
                ret = false;
            }
        }
        ret
    }

    fn load_stop_word_dict(&mut self) -> bool {
        let ext_stop_word_dict_files = self.cfg.get_ext_stop_word_dictionaries();
        let mut ret = true;
        for stop_file in ext_stop_word_dict_files.iter() {
            if !load(&mut self.stop_word_dict, stop_file.as_str()) {
                ret = false;
            }
        }
        ret
    }

    fn load_quantifier_dict(&mut self) -> bool {
        let file_path = self.cfg.get_quantifier_dictionary();
        load(&mut self.quantifier_dict, file_path.as_str())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::thread;
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
        for word in vec_exist {
            let hits = dictionary.match_in_main_dict(word);
            assert_eq!(true, hits.len() > 0);
        }
    }

    #[test]
    fn test_thread_safe() {
        let dict = Dictionary::new();
        let t = thread::spawn(move || {
            println!("{:?}", dict.is_stop_word("的", 0, 1));
        });
        t.join().unwrap();
    }
}
