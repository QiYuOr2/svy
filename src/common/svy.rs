use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use super::write_config;

#[derive(Serialize, Deserialize)]
pub struct Git {
    pub username: String,
    pub email: String,
}

impl Git {
    fn new(username: String, email: String) -> Self {
        Git { username, email }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Svy {
    /// npm源
    pub registry: BTreeMap<String, String>,

    /// git配置
    pub git: BTreeMap<String, Git>,
}

impl Svy {
    pub fn update_registry(&mut self, key: &String, value: &String) {
        if let Some(_) = self.registry.get(key) {
            self.registry.remove(key);
        }
        self.registry.insert(key.to_string(), value.to_string());

        write_config(&self).unwrap();
    }

    pub fn update_git_email(&mut self, key: &String, email: &String) {
        let config = match self.git.get_mut(key) {
            Some(git) => {
                let _git = Git::new(git.username.to_string(), email.to_string());
                self.git.remove(key);
                _git
            }
            None => Git::new("".to_string(), email.to_string()),
        };
        self.git.insert(key.to_string(), config);
        write_config(&self).unwrap();
    }

    pub fn update_git_name(&mut self, key: &String, name: &String) {
        let config = match self.git.get_mut(key) {
            Some(git) => {
                let _git = Git::new(name.to_string(), git.email.to_string());
                self.git.remove(key);
                _git
            }
            None => Git::new(name.to_string(), "".to_string()),
        };
        self.git.insert(key.to_string(), config);
        write_config(&self).unwrap();
    }
}
