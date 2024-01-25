mod client;
mod debugging;
mod graphics;
mod server;
mod shared;
mod simulation;

use bevy::prelude::*;
use client::ClientPlugin;
use debugging::DebugPlugin;
use graphics::GraphicsPlugin;
#[cfg(feature = "server")]
use server::ServerPlugin;
use shared::assets::AssetLoadingPlugin;
use simulation::GameSimulationPlugin;

fn main() {
    let mut app = App::new();
    app.add_plugins(AssetLoadingPlugin);

    //app.add_plugins(SharedEventsPlugin);

    #[cfg(not(feature = "server"))]
    app.add_plugins(ClientPlugin).add_plugins(GraphicsPlugin);
    #[cfg(feature = "server")]
    app.add_plugins(ServerPlugin);

    app.add_plugins(GameSimulationPlugin);

    #[cfg(debug_assertions)]
    app.add_plugins(DebugPlugin);

    app.run();
}
