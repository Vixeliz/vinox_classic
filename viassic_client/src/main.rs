use bevy::asset::LoadState;
use bevy::prelude::*;
use viassic_common::blocks::registry::load_textures;
use viassic_common::blocks::registry::ClubeAssetRegistry;
use viassic_common::blocks::registry::ClubeHandles;
use vinox_voxel::prelude::AssetRegistry;

fn main() {
    // Create the app
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.insert_resource(ClubeHandles::default());
    app.add_startup_system(load_textures);
    app.add_system(create_registry);
    app.run();
}

pub fn create_registry(
    mut commands: Commands,
    clube_handles: Res<ClubeHandles>,
    asset_server: Res<AssetServer>,
    mut completed: Local<bool>,
    textures: ResMut<Assets<Image>>,
) {
    if !*completed {
        for handles in clube_handles.values() {
            for handle in handles.iter() {
                if asset_server.get_load_state(handle) != LoadState::Loaded {
                    return;
                }
            }
        }

        commands.insert_resource(ClubeAssetRegistry(AssetRegistry::from_block_textures(
            textures,
            clube_handles.clone(),
        )));

        *completed = true;
    }
}
