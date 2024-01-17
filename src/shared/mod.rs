pub mod assets;

use bevy::prelude::*;

pub struct SharedEventsPlugin;

impl Plugin for SharedEventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GameAction>();
    }
}

// TODO: Game related event stuff should probably be moved to simulation module
// Shared module usage is for data/plugins/etc that is needed in multiple places but does not have
// a clear module it belongs to
#[derive(Event, Copy, Clone, Debug)]
pub enum GameAction {
    MoveLeft,
    MoveRight,
    Jump,
}
