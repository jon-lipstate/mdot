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
pub struct Motion {
    pub direction: Direction,
    pub value: usize,
}
impl Default for Motion {
    fn default() -> Self {
        Self {
            direction: Direction::South,
            value: 0,
        }
    }
}
