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
pub struct Orientation {
    pub direction: Direction,
}
impl Default for Orientation {
    fn default() -> Self {
        Self {
            direction: Direction::South,
        }
    }
}
