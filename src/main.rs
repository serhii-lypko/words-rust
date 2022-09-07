pub mod cli;
pub mod data_display_manager;
pub mod file_manager;
pub mod json_manager;
pub mod parser;
pub mod root_controller;

use file_manager::FileManager;
use root_controller::RootController;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type StdResult<T> = std::result::Result<T, Error>;

fn main() {
    // TODO: better use String insted of str to reduce complexity with lifetimes?
    let file_manager = FileManager::new("./words.json");

    let root_controller = RootController::new(&file_manager);

    root_controller.handle_command();
}
