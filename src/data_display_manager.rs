pub mod data_display_manager {
    use crate::root_controller::{FileData, WordPairs};

    pub fn show_all_pairs(file_data: &FileData) {
        if let Some(word_pairs) = file_data {
            return;
        }

        println!("No data to show");
    }

    pub fn exam(file_data: &FileData) {
        if let Some(word_pairs) = file_data {
            return;
        }

        println!("No data to show");
    }
}
