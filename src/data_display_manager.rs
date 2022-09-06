pub mod data_display_manager {
    use crate::root_controller::{FileData, WordPairs};
    use rand::Rng;

    fn shuffle_pairs(word_pairs: &WordPairs) -> WordPairs {
        let mut rng = rand::thread_rng();

        let mut shuffled_pairs: WordPairs = vec![];

        let picking_range = word_pairs.len();
        let mut picked_indexes: Vec<usize> = vec![];

        loop {
            let random_pick = rng.gen_range(0..picking_range);

            if !picked_indexes.contains(&random_pick) {
                picked_indexes.push(random_pick);
                shuffled_pairs.push(word_pairs[random_pick].clone());
            }

            if picked_indexes.len() == picking_range {
                break;
            }
        }

        shuffled_pairs
    }

    fn structurize_word_pairs_in_tuples(word_pairs: WordPairs) -> Vec<(String, String)> {
        let vec_of_tuples = word_pairs
            .iter()
            .map(|pair| (pair.eng.clone(), pair.ru.clone()));

        vec_of_tuples.collect()
    }

    pub fn show_all_pairs(file_data: &FileData) {
        if let Some(word_pairs) = file_data {
            let shuffled_pairs = shuffle_pairs(word_pairs);
            let word_pair_tuples = structurize_word_pairs_in_tuples(shuffled_pairs);

            println!("{:#?}", word_pair_tuples);

            return;
        }

        println!("No data to show");
    }

    pub fn exam(file_data: &FileData) {
        if let Some(word_pairs) = file_data {
            let shuffled_pairs = shuffle_pairs(word_pairs);
            let word_pair_tuples = structurize_word_pairs_in_tuples(shuffled_pairs);

            return;
        }

        println!("No data to show");
    }
}
