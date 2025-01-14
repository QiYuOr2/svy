//! Npm 源处理

use crate::common::{constants::NPM, Svy};
use clap::Args;
use colored::Colorize;
use std::process::Command;

#[derive(Args)]
pub struct Registry {
    /// 添加源
    #[clap(short = 's', long = "set")]
    pub key: Option<String>,

    /// 查询所有源
    #[clap(short, long)]
    pub list: bool,

    /// 切换对应源
    pub name: Option<String>,
}

impl Registry {
    pub fn exec(&self, svy: &mut Svy) {
        // 含有`--list`时，查询所有源
        if self.list == true {
            self.list(svy);
            return;
        }
        // 切换源
        if let Some(name) = &self.name {
            // 如果有key，添加源
            if self.key.is_some() {
                self.set(svy, name, &self.key.as_ref().unwrap());
                return;
            }
            self.change(svy, name);
            return;
        }
        // 默认为list操作
        self.list(svy);
    }
    
    fn list(&self, svy: &Svy) {
        // 获取当前使用的源
        let output = Command::new(NPM)
            .arg("config")
            .arg("get")
            .arg("registry")
            .output()
            .expect("npm registry获取失败");

        let result = String::from_utf8(output.stdout).unwrap();

        println!("");
        for r in &svy.registry {
            if r.1.trim() == result.trim() {
                println!("* {:6} \t{}", r.0.green(), r.1.green());
                continue;
            }
            println!("- {:6} \t{}", r.0, r.1);
        }
        println!("");
    }

    fn change(&self, svy: &Svy, name: &String) {
        if !svy.registry.keys().any(|k| k == name) {
            println!("未找到 name 为 {} 的源地址", name);
            return;
        }

        let output = Command::new(NPM)
            .arg("config")
            .arg("set")
            .arg("registry")
            .arg(svy.registry.get(name).unwrap())
            .output()
            .expect("npm registry 切换失败");

        let result = String::from_utf8(output.stdout).unwrap();
        if result.trim() != "" {
            println!("{}", result.red());
            return;
        }

        println!("\n成功切换到 [{}]", name.green());
        self.list(svy);
    }

    fn set(&self, svy: &mut Svy, key: &String, value: &String) {
        svy.update_registry(key, value);
        self.list(svy);
    }
}
