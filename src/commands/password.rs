use clap::Parser;
use clap::Subcommand;
use std::fs;
use serde::{self, Deserialize, Serialize};
use dirs::home_dir;
use toml_edit::{DocumentMut, Value, Table, Item};

#[derive(Parser)]
pub struct PasswordCommand {
    /// 命令行的输入
    #[command(subcommand)]
    command: PasswordSubCommand,
}

#[derive(Subcommand)]
pub enum PasswordSubCommand {
    Get {
        /// 获取特定目标（Host主机或站点）的账户信息
        target: String
    },
    Set {
        /// 目标站点
        target: String,
        /// 用户名
        username: String,
        /// 密码
        password: String,
    }
}

impl PasswordCommand {
    pub fn run(&self) {
        match &self.command {
            PasswordSubCommand::Get { target } => {
                read_target(target);
            }
            PasswordSubCommand::Set { target, username, password } => {
                write_target(target, username, password);
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct Config {
    #[serde(flatten)] // 展平映射，将每个 target 当作键值对存储
    targets: std::collections::HashMap<String, Target>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Target {
    username: String,
    password: String,
}

fn read_target(target: &String) {
    let mut config_path = home_dir().expect("无法访问Home目录");
    config_path.push(".config/yt/config.toml");
    if !config_path.exists() {
        println!("配置文件不存在");
        return;
    }

    let content = fs::read_to_string(config_path);
    match content {
        Ok(content) => {
            let config: Config = toml::from_str(&content).unwrap();
            match config.targets.get(target) {
                Some(target) => {
                    println!("用户名：{}", target.username);
                    println!("密码：{}", target.password);
                }
                None => {
                    println!("目标 {} 不存在", target);
                }
            }
        }
        Err(e) => {
            eprintln!("读取配置文件失败: {}", e);
        }
    }
}

fn write_target(target: &String, username: &String, password: &String) {
    let mut config_path = home_dir().expect("无法访问Home目录");
    config_path.push(".config/yt/config.toml");

    let content = if config_path.exists() {
        fs::read_to_string(&config_path).unwrap()
    } else {
        String::new()
    };

    let mut doc = content.parse::<DocumentMut>().unwrap();
    let mut new_table = Table::new();
    new_table.insert("username", Item::Value(Value::from(username)));
    new_table.insert("password", Item::Value(Value::from(password)));
    doc[target] = Item::Table(new_table);

    // 文件不存在时，会自动创建
    fs::write(config_path, doc.to_string()).unwrap();
}
