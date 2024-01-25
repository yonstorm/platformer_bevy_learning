use bevy::{prelude::*, utils::HashMap};
use bevy_rapier2d::prelude::*;

use super::actor_actions::ActorActionState;

#[derive(Component)]
pub struct Actor;

// TODO: this module should probably not have player? as it deals with all actors
#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(Bundle)]
pub struct ActorSimulationBundle {
    pub actor: Actor,
    pub collider: Collider,
    pub spatial_bundle: SpatialBundle,
    pub character_controller: KinematicCharacterController,
    pub velocity: Velocity,
    pub intents: ActorActionState,
}

impl ActorSimulationBundle {
    pub fn new(transform: Transform, collider: Collider) -> Self {
        Self {
            actor: Actor,
            collider,
            spatial_bundle: SpatialBundle {
                transform,
                ..Default::default()
            },
            character_controller: KinematicCharacterController {
                snap_to_ground: None,
                ..Default::default()
            },
            velocity: Velocity(Vec2::new(0., 0.)),
            intents: ActorActionState {
                states: HashMap::new(),
            },
        }
    }
}
