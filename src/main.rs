/*
    1. Read and write WordPair to json file
    2. Print pairs with formatting
    3. Shuffle pairs (random order)
    4. Print only eng or only ru side (in randmon order)
    5. Print a mix of eng-ru half-blind records (in randmon order)
    6. Delete all pairs or particular (by id)
*/

pub mod cli;
pub mod file_manager;
pub mod json_manager;
pub mod root_controller;

use file_manager::FileManager;
use json_manager::JsonManager;
use root_controller::RootController;

/* --- --- --- --- --- --- --- --- --- --- --- ---  */

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type StdResult<T> = std::result::Result<T, Error>;

fn main() {
    let file_manager = FileManager::new("./words.json");
    let json_manager = JsonManager::new();

    let root_controller = RootController::new(&file_manager, &json_manager);

    root_controller.handle_command();
}
