#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct LevelFadeInStartSet;
use bevy::ecs::schedule::SystemSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct SyncLevelPresentationSet;
