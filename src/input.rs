mod key;
mod modifier;

pub use key::Key;
use key::KeyCommand;
pub use modifier::KeyModifiers;
use modifier::{ToggleCommand, setup_scoreboard, update_scoreboard};

use std::{iter, sync::mpsc::Sender};

use bevy::{
    app::{App, Plugin, PostStartup, Update},
    utils::synccell::SyncCell,
};
use bevy_ecs::{
    event::EventReader,
    system::{ResMut, Resource},
};
use itertools::chain;
use valence::{
    command::{AddCommand, handler::CommandResultEvent, parsers::GreedyString},
    command_macros::Command,
};

#[derive(Resource)]
pub struct EventSender(pub SyncCell<Sender<Event>>);

/// [`Key`] and it's [`KeyModifiers`]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Event {
    pub key: Key,
    pub modifiers: KeyModifiers,
}

pub struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_command::<KeyCommand>()
            .add_command::<Input>()
            .add_command::<InputNewLine>()
            .add_command::<ToggleCommand>()
            .add_systems(PostStartup, setup_scoreboard)
            .add_systems(Update, (handle_commands, update_scoreboard));
    }
}

#[derive(Command, Debug, Clone)]
#[paths("input {input}", "i {input}")]
#[scopes("valence.command.input")]
struct Input {
    input: GreedyString,
}

type Command<'a, 'b, T> = EventReader<'a, 'b, CommandResultEvent<T>>;
fn handle_commands(
    mut input_commands: Command<Input>,
    mut input_newline_commands: Command<InputNewLine>,
    mut key_commands: Command<KeyCommand>,
    mut toggle_commands: Command<ToggleCommand>,
    mut event_sender: ResMut<EventSender>,
    mut modifiers: ResMut<KeyModifiers>,
) {
    let event_sender = event_sender.0.get();
    toggle_commands
        .read()
        .for_each(|command| match command.result {
            ToggleCommand::Shift => modifiers.toggle(KeyModifiers::SHIFT),
            ToggleCommand::Control => modifiers.toggle(KeyModifiers::CONTROL),
            ToggleCommand::Alt => modifiers.toggle(KeyModifiers::ALT),
        });

    chain![
        input_commands
            .read()
            .flat_map(|event| event.result.input.0.chars().map(Key::Char)),
        input_newline_commands.read().flat_map(|event| event
            .result
            .input
            .0
            .chars()
            .map(Key::Char)
            .chain(iter::once(Key::Enter))),
        key_commands.read().map(|event| Key::from(event.result))
    ]
    .for_each(|key| {
        _ = event_sender.send(Event {
            key,
            modifiers: *modifiers,
        })
    });
}

#[derive(Command, Debug, Clone)]
#[paths("inputln {input}", "l {input}")]
#[scopes("valence.command.inputln")]
struct InputNewLine {
    input: GreedyString,
}
