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
    pub action: Option<UserActions>,
}
impl Default for InputComponent {
    fn default() -> Self {
        Self {
            action: Option::None,
        }
    }
}

impl InputComponent {
    pub fn clear_action(&mut self) {
        self.action = None::<UserActions>;
    }
}
