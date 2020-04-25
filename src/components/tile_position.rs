use crate::{constants, defintions::Direction};
use amethyst::{
    assets::{AssetPrefab, PrefabData},
    core::transform::Transform,
    derive::PrefabData,
    ecs::{Component, DenseVecStorage, Entity, NullStorage, WriteStorage},
    Error,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Component, Default, Debug, Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
pub struct TilePosition {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}
impl TilePosition {
    pub fn create_transform(&self) -> Transform {
        let mut transform = Transform::default();
        transform.set_translation_xyz(
            (self.x as f32 * constants::TILE_SIZE) as f32 + 8.0,
            (self.y as f32 * constants::TILE_SIZE) as f32 + 8.0,
            self.z as f32 * 0.1,
        );
        transform
    }
}
