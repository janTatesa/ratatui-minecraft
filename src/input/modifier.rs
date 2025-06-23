use bevy_ecs::{
    entity::Entity,
    query::With,
    system::{Commands, Query, Res, Resource},
};
use bitflags::bitflags;
use valence::{
    ChunkLayer, EntityLayer,
    command_macros::Command,
    entity::EntityLayerId,
    scoreboard::{Objective, ObjectiveBundle, ObjectiveDisplay, ObjectiveScores},
    text::{Color, IntoText},
};
bitflags! {
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(transparent))]
    #[derive(Resource, Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
    pub struct KeyModifiers: u8 {
        const SHIFT = 0b0000_0001;
        const CONTROL = 0b0000_0010;
        const ALT = 0b0000_0100;
        const NONE = 0b0000_0000;
    }
}

fn objective_scores_from_modifiers(modifiers: KeyModifiers) -> ObjectiveScores {
    ObjectiveScores::with_map([
        (
            "shift".to_string(),
            modifiers.contains(KeyModifiers::SHIFT) as i32,
        ),
        (
            "ctrl".to_string(),
            modifiers.contains(KeyModifiers::CONTROL) as i32,
        ),
        (
            "alt".to_string(),
            modifiers.contains(KeyModifiers::ALT) as i32,
        ),
    ])
}

pub fn setup_scoreboard(
    layer: Query<Entity, (With<ChunkLayer>, With<EntityLayer>)>,
    mut commands: Commands,
) {
    _ = commands.spawn(ObjectiveBundle {
        name: Objective::new("modifiers"),
        display: ObjectiveDisplay("modifiers".color(Color::RED)),
        scores: objective_scores_from_modifiers(KeyModifiers::NONE),
        layer: EntityLayerId(layer.single()),
        ..Default::default()
    })
}

pub fn update_scoreboard(mut scores: Query<&mut ObjectiveScores>, modifiers: Res<KeyModifiers>) {
    *scores.single_mut() = objective_scores_from_modifiers(*modifiers);
}

#[derive(Command, Debug, Clone, Copy)]
#[paths("toggle", "t")]
#[scopes("valence.command.toggle")]
pub enum ToggleCommand {
    #[paths("shift")]
    Shift,
    #[paths("control")]
    Control,
    #[paths("alt")]
    Alt,
}
