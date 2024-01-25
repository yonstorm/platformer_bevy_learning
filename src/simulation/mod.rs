pub mod actor;
mod world;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use self::{actor::ActorPlugin, spawning::RespawnPlugin, world::WorldPlugin};

pub struct GameSimulationPlugin;

impl Plugin for GameSimulationPlugin {
    fn build(&self, app: &mut App) {
        // TODO: does physics belong here? is this the best place?
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(16.));
        app.add_plugins(WorldPlugin);
        app.add_plugins(RespawnPlugin);
        app.add_plugins(ActorPlugin);
    }
}

#[derive(Clone, Debug, Default, Hash, Eq, States, PartialEq)]
enum GameState {
    #[default]
    Waiting,
    Started,
}

mod spawning {
    use bevy::prelude::*;
    use bevy_rapier2d::geometry::Collider;
    use rand::seq::IteratorRandom;

    use super::{
        actor::components::{Actor, ActorSimulationBundle, Player},
        world::SpawnPoint,
        GameState,
    };

    pub struct RespawnPlugin;

    impl Plugin for RespawnPlugin {
        fn build(&self, app: &mut App) {
            app.add_state::<GameState>();
            app.add_systems(
                Update,
                set_game_started_state.run_if(in_state(GameState::Waiting)),
            );
            app.add_systems(Update, spawn_player.run_if(in_state(GameState::Started)));
        }
    }

    fn set_game_started_state(mut next_state: ResMut<NextState<GameState>>) {
        next_state.set(GameState::Started);
    }

    fn spawn_player(
        mut commands: Commands,
        player_query: Query<&Actor, With<Player>>,
        spawn_query: Query<&Transform, With<SpawnPoint>>,
        input: Res<Input<KeyCode>>,
    ) {
        // TODO: placeholder spawning mechanic
        if !input.just_pressed(KeyCode::Space) {
            return;
        }

        if let Ok(_) = player_query.get_single() {
            debug!("Tried to spawn player, but player is already spawned");
            return;
        }

        let mut rng = rand::thread_rng();

        let spawn_point = match spawn_query.iter().choose(&mut rng) {
            Some(spawn_point) => spawn_point,
            None => {
                debug!("Wanted to spawn but no spawn point was found");
                return;
            }
        };

        commands.spawn((
            Player,
            ActorSimulationBundle::new(spawn_point.clone(), Collider::cuboid(8., 8.)),
        ));
    }
}
