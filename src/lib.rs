mod backend;
mod color;
mod game_manager;
mod init_clients;
mod input;
mod setup;

use std::sync::mpsc;

use backend::MinecraftTermBackend;
use bevy::utils::synccell::SyncCell;
use game_manager::{TermWrapper, TerminalGameManager};
use init_clients::init_clients;
use input::{EventSender, InputPlugin};
use setup::setup;

use ratatui::Terminal;

use tap::Pipe;
use valence::{
    DefaultPlugins,
    app::{App, Startup, Update},
    prelude::{ConnectionMode, NetworkSettings},
};

use bevy_trait_query::RegisterExt;
use valence_screens::{ScreenPlugin, game_manager::GameManager};

pub use input::{Event, Key, KeyModifiers};
pub use valence::MINECRAFT_VERSION;

const PIXEL_SIZE: u32 = 4;
const HORIZONTAL_SIZE: u32 = 22;
const VERTICAL_SIZE: u32 = 6;
pub const WIDTH: u32 = HORIZONTAL_SIZE * 2 * PIXEL_SIZE;
pub const HEIGHT: u32 = VERTICAL_SIZE * 2 * PIXEL_SIZE;
const SPAWN_Y: i32 = 64;

pub fn eventless<T>(_: &mut T, _: Event) {}

pub type MinecraftTerm = Terminal<MinecraftTermBackend>;

pub fn run<
    T: 'static + Send,
    U: FnMut(&mut T, &mut MinecraftTerm) + Send + 'static,
    E: FnMut(&mut T, Event) + Send + 'static,
>(
    state: T,
    updating_fn: U,
    event_fn: E,
) {
    let (event_sender, event_receiver) = mpsc::channel();
    App::new()
        .add_plugins((DefaultPlugins, ScreenPlugin, InputPlugin))
        .insert_resource(NetworkSettings {
            address: "0.0.0.0:25565".parse().unwrap(),
            connection_mode: ConnectionMode::Offline,
            ..Default::default()
        })
        .insert_resource(
            TerminalGameManager {
                terminal: Terminal::default(),
                state: SyncCell::new(state),
                updating_fn: SyncCell::new(updating_fn),
                event_fn: SyncCell::new(event_fn),
                event_receiver: SyncCell::new(event_receiver),
            }
            .pipe(Some)
            .pipe(TermWrapper),
        )
        .insert_resource(KeyModifiers::NONE)
        .insert_resource(event_sender.pipe(SyncCell::new).pipe(EventSender))
        .add_systems(Startup, setup::<T, U, E>)
        .add_systems(Update, init_clients)
        .register_component_as::<dyn GameManager, TerminalGameManager<T, U, E>>()
        .run();
}
