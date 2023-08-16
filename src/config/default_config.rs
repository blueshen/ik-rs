use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use std::vec::Vec;

use serde::{Deserialize, Serialize};
use {serde, serde_yaml};

use crate::config::configuration::Configuration;

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
    root_path.push('/');
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
        let root_path = root_path();
        let dicts = self
            .ext_dicts
            .iter()
            .map(|dict| root_path.clone() + dict)
            .collect();
        dicts
    }

    fn get_ext_stop_word_dictionaries(&self) -> Vec<String> {
        let mut dicts = Vec::new();
        let root_path = root_path();
        dicts.push(root_path.clone() + self.stop_word_dict.as_str());
        let ext_stopwords = self
            .ext_stop_word_dicts
            .iter()
            .map(|dict| root_path.clone() + dict.as_str())
            .collect::<Vec<String>>();
        dicts.extend(ext_stopwords);
        dicts
    }
}

#[cfg(test)]
mod test {
    use log;

    use super::*;

    #[test]
    pub fn test_config() {
        let config = DefaultConfig::new();
        log::info!("{:?}", config);
        log::info!("{}", config.get_main_dictionary());
        log::info!("{}", config.get_quantifier_dictionary());
        log::info!("{:?}", config.get_ext_dictionaries());
        log::info!("{:?}", config.get_ext_stop_word_dictionaries());
    }
}
