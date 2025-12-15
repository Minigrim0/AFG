use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use serde::Deserialize;

use super::state::AppState;

#[derive(Debug, Deserialize)]
pub struct Wall {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

#[derive(serde::Deserialize, bevy::asset::Asset, bevy::reflect::TypePath)]
pub struct Map {
    pub title: String,
    pub size: (i32, i32),
    pub tile_size: i32,
    pub spawn_places: ((i32, i32, i32, i32), (i32, i32, i32, i32)),
    pub walls: Vec<Wall>,
}

#[derive(Resource)]
pub struct MapHandle(pub Handle<Map>);

/// Loads a map from a toml file
pub fn setup_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    let map = MapHandle(asset_server.load("maps/level1.map.toml"));
    commands.insert_resource(map);
}

pub fn spawn_map(
    mut commands: Commands,
    map: Res<MapHandle>,
    maps: ResMut<Assets<Map>>,
    mut state: ResMut<NextState<AppState>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if let Some(map) = maps.get(map.0.id()) {
        let tile_size = map.tile_size as f32;
        for wall in map.walls.iter() {
            commands
                .spawn(RigidBody::Fixed)
                .insert(Collider::cuboid(
                    (wall.width as f32 * tile_size) / 2.0,
                    (wall.height as f32 * tile_size) / 2.0,
                ))
                .insert(Transform::from_xyz(
                    wall.x as f32 * tile_size + (wall.width as f32 * tile_size) / 2.0,
                    wall.y as f32 * tile_size + (wall.height as f32 * tile_size) / 2.0,
                    0.0,
                ))
                .insert(Mesh2d(meshes.add(Rectangle::new(
                    wall.width as f32 * tile_size,
                    wall.height as f32 * tile_size,
                ))))
                .insert(MeshMaterial2d(
                    materials.add(ColorMaterial::from_color(Color::srgb(0.2, 0.2, 0.3))),
                ));
        }
        state.set(AppState::Running);
    }
}
