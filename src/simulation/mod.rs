mod world;
mod actor;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use self::world::WorldPlugin;

pub struct GameSimulationPlugin;

impl Plugin for GameSimulationPlugin {
    fn build(&self, app: &mut App) {
        // TODO: does physics belong here? is this the best place?
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(8.));
        app.add_plugins(WorldPlugin);
    }
}
