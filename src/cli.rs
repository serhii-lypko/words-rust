pub mod cli {
    use std::io;
    use std::io::prelude::*;

    #[derive(Debug)]
    pub enum Command {
        Show,
        AddPair,
        DeleteAll,
        None,
    }

    impl From<String> for Command {
        fn from(command_string: String) -> Self {
            match command_string.as_str() {
                "show" => Command::Show,
                "add" => Command::AddPair,
                "delete-all" => Command::DeleteAll,
                _ => Command::None,
            }
        }
    }

    impl Command {
        pub fn show_valid_commands() {
            let valid_commands = vec![
                String::from("show"),
                String::from("add"),
                String::from("delete-all"),
            ];

            println!("Valid commands:");

            for command in valid_commands {
                println!("- {}", command);
            }
        }
    }

    pub fn read_command() -> Command {
        use std::env;

        let args: Vec<String> = env::args().collect();
        let args_len = args.len();
        let min_args_len_required = 2;

        if args_len < min_args_len_required {
            return Command::None;
        }

        let command = &args[1];
        let command: Command = String::from(command).into();

        command
    }

    pub fn collect_data_from_cli() -> Vec<String> {
        let mut vec: Vec<String> = Vec::new();
        let stdin = io::stdin();

        println!("Enter a list of words, EN-RU. End with Ctrl-D.");
        for line in stdin.lock().lines() {
            let line = line.unwrap();

            if let Ok(line_string) = line.trim().parse() {
                // TODO: cli-reading level validation?

                vec.push(line_string);
            }
        }
        vec
    }
}
