use bevy::{prelude::*, utils::HashMap};

use crate::simulation::actor::actor_actions::{GameActionEvent, ActorAction, GameActionState};

#[derive(Resource)]
struct InputMapping {
    key_to_action: HashMap<KeyCode, ActorAction>
}

impl Default for InputMapping {
    fn default() -> Self {
        let mut key_to_action = HashMap::new();
        key_to_action.insert(KeyCode::A, ActorAction::MoveLeft);
        key_to_action.insert(KeyCode::D, ActorAction::MoveRight);
        key_to_action.insert(KeyCode::Space, ActorAction::Jump);

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
    mut action_events: EventWriter<GameActionEvent<ActorAction>>,
) {
    for (key, action) in input_mapping.key_to_action.iter() {
        if keyboard_input.just_pressed(*key) {
            debug!("sending action: {:?}", action);
            let event = GameActionEvent::<ActorAction>{
                state: GameActionState::Initiated,
                action: *action,
            };
            action_events.send(event);
        }
        else if keyboard_input.pressed(*key) {
            let event = GameActionEvent::<ActorAction>{
                state: GameActionState::OnGoing,
                action: *action,
            };
            action_events.send(event);
        }
        else if keyboard_input.just_released(*key) {
            let event = GameActionEvent::<ActorAction>{
                state: GameActionState::Completed,
                action: *action,
            };
            action_events.send(event);
        }
    }
}
