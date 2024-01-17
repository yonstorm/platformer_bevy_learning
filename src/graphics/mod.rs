mod camera;

use bevy::prelude::*;

use self::camera::CameraPlugin;

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CameraPlugin);
    }
}
