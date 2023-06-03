use strum::{Display, EnumIter, EnumString};
use vinox_voxel::prelude::*;

#[derive(EnumString, Display, EnumIter)]
pub enum ClubeType {
    Dirt,
    Grass,
    Stone,
    Wood,
}

pub struct Clube {
    pub identifier: ClubeType,
    pub texture: usize,
    pub geometry: BlockGeometry,
    pub visibility: VoxelVisibility,
}

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
