mod shared;
mod simulation;
mod client;
mod graphics;
mod debugging;
mod server;

use bevy::prelude::*;
use client::ClientPlugin;
use debugging::DebugPlugin;
use graphics::GraphicsPlugin;
#[cfg(feature = "server")]
use server::ServerPlugin;
use simulation::GameSimulationPlugin;
use shared::{assets::AssetLoadingPlugin, SharedEventsPlugin};

fn main() {
    let mut app = App::new();
    app.add_plugins(AssetLoadingPlugin);

    app.add_plugins(SharedEventsPlugin);

    #[cfg(not(feature = "server"))]
    app.add_plugins(ClientPlugin)
        .add_plugins(GraphicsPlugin);
    #[cfg(feature = "server")]
    app.add_plugins(ServerPlugin);

    app.add_plugins(GameSimulationPlugin);

    #[cfg(debug_assertions)]
    app.add_plugins(DebugPlugin);

    app.run();
}
