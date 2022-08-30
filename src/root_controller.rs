use serde::{Deserialize, Serialize};

use crate::cli::{read_command, Command};
use crate::file_manager::FileManager;
use crate::json_manager::JsonManager;

#[derive(Serialize, Deserialize, Debug)]
struct WordPair {
    eng: String,
    ru: String,
    id: u16,
}

type WordPairs = Vec<WordPair>;

pub struct RootController<'a> {
    file_manager: &'a FileManager<'a>,
    json_manager: &'a JsonManager,
}

impl<'a> RootController<'a> {
    pub fn new(file_manager: &'a FileManager<'a>, json_manager: &'a JsonManager) -> Self {
        Self {
            file_manager,
            json_manager,
        }
    }

    fn read_file_data(&self) -> Option<WordPairs> {
        let file_string = self.file_manager.read_file();

        if let Some(file_string) = file_string {
            let parsed: WordPairs = self.json_manager.deserialize(file_string.as_str());
            Some(parsed)
        } else {
            None
        }
    }

    pub fn handle_command(&self) {
        let command = read_command();
        let file_data = self.read_file_data();

        match command {
            Command::Show => match file_data {
                Some(word_pairs) => {
                    println!("{:#?}", &word_pairs);
                }
                None => {
                    println!("No data yet")
                }
            },
            Command::AddPair => match file_data {
                Some(mut word_pairs) => {
                    let new_pair = WordPair {
                        eng: "James".to_string(),
                        ru: "Frames 019z".to_string(),
                        id: 3,
                    };

                    word_pairs.push(new_pair);

                    let serialized_data = self.json_manager.serialize(word_pairs);

                    // TODO: delete, create and write to this file
                }
                None => {
                    // TODO: create and write to this file
                }
            },
            _ => {
                println!("Did not get any valid command")
            }
        }
    }
}
