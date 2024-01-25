use bevy::{prelude::*, utils::{HashMap }};
use bevy_rapier2d::{control::{KinematicCharacterController, KinematicCharacterControllerOutput}};

use self::{actor_actions::{GameActionEvent, ActorAction, GameActionState, ActorActionState}, components::*};

pub mod components;

pub struct ActorPlugin;

impl Plugin for ActorPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GameActionEvent<ActorAction>>();
        app.add_systems(Update, (collect_actor_intents,
                                 update_actor_velocity,
                                 actor_move_horizontal,
                                 apply_actor_friction,
                                 handle_actor_movement).chain());
    }
}


const GRAVITY_SCALE: f32 = 15.;
const GRAVITY: f32 = -9.8;

const TILE_HEIGHT: f32 = 16.;
const JUMP_HEIGHT: f32 = TILE_HEIGHT * 4.;

// TODO: handle state change stuff here. ie. time since last change
fn collect_actor_intents(
    mut events: EventReader<GameActionEvent<ActorAction>>,
    mut actors: Query<&mut ActorActionState, With<Actor>>,
) {
    for mut intents in &mut actors {
        for event in events.read() {
            intents.states.insert(event.action, *event);
        }
    }
}

fn apply_actor_friction(
    mut velocity_query: Query<(&mut Velocity, &KinematicCharacterControllerOutput), With<Actor>>,
    time: Res<Time>,
) {
    for (mut velocity, output) in &mut velocity_query{
        // do not apply friction if in air
        if !output.grounded{
            continue;
        }

        let direction = velocity.0.x.signum();
        // friction
        let friction = ACCELERATION * 0.6 * time.delta_seconds();
        // Apply friction in the direction opposite to the velocity
        velocity.0.x -= velocity.0.x.signum() * friction;

        // Clamp the velocity to zero if friction has reversed its direction
        if velocity.0.x.signum() != direction.signum() {
            velocity.0.x = 0.0;
        }   
    }
}

const ACCELERATION: f32 = 2.5 * 16.0; // meters * pixelsPerMeter = meters per second
fn actor_move_horizontal(
    mut velocity_query: Query<(&mut Velocity,&ActorActionState, &KinematicCharacterControllerOutput), With<Actor>>,
    time: Res<Time>,
) {
    for (mut velocity, action_state, output) in &mut velocity_query {
        let mut direction = 0.;
        for (action, event) in &action_state.states{
            match action {
                ActorAction::MoveLeft | ActorAction::MoveRight => {
                    match event.state {
                        GameActionState::Initiated | GameActionState::OnGoing => {
                            let right = if let ActorAction::MoveRight = action { 1. } else { 0. };
                            let left = if let ActorAction::MoveLeft = action { 1. } else { 0. };
                            
                            direction = right - left;
                        },
                        GameActionState::Completed => {
                        },
                    }

                },
                _ => {}
            }
        }

        // TODO: ugly hack, move to its own system? Velocity/acceleration should not accumulate
        // when colliding, also the same reset should happen when jumping and hitting the ceiling
        let mut accel_scale = 1.;
        for collision in &output.collisions {
            let toi = collision.toi;
            if let Some(details) = toi.details {
                if details.normal1.y == 1.0 || details.normal2.y == -1.0 {
                    continue;
                }
                
                let side_collision = details.normal1.x.abs() > 0. || details.normal2.x.abs() > 0.;
                if side_collision {
                    debug!("side collision: {:?}", details.normal1.x); 
                    accel_scale = 1. - details.normal1.x.abs(); 
                }
            }
        }

        if !output.grounded {
            continue;
        }
        // Instant direction change
        if direction != 0.0 && velocity.0.x.signum() != direction {
            velocity.0.x = 0.0; // Reset velocity for instant direction change
        }

        // Calculate dynamic acceleration
        let current_speed = velocity.0.x.abs();
        let dynamic_acceleration = ACCELERATION / (1.0 + current_speed);

        // Apply acceleration
        velocity.0.x += direction * dynamic_acceleration * time.delta_seconds() * accel_scale;   
    }
}

fn update_actor_velocity(
    mut velocity_query: Query<(&mut Velocity,&ActorActionState, &KinematicCharacterControllerOutput), With<Actor>>,
    time: Res<Time>,
) {
    for (mut velocity, action_state, output) in &mut velocity_query {
        for intent in &action_state.states{
            match intent.0 {
                ActorAction::Jump => {
                    match intent.1.state {
                        GameActionState::Initiated => {
                            if output.grounded {
                                velocity.0.y = f32::sqrt(JUMP_HEIGHT * (GRAVITY * GRAVITY_SCALE) * -2.);
                            }
                        },
                        GameActionState::OnGoing => {
                        },
                        _ => {}
                    }

                },
                _ => {}
            }
        }
        velocity.0.y += (GRAVITY * GRAVITY_SCALE) * time.delta_seconds();
    }
}

fn handle_actor_movement(
    mut player_query: Query<(&Velocity, &mut KinematicCharacterController), With<Actor>>,
    time: Res<Time>,
) {
    for (velocity, mut controller) in &mut player_query{
        let mut translation = Vec2::new(0., 0.);

        translation.x = velocity.0.x;
        translation.y = velocity.0.y * time.delta_seconds();
        controller.translation = Some(translation);
    }
}

pub mod actor_actions {
    use bevy::{utils::HashMap, ecs::{component::Component, event::Event}};

    #[derive(PartialEq,Copy,Clone, Eq)]
    pub enum GameActionState {
        Initiated,
        OnGoing,
        Completed,
    }

    #[derive(PartialEq, Copy, Clone, Eq, Hash, Debug)]
    pub enum ActorAction {
        Jump,
        MoveLeft,
        MoveRight,
    }

    #[derive(Event, Copy, Clone)]
    pub struct GameActionEvent<T>
    where
        T: PartialEq
    {
        pub state: GameActionState,
        pub action: T,
        //duration: f32, 
        //intensity: f32,
    }

    #[derive(Component)]
    pub struct ActorActionState {
        pub states: HashMap<ActorAction, GameActionEvent<ActorAction>>,
    }
}
