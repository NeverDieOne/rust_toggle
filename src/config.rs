use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct ToggleConfig {   
    pub api_key: String,
    pub proxy: String,
    pub workspace_id: u32,
}


impl ::std::default::Default for ToggleConfig {
    fn default() -> Self { Self { 
        api_key: "".into(),
        proxy: "".into(),
        workspace_id: 7037761,
    } }
}


pub fn _load_config(config_name: &str) -> ToggleConfig {
    confy::load("self", config_name)
        .expect("Can not load config")
}


pub fn _save_config(config_name: &str, conf: ToggleConfig) {
    confy::store("self", config_name, conf)
        .expect("Can not save config")
}