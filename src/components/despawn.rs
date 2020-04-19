use amethyst::ecs::{Component, DenseVecStorage, FlaggedStorage};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Despawn {
    pub remove_at_time: f64,
}

impl Despawn {
    pub fn new(remove_at_time: f64) -> Despawn {
        Despawn { remove_at_time }
    }
}

impl Component for Despawn {
    type Storage = FlaggedStorage<Self, DenseVecStorage<Self>>;
}
