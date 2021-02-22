use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, StageLabel)]
pub enum AppStage {
    Update,
    Physics,
    View,
}
