pub mod parser {
    use crate::root_controller::{WordPair, WordPairs};
    use uuid::Uuid;

    fn parse_string_to_word_struct(raw_string: String) -> Result<WordPair, String> {
        let words: Vec<&str> = raw_string.split("-").collect();

        if words.len() != 2 {
            return Err(String::from("Error: malformed string"));
        }

        let eng = words[0].to_string();
        let ru = words[1].to_string();

        Ok(WordPair {
            eng,
            ru,
            id: Uuid::new_v4(),
        })
    }

    pub fn parse_raw_strings(raw_data: Vec<String>) -> Result<WordPairs, String> {
        raw_data
            .into_iter()
            .map(parse_string_to_word_struct)
            .collect()
    }
}
