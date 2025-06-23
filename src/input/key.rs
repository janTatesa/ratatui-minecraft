use valence::command_macros::Command;

#[derive(Debug, Clone, Copy, Command)]
#[paths("key", "k")]
#[scopes("valence.command.key")]
pub enum KeyCommand {
    #[paths("bspace")]
    Backspace,
    #[paths("enter")]
    Enter,
    #[paths("left")]
    Left,
    #[paths("right")]
    Right,
    #[paths("up")]
    Up,
    #[paths("down")]
    Down,
    #[paths("home")]
    Home,
    #[paths("end")]
    End,
    #[paths("pgup")]
    PageUp,
    #[paths("pgdn")]
    PageDown,
    #[paths("tab")]
    Tab,
    #[paths("btab")]
    BackTab,
    #[paths("del")]
    Delete,
    #[paths("ins")]
    Insert,
    #[paths("space")]
    Space,
    #[paths("f {num}")]
    F { num: u32 },
    #[paths("esc")]
    Esc,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Key {
    Backspace,
    Enter,
    Left,
    Right,
    Up,
    Down,
    Home,
    End,
    PageUp,
    PageDown,
    Tab,
    BackTab,
    Delete,
    Insert,
    F(u8),
    Esc,
    Char(char),
}

impl From<KeyCommand> for Key {
    fn from(value: KeyCommand) -> Self {
        match value {
            KeyCommand::Backspace => Key::Backspace,
            KeyCommand::Enter => Key::Enter,
            KeyCommand::Left => Key::Left,
            KeyCommand::Right => Key::Right,
            KeyCommand::Up => Key::Up,
            KeyCommand::Down => Key::Down,
            KeyCommand::Home => Key::Home,
            KeyCommand::End => Key::End,
            KeyCommand::PageUp => Key::PageUp,
            KeyCommand::PageDown => Key::PageDown,
            KeyCommand::Tab => Key::Tab,
            KeyCommand::BackTab => Key::BackTab,
            KeyCommand::Delete => Key::Delete,
            KeyCommand::Insert => Key::Insert,
            KeyCommand::Space => Key::Char(' '),
            KeyCommand::F { num } => Key::F(num as u8),
            KeyCommand::Esc => Key::Esc,
        }
    }
}
