use itertools::Itertools;

use bevy_ecs::system::{Commands, Res, ResMut};
use valence::{
    BlockState, LayerBundle, Server,
    entity::{EntityLayerId, Position},
    ident,
    math::DVec3,
    prelude::{BiomeRegistry, DimensionTypeRegistry, UnloadedChunk},
};

use crate::{
    Event, HEIGHT, HORIZONTAL_SIZE, MinecraftTerm, PIXEL_SIZE, SPAWN_Y, WIDTH,
    game_manager::TermWrapper,
};

fn for_two_dimensional_range<F: FnMut(i32, i32) -> T, T>(num: i32, mut closure: F) {
    let range = -num..num;
    range
        .clone()
        .cartesian_product(range)
        .for_each(|(x, y)| _ = closure(x, y));
}

const SCREEN_Z_OFFSET: i32 = 8;
pub fn setup<
    T: Send + 'static,
    U: Send + 'static + FnMut(&mut T, &mut MinecraftTerm),
    E: Send + 'static + FnMut(&mut T, Event),
>(
    mut commands: Commands,
    server: Res<Server>,
    dimensions: Res<DimensionTypeRegistry>,
    biomes: Res<BiomeRegistry>,
    mut term: ResMut<TermWrapper<T, U, E>>,
) {
    const SCREEN_POS: DVec3 = DVec3::new(
        HORIZONTAL_SIZE as f64 / -2.,
        SPAWN_Y as f64 + 1.,
        -SCREEN_Z_OFFSET as f64,
    );

    let mut layer = LayerBundle::new(ident!("overworld"), &dimensions, &biomes, &server);

    const CHUNK_RANGE: i32 = 5;
    for_two_dimensional_range(CHUNK_RANGE, |x, z| {
        layer.chunk.insert_chunk([x, z], UnloadedChunk::new())
    });

    for_two_dimensional_range(HORIZONTAL_SIZE as i32 / 2, |x, z| {
        layer
            .chunk
            .set_block([x, SPAWN_Y, z], BlockState::DEEPSLATE_TILES)
    });

    let layer_id = commands.spawn(layer).id();

    let _ = valence_screens::build_screen(
        &mut commands,
        EntityLayerId(layer_id),
        Position(SCREEN_POS),
        WIDTH,
        HEIGHT,
        PIXEL_SIZE,
        true,
        term.0.take().unwrap(),
    );
}
