use crate::defintions::{Direction, UserActions};
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
pub struct InputComponent {
    pub action: UserActions,
}
impl Default for InputComponent {
    fn default() -> Self {
        Self {
            action: UserActions::None,
        }
    }
}
