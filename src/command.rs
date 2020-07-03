pub enum CommandKind {
    Quit,
    Help,
}

pub fn parse_input(input: &String) -> Option<CommandKind> {
    let input = input.trim().to_lowercase();

    if input == ":q" || input == ":quit" {
        return Some(CommandKind::Quit);
    }

    if input == ":h" || input == ":help" {
        return Some(CommandKind::Help);
    }

    return None;
}
