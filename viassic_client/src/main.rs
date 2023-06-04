use bevy::asset::LoadState;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use viassic_common::blocks::registry::load_textures;
use viassic_common::blocks::registry::ClubeAssetRegistry;
use viassic_common::blocks::registry::ClubeHandles;
use vinox_voxel::prelude::AssetRegistry;

#[derive(Default, States, Debug, Hash, PartialEq, Eq, Clone)]
pub enum GameState {
    #[default]
    Loading,
    Menu,
    Game,
}

fn main() {
    // Create the app
    let mut app = App::new();
    app.add_state::<GameState>();
    app.add_plugins(DefaultPlugins);
    app.add_plugin(WorldInspectorPlugin::new());
    app.insert_resource(ClubeHandles::default());
    app.add_system(load_textures.in_schedule(OnEnter(GameState::Loading)));
    app.add_system(game_setup.in_schedule(OnEnter(GameState::Game)));
    app.add_system(create_registry.in_set(OnUpdate(GameState::Loading)));
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

        commands.insert_resource(NextState(Some(GameState::Game)));

        *completed = true;
    }
}

pub fn game_setup(mut commands: Commands) {
    commands.spawn(Camera3dBundle::default());
}
