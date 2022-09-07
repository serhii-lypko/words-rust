use serde::{Deserialize, Serialize};
use std::process;
use uuid::Uuid;

use crate::cli::cli::{collect_data_from_cli, read_command, Command};
use crate::data_display_manager::data_display_manager::{exam, show_all};
use crate::json_manager::json_manager::{deserialize, serialize};
use crate::parser::parser::parse_raw_strings;

use crate::file_manager::FileManager;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WordPair {
    pub eng: String,
    pub ru: String,
    pub id: Uuid,
}

pub type WordPairs = Vec<WordPair>;

pub type FileData = Option<WordPairs>;

pub struct RootController<'a> {
    file_manager: &'a FileManager<'a>,
}

impl<'a> RootController<'a> {
    pub fn new(file_manager: &'a FileManager<'a>) -> Self {
        Self { file_manager }
    }

    fn get_file_data(&self) -> FileData {
        let file_string = self.file_manager.get_file_string();

        if let Some(file_string) = file_string {
            let parsed: WordPairs = deserialize(file_string.as_str());
            Some(parsed)
        } else {
            None
        }
    }

    fn serialize_and_write_data(&self, data: WordPairs) {
        let serialized_data = serialize(data);
        let result = self.file_manager.write_to_file(serialized_data);

        if let Ok(()) = result {
            println!("Data written");
        }
    }

    fn handle_add_command(&self, file_data: FileData) {
        let raw_word_strings = collect_data_from_cli();
        let new_word_pairs = parse_raw_strings(raw_word_strings);

        if let Err(parsing_err) = new_word_pairs {
            println!("{}", parsing_err);
            process::exit(1);
        }

        match file_data {
            Some(mut word_pairs) => {
                word_pairs.append(&mut new_word_pairs.unwrap());
                self.serialize_and_write_data(word_pairs);
            }
            None => {
                self.serialize_and_write_data(new_word_pairs.unwrap());
            }
        }
    }

    fn handle_delete_command(&self) {
        let deletion_result = self.file_manager.delete_file();

        if let Ok(_) = deletion_result {
            println!("All data have been deleted");
        }
    }

    pub fn handle_command(&self) {
        let command = read_command();
        let file_data = self.get_file_data();

        match command {
            Command::Show => show_all(&file_data),
            Command::Exam => exam(&file_data),
            Command::Add => self.handle_add_command(file_data),
            Command::DeleteAll => self.handle_delete_command(),
            _ => {
                println!("Did not get any valid command.");
                Command::show_valid_commands();
            }
        }
    }
}
