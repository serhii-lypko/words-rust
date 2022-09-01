pub mod json_manager {
    use serde::{Deserialize, Serialize};

    pub fn serialize<T: Serialize>(data: T) -> String {
        let serialized = serde_json::to_string(&data).expect("Could not serialize data");
        serialized
    }

    pub fn deserialize<'a, T: Deserialize<'a>>(string: &'a str) -> T {
        let deserialized: T = serde_json::from_str(string).expect("Could not deserialize data");
        deserialized
    }
}
