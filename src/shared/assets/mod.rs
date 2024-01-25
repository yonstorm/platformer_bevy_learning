use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub struct AssetLoadingPlugin;

impl Plugin for AssetLoadingPlugin {
    fn build(&self, app: &mut App) {
        info!("AssetLoadingPlugin");
        app.add_state::<AssetLoadingState>();
        app.add_loading_state(
            LoadingState::new(AssetLoadingState::Loading)
                .continue_to_state(AssetLoadingState::Ready),
        );
    }
}

#[derive(Clone, Debug, Default, Hash, Eq, States, PartialEq)]
pub enum AssetLoadingState {
    #[default]
    Loading,
    Ready,
}
