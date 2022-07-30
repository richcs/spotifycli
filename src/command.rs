pub struct Command {
    pub command_type: CommandType,
    pub args: Vec<String>,
}

impl Command {
    pub fn new(input: String) -> Command {
        let mut parts = input.split_whitespace();
        let command_type_string = parts.next().unwrap();
        let given_command_type = command_type_string.parse().unwrap();
        let given_args = parts.map(str::to_string).collect();

        Command {
            command_type: given_command_type,
            args: given_args,
        }
    }
}

pub enum CommandType {
    Play,
    Pause,
    Stop,
    List,
    Whoami,
    Help,
    Quit,
    Unknown,
}

impl std::str::FromStr for CommandType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "play" => Ok(CommandType::Play),
            "pause" => Ok(CommandType::Pause),
            "stop" => Ok(CommandType::Stop),
            "ls" | "list" => Ok(CommandType::List),
            "whoami" => Ok(CommandType::Whoami),
            "help" | "man" => Ok(CommandType::Help),
            "quit" => Ok(CommandType::Quit),
            _ => Ok(CommandType::Unknown),
        }
    }
}

#[derive(Debug)]
pub struct CommandOk {}

#[derive(Debug)]
pub struct CommandError {}
