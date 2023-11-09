use bevy::prelude::*;

#[derive(Debug, Component, Clone, Reflect)]
pub enum WarriorKind {
    Jotaile,
    // Ed,
    // Rod,
    // Turi,
    // Fred,
}

#[derive(Debug, Component, Reflect, Default)]
#[reflect(Component)]
pub struct Speed {
    pub walk: f32,
    pub jump: f32,
}
