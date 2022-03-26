use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Svy {
    pub registry: HashMap<String, String>,
}

impl Svy {}
