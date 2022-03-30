use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::write_config;

#[derive(Serialize, Deserialize)]
pub struct Svy {
    pub registry: HashMap<String, String>,
}

impl Svy {
    pub fn update_registry(&mut self, key: &String, value: &String) {
        if let Some(_) = self.registry.get(key) {
            self.registry.remove(key);
        }
        self.registry.insert(key.to_string(), value.to_string());

        write_config(&self).unwrap();
    }
}
