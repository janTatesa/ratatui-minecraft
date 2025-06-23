use bevy_ecs::{
    entity::Entity,
    query::{Added, With},
    system::Query,
};

use valence::{
    ChunkLayer, EntityLayer, GameMode, Text,
    client::{Client, VisibleChunkLayer, VisibleEntityLayers},
    command::scopes::CommandScopes,
    entity::{EntityLayerId, Look, Position},
    message::SendMessage,
    op_level::OpLevel,
    text::{Color, IntoText},
};

use crate::SPAWN_Y;

#[allow(clippy::type_complexity)]
pub fn init_clients(
    mut clients: Query<
        (
            &mut Client,
            &mut EntityLayerId,
            &mut VisibleChunkLayer,
            &mut VisibleEntityLayers,
            &mut CommandScopes,
            &mut Position,
            &mut Look,
            &mut GameMode,
            &mut OpLevel,
        ),
        Added<Client>,
    >,
    layer: Query<Entity, (With<ChunkLayer>, With<EntityLayer>)>,
) {
    for (
        mut client,
        mut layer_id,
        mut visible_chunk_layer,
        mut visible_entity_layers,
        mut permissions,
        mut pos,
        mut look,
        mut game_mode,
        mut op_level,
    ) in &mut clients
    {
        let layer = layer.single();
        layer_id.0 = layer;
        visible_chunk_layer.0 = layer;
        visible_entity_layers.0.insert(layer);

        pos.set([0., SPAWN_Y as f64 + 1., 0.]);
        const FACING_NORTH: f32 = 180.;
        look.yaw = FACING_NORTH;

        *game_mode = GameMode::Creative;
        const MAX_PERMS: u8 = 3;
        op_level.set(MAX_PERMS);
        permissions.add("valence.command");

        client.send_chat_message(chat_message());
    }
}

fn chat_message() -> Text {
    "Welcome to ratatui-minecraft. To innteract with the terminal use the following commands"
        .color(Color::GRAY)
        + command_hint('i', "input", "Input a sequence of characters")
        + command_hint(
            'l',
            "inputln",
            "Input a sequence of characters and add an enter afterwards",
        )
        + command_hint('k', "key", "Press a special key")
        + command_hint('t', "toggle", "Toggle a key modifier")
}

fn command_hint(short: char, long: &'static str, desc: &'static str) -> Text {
    "\n".into_text()
        + short.color(Color::RED)
        + ", ".color(Color::GRAY)
        + long.color(Color::RED)
        + " - ".color(Color::GRAY)
        + desc.color(Color::WHITE)
}
