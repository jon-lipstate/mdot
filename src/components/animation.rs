use amethyst::ecs::{Component, DenseVecStorage, FlaggedStorage};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Animation {
    pub duration: f32,
    pub elapsed: f32,
    pub sprite_data: Vec<(usize, f32)>,
}

impl Animation {
    pub fn new(duration: f32, sprites: Vec<(usize, f32)>) -> Animation {
        Animation {
            duration,
            elapsed: 0.0,
            sprite_data: sprites,
        }
    }
}

impl Component for Animation {
    type Storage = FlaggedStorage<Self, DenseVecStorage<Self>>;
}
