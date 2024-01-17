use bevy::prelude::*;
use bevy_rapier2d::render::{RapierDebugRenderPlugin, DebugRenderMode};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        // Debug physics
        debug!("debug plugin active");
        app.add_plugins(RapierDebugRenderPlugin {
            mode: DebugRenderMode::all(),
            ..Default::default()
        });
    }
}
