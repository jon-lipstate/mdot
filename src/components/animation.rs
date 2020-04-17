use amethyst::ecs::{Component, DenseVecStorage, FlaggedStorage};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Animation {
    current_frame: usize,
    duration: f32,
    elapsed: f32,
    percent_complete: f32,
    sprite_data: Vec<(usize, f32)>,
}

impl Animation {
    pub fn new(duration: f32) -> Animation {
        Animation {
            current_frame: 0,
            duration,
            elapsed: 0.0,
            percent_complete: 0.0,
            sprite_data: Vec::new(),
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.elapsed += dt;
        self.percent_complete = self.elapsed / self.duration;
    }

    pub fn delete(&self) -> bool {
        if self.percent_complete >= 1.0 {
            return true;
        }
        false
    }
}

impl Component for Animation {
    type Storage = FlaggedStorage<Self, DenseVecStorage<Self>>;
}
