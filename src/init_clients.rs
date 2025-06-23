use bevy_ecs::{
    entity::Entity,
    query::{Added, With},
    system::Query,
};
use valence::{
    ChunkLayer, EntityLayer, GameMode,
    client::{Client, VisibleChunkLayer, VisibleEntityLayers},
    command::scopes::CommandScopes,
    entity::{EntityLayerId, Look, Position},
    op_level::OpLevel,
};

use crate::SPAWN_Y;

#[allow(clippy::type_complexity)]
pub fn init_clients(
    mut clients: Query<
        (
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
        look.yaw = 180.;
        *game_mode = GameMode::Creative;
        op_level.set(4);
        permissions.add("valence.command");
    }
}
