// 配置管理类接口
pub trait Configuration {
    fn get_main_dictionary(&self) -> String;
    fn get_quantifier_dictionary(&self) -> String;
    fn get_ext_dictionaries(&self) -> Vec<String>;
    fn get_ext_stop_word_dictionaries(&self) -> Vec<String>;
}
