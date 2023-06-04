use ahash::HashMap;
use bevy::asset::LoadState;
use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::render_resource::PrimitiveTopology;
use bevy_flycam::PlayerPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use viassic_common::blocks::blocks::Clube;
use viassic_common::blocks::blocks::ClubeRegistry;
use viassic_common::blocks::registry::load_textures;
use viassic_common::blocks::registry::ClubeAssetRegistry;
use viassic_common::blocks::registry::ClubeHandles;
use vinox_voxel::mesh::mesh::full_mesh;
use vinox_voxel::prelude::AssetRegistry;
use vinox_voxel::prelude::*;

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
    app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()).build());
    app.add_plugin(PlayerPlugin);
    app.add_plugin(WorldInspectorPlugin::new());
    app.insert_resource(ClubeHandles::default());
    app.insert_resource(Msaa::Off);
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

pub fn game_setup(
    mut commands: Commands,
    clube_asset_registry: Res<ClubeAssetRegistry>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // commands.spawn(Camera3dBundle::default());

    let mut chunk = ChunkData::<Clube, ClubeRegistry>::default();
    chunk.set(
        RelativeVoxelPos::new(8, 4, 8),
        Clube {
            identifier: viassic_common::blocks::blocks::ClubeType::Grass,
            geometry: BlockGeometry::Block,
            visibility: VoxelVisibility::Opaque,
        },
    );
    for y in 0..2 {
        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                if y == 1 {
                    if x == CHUNK_SIZE - 1 || z == CHUNK_SIZE - 1 || x == 0 || z == 0 {
                        chunk.set(
                            RelativeVoxelPos::new(x as u32, y + 1, z as u32),
                            Clube {
                                identifier: viassic_common::blocks::blocks::ClubeType::Grass,
                                geometry: BlockGeometry::Block,
                                visibility: VoxelVisibility::Opaque,
                            },
                        );
                        continue;
                    } else {
                        chunk.set(
                            RelativeVoxelPos::new(x as u32, y + 1, z as u32),
                            Clube {
                                identifier: viassic_common::blocks::blocks::ClubeType::Dirt,
                                geometry: BlockGeometry::Block,
                                visibility: VoxelVisibility::Opaque,
                            },
                        );
                        continue;
                    }
                }
                chunk.set(
                    RelativeVoxelPos::new(x as u32, y + 1, z as u32),
                    Clube {
                        identifier: viassic_common::blocks::blocks::ClubeType::Stone,
                        geometry: BlockGeometry::Block,
                        visibility: VoxelVisibility::Opaque,
                    },
                );
            }
        }
    }

    let mut geo_table = GeometryRegistry(HashMap::default());
    geo_table.insert("vinox:block".to_string(), Geometry::default());
    geo_table.insert(
        "vinox:slab".to_string(),
        Geometry {
            namespace: "vinox".to_string(),
            name: "slab".to_string(),
            blocks: [false, false, true, false, false, false],
            blocks_self: Some([true, true, false, false, true, true]),
            element: BlockGeo {
                pivot: (0, 0, 0),
                rotation: (0, 0, 0),
                cubes: vec![FaceDescript {
                    uv: [
                        ((0, 0), (16, 8)),
                        ((0, 0), (16, 8)),
                        ((16, 16), (-16, -16)),
                        ((16, 16), (-16, -16)),
                        ((0, 0), (16, 8)),
                        ((0, 0), (16, 8)),
                    ],
                    discard: [false, false, false, false, false, false],
                    texture_variance: [false, false, false, false, false, false],
                    cull: [true, true, true, false, true, true],
                    origin: (0, 0, 0),
                    end: (16, 8, 16),
                    rotation: (0, 0, 0),
                    pivot: (8, 8, 8),
                }],
            },
        },
    );

    let mesh = full_mesh(
        &ChunkBoundary::<Clube, ClubeRegistry>::new(
            chunk,
            Box::default(),
            &ClubeRegistry::default(),
            &geo_table,
            &*clube_asset_registry,
        ),
        &clube_asset_registry.texture_atlas,
        IVec3::new(0, 0, 0),
    );
    // println!("{}", mesh.chunk_mesh.vertices.len());
    let mut bevy_mesh = Mesh::new(PrimitiveTopology::TriangleList);
    bevy_mesh.set_indices(Some(Indices::U32(mesh.chunk_mesh.indices)));
    bevy_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, mesh.chunk_mesh.vertices.clone());
    bevy_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, mesh.chunk_mesh.normals);

    if let Some(mesh_colors) = mesh.chunk_mesh.colors {
        bevy_mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, mesh_colors);
    }
    if let Some(mesh_uvs) = mesh.chunk_mesh.uvs {
        bevy_mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, mesh_uvs);
    }

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.2,
    });

    // commands.spawn(PointLightBundle {
    //     point_light: PointLight::default(),
    //     transform: Transform::from_xyz(8.0, 3.0, 8.0),
    //     ..default()
    // });

    commands.spawn(PbrBundle {
        mesh: meshes.add(bevy_mesh),
        material: materials.add(StandardMaterial {
            base_color: Color::WHITE,
            base_color_texture: Some(clube_asset_registry.texture_atlas.texture.clone()),
            perceptual_roughness: 1.0,
            ..Default::default()
        }),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..Default::default()
    });
}
