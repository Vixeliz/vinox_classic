use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString};
use vinox_voxel::prelude::*;

#[derive(EnumString, Display, EnumIter, PartialEq, Eq, Clone, Serialize, Deserialize, Default)]
pub enum ClubeType {
    #[default]
    Air,
    Dirt,
    Grass,
    Stone,
    Wood,
}

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Clube {
    pub identifier: ClubeType,
    pub texture: usize,
    pub geometry: BlockGeometry,
    pub visibility: VoxelVisibility,
}

impl Default for Clube {
    fn default() -> Self {
        Self {
            identifier: ClubeType::Air,
            texture: 0,
            geometry: BlockGeometry::Block,
            visibility: VoxelVisibility::Empty,
        }
    }
}

#[derive(Default, Clone)]
pub struct ClubeRegistry {}

impl VoxRegistry<Clube> for ClubeRegistry {
    fn is_empty(&self, vox: Clube) -> bool {
        vox.visibility == VoxelVisibility::Empty
    }
}

impl Voxel<ClubeRegistry> for Clube {
    fn is_empty(&self, _registry: Option<&ClubeRegistry>) -> bool {
        self.visibility == VoxelVisibility::Empty
    }

    fn is_true_empty(&self, _registry: Option<&ClubeRegistry>) -> bool {
        self.visibility == VoxelVisibility::Empty
    }

    fn is_opaque(&self, _registry: Option<&ClubeRegistry>) -> bool {
        self.visibility == VoxelVisibility::Opaque
    }

    fn identifier(&self) -> String {
        self.identifier.to_string()
    }
}

impl RenderedVoxel<Clube, ClubeRegistry> for Clube {
    fn to_geo_idx(
        &self,
        geo_pal: Option<&mut GeoPalette>,
        geo_registry: Option<&GeometryRegistry>,
        _vox_registry: Option<&ClubeRegistry>,
    ) -> Option<usize> {
        if let Some(geo_registry) = geo_registry {
            // if let Some(vox_registry) = vox_registry {
            // if let Some(block_data) = vox_registry.get(&self.identifier) {
            if let Some(geo_pal) = geo_pal {
                let geo_data = geo_registry.get(&self.geometry.clone().get_geo_namespace());

                let geo_data_new = geo_data.unwrap().element.clone();
                return Some(if geo_pal.palette.contains(&geo_data_new) {
                    geo_pal
                        .palette
                        .iter()
                        .position(|r| r.clone() == geo_data_new)
                        .unwrap()
                } else {
                    geo_pal.palette.push(geo_data_new.clone());
                    geo_pal.palette.len() - 1
                    // geo_pal
                    //     .palette
                    //     .iter()
                    //     .position(|r| r.clone() == geo_data_new)
                    //     .unwrap()
                });
            }
            // }
            // }
        }
        None
    }

    fn to_match_idx(&self, match_pal: Option<&mut BlockMatches>) -> usize {
        if let Some(match_pal) = match_pal {
            if match_pal.matches.contains(&self.identifier.to_string()) {
                match_pal
                    .matches
                    .iter()
                    .position(|r| r.clone().eq(&self.identifier.to_string()))
                    .unwrap()
            } else {
                match_pal.matches.push(self.identifier.to_string().clone());
                match_pal
                    .matches
                    .iter()
                    .position(|r| r.clone().eq(&self.identifier.to_string()))
                    .unwrap()
            }
        } else {
            0
        }
    }

    fn to_texture_idx(
        &self,
        _vox_registry: Option<&ClubeRegistry>,
        asset_registry: Option<&AssetRegistry>,
    ) -> Option<[usize; 6]> {
        if let Some(asset_registry) = asset_registry {
            return asset_registry
                .texture_indexes
                .get(&self.identifier.to_string())
                .copied();
        }
        None
    }

    fn blocking_sides(
        &self,
        _vox_registry: Option<&ClubeRegistry>,
        geo_registry: Option<&GeometryRegistry>,
    ) -> Option<([bool; 6], Option<[bool; 6]>)> {
        if let Some(geo_registry) = geo_registry {
            // if let Some(vox_registry) = vox_registry {
            // if let Some(block_data) = vox_registry.get(&self.identifier) {
            if let Some(geo_data) = geo_registry.get(&self.geometry.clone().get_geo_namespace()) {
                return Some((geo_data.blocks, geo_data.blocks_self));
            }
            // }
            // }
        }
        None
    }

    fn light_level() -> Option<u8> {
        None
    }

    fn to_visibility(
        &self,
        _vox_registry: Option<&ClubeRegistry>,
        _geo_registry: Option<&GeometryRegistry>,
    ) -> Option<VoxelVisibility> {
        Some(self.visibility)
    }
}
