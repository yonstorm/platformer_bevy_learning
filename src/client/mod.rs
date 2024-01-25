mod input;

use bevy::prelude::*;

use self::input::InputPlugin;

pub struct ClientPlugin;

// TODO: should probably move window stuff to its own module
impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: (960., 600.).into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
        );

        app.add_plugins(InputPlugin);
    }
}
