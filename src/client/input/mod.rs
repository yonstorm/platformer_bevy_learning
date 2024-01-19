
use bevy::{prelude::*, utils::HashMap};

use crate::shared::GameAction;

#[derive(Resource)]
struct InputMapping {
    key_to_action: HashMap<KeyCode, GameAction>
}

impl Default for InputMapping {
    fn default() -> Self {
        let mut key_to_action = HashMap::new();
        key_to_action.insert(KeyCode::A, GameAction::MoveLeft);
        key_to_action.insert(KeyCode::D, GameAction::MoveRight);
        key_to_action.insert(KeyCode::Space, GameAction::Jump);

        InputMapping { key_to_action }
    }
}

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InputMapping>();
        app.add_systems(Update, map_input_to_events); 
    }
}

fn map_input_to_events(
    keyboard_input: Res<Input<KeyCode>>,
    input_mapping: Res<InputMapping>,
    mut action_events: EventWriter<GameAction>,
) {
    for (key, action) in input_mapping.key_to_action.iter() {
        if keyboard_input.just_pressed(*key) | keyboard_input.pressed(*key) {
            //debug!("sending action: {:?}", action);
            action_events.send(*action);
        }
    }
}
