use ahash::{HashMap, HashMapExt};
use bevy::prelude::*;
use strum::IntoEnumIterator;
use vinox_voxel::prelude::AssetRegistry;

use super::blocks::ClubeType;

#[derive(Resource, Deref, DerefMut, Default)]
pub struct ClubeHandles(pub HashMap<String, [Handle<Image>; 6]>);

#[derive(Resource, Deref, DerefMut)]
pub struct ClubeAssetRegistry(pub AssetRegistry);

pub fn load_textures(asset_server: Res<AssetServer>, mut loading: ResMut<ClubeHandles>) {
    let mut texture_array = HashMap::new();
    for name in ClubeType::iter() {
        match name {
            ClubeType::Dirt => {
                let handle: Handle<Image> = asset_server.load("textures/dirt.png");
                texture_array.insert(
                    name.to_string(),
                    [
                        handle.clone(),
                        handle.clone(),
                        handle.clone(),
                        handle.clone(),
                        handle.clone(),
                        handle.clone(),
                    ],
                );
            }
            ClubeType::Grass => {
                let handle: Handle<Image> = asset_server.load("textures/dirt.png");
                let top_handle: Handle<Image> = asset_server.load("textures/grass.png");
                let side_handle: Handle<Image> = asset_server.load("textures/grass_side.png");
                texture_array.insert(
                    name.to_string(),
                    [
                        top_handle.clone(),
                        handle.clone(),
                        side_handle.clone(),
                        side_handle.clone(),
                        side_handle.clone(),
                        side_handle.clone(),
                    ],
                );
            }
            ClubeType::Stone => {
                let handle: Handle<Image> = asset_server.load("textures/stone.png");
                texture_array.insert(
                    name.to_string(),
                    [
                        handle.clone(),
                        handle.clone(),
                        handle.clone(),
                        handle.clone(),
                        handle.clone(),
                        handle.clone(),
                    ],
                );
            }
            ClubeType::Wood => {
                let handle: Handle<Image> = asset_server.load("textures/wood.png");
                texture_array.insert(
                    name.to_string(),
                    [
                        handle.clone(),
                        handle.clone(),
                        handle.clone(),
                        handle.clone(),
                        handle.clone(),
                        handle.clone(),
                    ],
                );
            }
            _ => {}
        }
    }

    **loading = texture_array;
}
