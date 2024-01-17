use bevy::prelude::*;
use bevy_asset_loader::{asset_collection::AssetCollection, loading_state::{LoadingStateAppExt, config::{LoadingStateConfig, ConfigureLoadingState}}};
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::geometry::Collider;

use crate::shared::assets::AssetLoadingState;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
       app
           .insert_resource(LevelSelection::index(0))
           .add_plugins(LdtkPlugin);

       app.configure_loading_state(
           LoadingStateConfig::new(AssetLoadingState::Loading)
           .load_collection::<LevelAsset>()
       );

       app.add_systems(OnEnter(AssetLoadingState::Ready), load_world);
       app.add_systems(Update, add_world_cell_colliders);
       app.add_systems(Update, add_world_spawn_points);

       // TODO: think how to better handle debugging behaviour, maybe the debug module approach is
       // not the best way, if is move this there
       #[cfg(debug_assertions)]
       app.add_systems(Update, debug_draw_spawn_point_gizmos);

    }
}

#[derive(AssetCollection, Resource)]
struct LevelAsset {
    #[asset(path = "map.ldtk")]
    handle: Handle<LdtkProject>
}

fn load_world(
    mut commands: Commands,
    level: Res<LevelAsset>
    )
{
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: level.handle.clone(),
        ..Default::default()
    });
}

// TODO: should the creation of colliders be moved to a phycics simulation module?
// or should the collider generation stay in world module?
fn add_world_cell_colliders(
        mut commands: Commands, 
        cell_query: Query<Entity, Added<IntGridCell>>
        )
{
    let mut count = 0;
    cell_query.for_each(|entity| {
        count += 1;
        commands.entity(entity)
            .insert(Collider::cuboid(4., 4.));
    });

    if count > 0 {
        info!("added {:?} colliders", count);
    }
}

#[derive(Component)]
struct SpawnPoint;

fn add_world_spawn_points(
    mut commands: Commands,
    entity_query: Query<Entity, Added<EntityInstance>>
    )
{
    entity_query.for_each(|entity| {
        commands.entity(entity)
            .insert(SpawnPoint);
    });
}

fn debug_draw_spawn_point_gizmos(
    mut gizmos: Gizmos,
    spawn_points: Query<&Transform, With<SpawnPoint>>
    )
{
    spawn_points.for_each(|transform| {
        let position = Vec2::new(transform.translation.x, transform.translation.y); 
        gizmos.circle_2d(position, 8., Color::PINK);
    });
}
