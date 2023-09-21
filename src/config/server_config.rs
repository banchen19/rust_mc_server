use serde::{Serialize, Deserialize};

// 异步
use std::{
    error::Error,
    fs,
    sync::{Arc, Mutex},
    thread,
};

use super::yml_util;

//服务端配置文件
#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct Config {
   pub server_name: String,
   pub version:String,
   pub player_max: u32,
   pub gamemode: String,
   pub ipv4_port: u16,
   pub ipv6_port: u16,
}

pub fn inti_config() -> Result<Config, Box<dyn Error>> {
    let file_path = "config.yml";
    let config = Config {
        server_name: "Rust_MC_Server_dev".to_string(),
        version: "1.20.12".to_string(),
        player_max: 2023,
        gamemode: "Survival".to_string(),
        ipv4_port: 19132,
        ipv6_port: 19133,
    };
    match fs::metadata(&file_path) {
        Err(_) => {
            let text = "File does not exist, start writing".to_string();
            if let Err(err) = yml_util::write_config_yml(&config, &file_path) {
                println!("Unable to write configuration file：{}", err);
            }
            println!("{}",text)
        }
        Ok(_) => {
            let text = "Detected presence of configuration file".to_string();
            println!("{}", text);
        }
    }
    read_config_yml(&file_path)
}

// 读取配置文件
pub fn read_config_yml(file_path: &str) -> Result<Config, Box<dyn Error>> {
    let config = yml_util::read_config_yml(file_path)?;
    Ok(config)
}