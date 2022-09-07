pub mod data_display_manager {
    use crate::root_controller::{FileData, WordPairs};
    use rand::Rng;

    type WordPairTuples = Vec<(String, String)>;

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

    fn structurize_word_pairs_in_tuples(word_pairs: WordPairs) -> WordPairTuples {
        let vec_of_tuples = word_pairs
            .iter()
            .map(|pair| (pair.eng.clone(), pair.ru.clone()));

        vec_of_tuples.collect()
    }

    fn define_longest_word_len(word_pair_tuples: &WordPairTuples) -> usize {
        let lengths: Vec<usize> = word_pair_tuples.iter().map(|(eng, _)| eng.len()).collect();

        *lengths.iter().max().unwrap()
    }

    fn create_visual_space() {
        for _ in 0..2 {
            println!("");
        }
    }

    fn show_words_in_column(words: Vec<&String>) {
        create_visual_space();
        for word in words {
            println!("{}", word);
        }
        create_visual_space();
    }

    pub fn show_all(file_data: &FileData) {
        if let Some(word_pairs) = file_data {
            let shuffled_pairs = shuffle_pairs(word_pairs);
            let word_pair_tuples = structurize_word_pairs_in_tuples(shuffled_pairs);

            let longest_word_length = define_longest_word_len(&word_pair_tuples);

            create_visual_space();
            for (eng, ru) in word_pair_tuples {
                let base_word_len = eng.len();
                let max_len_delta = longest_word_length - base_word_len;
                let separator = "-";
                let basic_separation_distance = 5;

                let formatted_pair = format!(
                    "{} {} {}",
                    eng,
                    separator.repeat(max_len_delta + basic_separation_distance),
                    ru
                );

                println!("{}", formatted_pair);
            }
            create_visual_space();

            return;
        }

        println!("No data to show");
    }

    pub fn exam(file_data: &FileData) {
        if let Some(word_pairs) = file_data {
            let shuffled_pairs = shuffle_pairs(word_pairs);
            let word_pair_tuples = structurize_word_pairs_in_tuples(shuffled_pairs);

            let mut rng = rand::thread_rng();
            let random_pick: usize = rng.gen_range(0..2);

            let eng_words: Vec<&String> = word_pair_tuples.iter().map(|(eng, _)| eng).collect();
            let ru_words: Vec<&String> = word_pair_tuples.iter().map(|(_, ru)| ru).collect();

            match random_pick {
                0 => show_words_in_column(eng_words),
                1 => show_words_in_column(ru_words),
                _ => {}
            }

            return;
        }

        println!("No data to show");
    }
}
