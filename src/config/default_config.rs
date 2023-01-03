use crate::config::configuration::Configuration;
use serde;
use serde::{Deserialize, Serialize};
#[warn(unused_imports)]
use serde_yaml;
use std::fs::File;
use std::io::{BufReader, Read};
use std::marker::{Send, Sync};
use std::path::Path;
use std::vec::Vec;

// 分词器配置文件路径
const IK_CONFIG_NAME: &str = "ik.yml";

#[derive(Debug, Serialize, Deserialize)]
pub struct DefaultConfig {
    main_dict: String,
    quantifier_dict: String,
    stop_word_dict: String,
    ext_dicts: Vec<String>,
    ext_stop_word_dicts: Vec<String>,
}

unsafe impl Sync for DefaultConfig {}
unsafe impl Send for DefaultConfig {}

impl DefaultConfig {
    pub fn new() -> DefaultConfig {
        let root_path = env!("CARGO_MANIFEST_DIR");
        let conf_file_path = Path::new(root_path).join(IK_CONFIG_NAME);
        let file = File::open(conf_file_path).expect("open ik.yml error");
        let mut reader = BufReader::new(file);
        let mut yaml_str: String = "".to_string();
        reader
            .read_to_string(&mut yaml_str)
            .expect("read ik.yml error");
        let config: DefaultConfig = serde_yaml::from_str(yaml_str.as_str()).expect("json error");
        config
    }
}

fn root_path() -> String {
    let mut root_path = env!("CARGO_MANIFEST_DIR").to_string();
    root_path.push_str("/");
    root_path
}

impl Configuration for DefaultConfig {
    fn get_main_dictionary(&self) -> String {
        let mut root_path = root_path();
        root_path.push_str(self.main_dict.as_str());
        root_path
    }

    fn get_quantifier_dictionary(&self) -> String {
        let mut root_path = root_path();
        root_path.push_str(self.quantifier_dict.as_str());
        root_path
    }

    fn get_ext_dictionaries(&self) -> Vec<String> {
        let mut dicts = Vec::new();
        for dict in &self.ext_dicts {
            let mut root_path = root_path();
            root_path.push_str(dict);
            dicts.push(root_path);
        }
        dicts
    }

    fn get_ext_stop_word_dictionaries(&self) -> Vec<String> {
        let mut dicts = Vec::new();
        let mut stop_word_full = root_path();
        stop_word_full.push_str(&self.stop_word_dict);
        dicts.push(stop_word_full);
        for dict in &self.ext_stop_word_dicts {
            let mut root_path = root_path();
            root_path.push_str(dict);
            dicts.push(root_path);
        }
        dicts
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_config() {
        let config = DefaultConfig::new();
        println!("{:?}", config);
        println!("{}", config.get_main_dictionary());
        println!("{}", config.get_quantifier_dictionary());
        println!("{:?}", config.get_ext_dictionaries());
        println!("{:?}", config.get_ext_stop_word_dictionaries());
    }
}
