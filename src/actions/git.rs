//! Git 处理

use crate::common::{utils, Svy};
use clap::Args;
use colored::Colorize;

#[derive(Args)]
pub struct Git {
    /// 添加
    #[clap(short = 'n', long = "set-name")]
    pub new_name: Option<String>,

    /// 添加
    #[clap(short = 'e', long = "set-email")]
    pub new_email: Option<String>,

    /// 查询
    #[clap(short, long)]
    pub list: bool,

    /// 切换
    pub name: Option<String>,
}

impl Git {
    pub fn exec(&self, svy: &mut Svy) {
        if self.list {
            self.list(svy);
            return;
        }

        if let Some(name) = &self.name {
            if self.new_email.is_none() && self.new_name.is_none() {
                self.change(svy, name);
                return;
            }

            // update or insert
            if let Some(new_name) = &self.new_name {
                svy.update_git_name(&name, &new_name);
            }
            if let Some(new_email) = &self.new_email {
                svy.update_git_email(&name, &new_email);
            }
            return;
        }

        self.list(svy)
    }

    fn list(&self, svy: &Svy) {
        if svy.git.is_empty() {
            println!("{}", "暂未配置git信息".red());
            return;
        }

        let name_output = utils::git_config("user.name");
        let email_output = utils::git_config("user.email");

        println!();
        for config in &svy.git {
            if name_output.trim() == config.1.username && email_output.trim() == config.1.email {
                println!(
                    "* {:6}\t[{}] <{}>",
                    config.0.green(),
                    config.1.username.green(),
                    config.1.email.green()
                );
                continue;
            }
            println!("- {:6}\t[{}] <{}>", config.0, config.1.username, config.1.email)
        }
        println!();
    }

    fn change(&self, svy: &Svy, name: &String) {
        if let Some(config) = svy.git.get(name) {
            let name_output = utils::set_git_config("user.name", &config.username);
            let email_output = utils::set_git_config("user.email", &config.email);

            println!();
            if name_output.trim().len() == 0 && email_output.trim().len() == 0 {
                println!("[{}] 切换成功\n", name.green());
                return;
            }
        }
        println!("[{}] 切换失败\n", name.red());
    }
}
