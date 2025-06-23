use crate::{MinecraftTerm, input::Event};

use std::sync::mpsc::Receiver;

use bevy::utils::synccell::SyncCell;
use bevy_ecs::system::Resource;
use valence::prelude::Component;
use valence_screens::{buffer::ScreenBuffer, game_manager::GameManager};

// HACK: To have ownership of a mutable resource we need to wrap it in an option
#[derive(Resource)]
pub struct TermWrapper<T, U, E>(pub Option<TerminalGameManager<T, U, E>>);

#[derive(Component)]
pub struct TerminalGameManager<T, U, E> {
    pub state: SyncCell<T>,
    pub terminal: MinecraftTerm,
    pub updating_fn: SyncCell<U>,
    pub event_fn: SyncCell<E>,
    pub event_receiver: SyncCell<Receiver<Event>>,
}

impl<
    T: 'static + Send,
    U: FnMut(&mut T, &mut MinecraftTerm) + Send + 'static,
    E: FnMut(&mut T, Event) + Send + 'static,
> GameManager for TerminalGameManager<T, U, E>
{
    fn init(&mut self, _width: u32, _height: u32, _has_fg: bool) {}

    fn draw(&self) -> ScreenBuffer {
        self.terminal.backend().screen_buffer()
    }

    fn tick(&mut self) {
        if let Ok(event) = self.event_receiver.get().try_recv() {
            (self.event_fn.get())(self.state.get(), event)
        }

        (self.updating_fn.get())(self.state.get(), &mut self.terminal)
    }
}
