use crate::constants;
use amethyst::{
    core::Transform,
    ecs::{Component, DenseVecStorage, FlaggedStorage},
};
impl Component for TilePosition {
    type Storage = FlaggedStorage<Self, DenseVecStorage<Self>>;
}

pub struct TilePosition {
    pub x: usize,
    pub y: usize,
    pub z: usize,
    pub gid: usize,
}

impl TilePosition {
    pub fn new(x: usize, y: usize, z: usize, gid: usize) -> Self {
        Self { x, y, z, gid }
    }
    pub fn get_transform(&mut self) -> Transform {
        let mut transform = Transform::default();
        transform.set_translation_xyz(
            (self.x as f32 * 16.) as f32 + 8.0,
            (self.y as f32 * 16.) as f32 + 8.0,
            self.z as f32 * 0.1,
        );
        transform
    }
    pub fn to_trans(&mut self) -> Transform {
        let mut transform = Transform::default();
        transform.set_translation_xyz(
            (self.x as f32 * constants::TILE_SIZE) as f32 + 8.0,
            (self.y as f32 * constants::TILE_SIZE) as f32 + 8.0,
            self.z as f32 * 0.1,
        );
        transform
    }
}
