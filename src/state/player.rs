use crate::kind::Position;

pub(crate) struct Player {
    pub entity: bevy::ecs::Entity,
    pub position: Position,
}
