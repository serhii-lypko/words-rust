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

// TODO: cli manager?

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
