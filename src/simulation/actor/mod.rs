use bevy::prelude::*;
use bevy_rapier2d::{geometry::Collider, control::{KinematicCharacterController, KinematicCharacterControllerOutput}, dynamics::RigidBody};

use crate::shared::GameAction;

pub struct ActorPlugin;

impl Plugin for ActorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_actor_velocity);
        app.add_systems(Update, handle_actor_movement);
    }
}

#[derive(Component)]
pub struct Actor;

// TODO: this module should probably not have player? as it deals with all actors
#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Velocity(Vec2);

#[derive(Bundle)]
pub struct ActorSimulationBundle {
    pub actor: Actor,
    pub collider: Collider,
    pub spatial_bundle: SpatialBundle,
    pub character_controller: KinematicCharacterController,
    pub velocity: Velocity,
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
        }
    }
}

const GRAVITY_SCALE: f32 = 15.;
const GRAVITY: f32 = -9.8;

const JUMP_HEIGHT: f32 = 24.;

fn update_actor_velocity(
    mut events: EventReader<GameAction>,
    mut velocity_query: Query<(&mut Velocity, &KinematicCharacterControllerOutput), With<Actor>>,
    time: Res<Time>,
) {
    for (mut velocity, output) in &mut velocity_query {
        velocity.0.x = 0.;
        for event in events.read() {
            match event {
                GameAction::MoveLeft | GameAction::MoveRight => {
                    let right = if let GameAction::MoveRight = event { 1. } else { 0. };
                    let left = if let GameAction::MoveLeft = event { 1. } else { 0. };

                    velocity.0.x = (right - left) * 100. * time.delta_seconds();
                },
                GameAction::Jump => {
                    if output.grounded {
                        velocity.0.y = f32::sqrt(JUMP_HEIGHT * (GRAVITY * GRAVITY_SCALE) * -2.);
                    }

                }
            }
        }

        //velocity.0.y += -70. * time.delta_seconds();
        velocity.0.y += (GRAVITY * GRAVITY_SCALE) * time.delta_seconds();
        debug!("velocity: {:?}", velocity.0.y);
        //velocity.0.y = velocity.0.y.max(0.0);
    }
}

fn handle_actor_movement(
    mut player_query: Query<(&Velocity, &mut KinematicCharacterController,Option<&KinematicCharacterControllerOutput>), With<Actor>>,
    time: Res<Time>,
) {
    for (velocity, mut controller,output) in &mut player_query{
        let mut translation = match controller.translation {
            Some(translation) => {
                translation
            },
            None => {
                Vec2::new(0., 0.)
            }
        };

        translation.x = velocity.0.x;
        translation.y = velocity.0.y * time.delta_seconds();
        controller.translation = Some(translation);
    }
}
