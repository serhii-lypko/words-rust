use serde::{Deserialize, Serialize};

use crate::cli::cli;
use crate::json_manager::json_manager;
use crate::parser::parser;

use crate::file_manager::FileManager;

#[derive(Serialize, Deserialize, Debug)]
pub struct WordPair {
    eng: String,
    ru: String,
    id: u16,
}

pub type WordPairs = Vec<WordPair>;

pub struct RootController<'a> {
    file_manager: &'a FileManager<'a>,
}

impl<'a> RootController<'a> {
    pub fn new(file_manager: &'a FileManager<'a>) -> Self {
        Self { file_manager }
    }

    fn get_file_data(&self) -> Option<WordPairs> {
        use json_manager::deserialize;

        let file_string = self.file_manager.get_file_string();

        if let Some(file_string) = file_string {
            let parsed: WordPairs = deserialize(file_string.as_str());
            Some(parsed)
        } else {
            None
        }
    }

    fn serialize_and_write_data(&self, data: WordPairs) {
        use json_manager::serialize;

        let serialized_data = serialize(data);
        let result = self.file_manager.write_to_file(serialized_data);

        if let Ok(()) = result {
            println!("Data writen");
        }
    }

    pub fn handle_command(&self) {
        use cli::{collect_data_from_cli, read_command, Command};
        use parser::parse_word_strings;

        let command = read_command();
        let file_data = self.get_file_data();

        match command {
            Command::Show => match file_data {
                Some(word_pairs) => {
                    // TODO: data_display_manager
                    println!("{:#?}", &word_pairs);
                }
                None => println!("No data yet"),
            },
            Command::AddPair => {
                // let raw_word_strings = collect_data_from_cli();
                let raw_word_strings = vec![
                    "James-Frames".to_string(),
                    "Ricard-Johnson".to_string(),
                    "Tom-Williams".to_string(),
                ];

                let parsed_data = parse_word_strings(raw_word_strings);

                // println!("{:#?}", raw_word_strings);

                match file_data {
                    Some(mut word_pairs) => {
                        let new_pair = WordPair {
                            eng: "Next one".to_string(),
                            ru: "Dwons".to_string(),
                            id: 1,
                        };
                        word_pairs.push(new_pair);
                        // self.serialize_and_write_data(word_pairs);
                    }
                    None => {
                        let mut word_pairs: WordPairs = vec![];
                        let new_pair = WordPair {
                            eng: "Total new".to_string(),
                            ru: "Zero 01".to_string(),
                            id: 0,
                        };
                        word_pairs.push(new_pair);
                        // self.serialize_and_write_data(word_pairs);
                    }
                }
            }
            _ => {
                println!("Did not get any valid command.");
                Command::show_valid_commands();
            }
        }
    }
}
