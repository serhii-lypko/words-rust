use serde::{Deserialize, Serialize};
use serde_json;

pub struct JsonManager;

impl JsonManager {
    pub fn new() -> Self {
        Self {}
    }

    pub fn deserialize<'a, T: Deserialize<'a>>(&self, string: &'a str) -> T {
        let deserialized: T = serde_json::from_str(string).expect("Could not deserialize data");
        deserialized
    }

    pub fn serialize<T: Serialize>(&self, data: T) -> String {
        let serialized = serde_json::to_string(&data).expect("Could not serialize data");
        serialized
    }
}
