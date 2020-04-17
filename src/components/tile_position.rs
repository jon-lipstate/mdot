use crate::defintions::Direction;
use amethyst::{
    assets::{AssetPrefab, PrefabData},
    core::math::Vector2,
    derive::PrefabData,
    ecs::{Component, DenseVecStorage, Entity, NullStorage, WriteStorage},
    Error,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Component, Debug, Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
pub struct TilePosition {
    pub position: Vector2<isize>,
}
impl Default for TilePosition {
    fn default() -> Self {
        Self {
            position: Vector2::zeros(),
        }
    }
}
