use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Svy {
    pub registry: HashMap<String, String>,
}

impl Svy {
    pub fn list_registry(&self) {
        for r in &self.registry {
            println!("{}\t{}", r.0, r.1);
        }
    }
}
