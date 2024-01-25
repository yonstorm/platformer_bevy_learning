use bevy::prelude::*;

use crate::simulation::actor::components::Player;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera);
        app.add_systems(Update, camera_follow_player);
    }
}

fn setup_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scale = 0.5;
    commands.spawn((camera, MainCamera));
}

#[derive(Component)]
struct MainCamera;

fn camera_follow_player(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
) {
    let player_transform = match player_query.get_single() {
        Ok(player_transform) => player_transform,
        Err(_) => {
            return;
        }
    };

    let mut camera_transform = match camera_query.get_single_mut() {
        Ok(camera_transform) => camera_transform,
        Err(_) => {
            debug!("No camera found");
            return;
        }
    };

    camera_transform.translation.x = player_transform.translation.x;
    camera_transform.translation.y = player_transform.translation.y;
}
