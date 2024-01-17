use bevy::{app::ScheduleRunnerPlugin, prelude::*, utils::Duration, winit::WinitPlugin};

pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        info!("ServerPlugin active");
        app.add_plugins(DefaultPlugins.build().disable::<WinitPlugin>());
        app.add_plugins(
            ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
                1.0/60.0,
            )),
        );
    }
}
