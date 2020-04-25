use crate::{components::TilePosition, defintions::Direction};
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
pub struct Moving {
    pub start_tile_position: TilePosition,
    pub direction: Direction,
    pub value: isize,
    pub time_remaining: f32,
    pub duration: f32,
}
impl Moving {
    pub fn new(start: TilePosition, dir: Direction, val: isize, duration: f32) -> Self {
        Self {
            start_tile_position: start,
            direction: dir,
            value: val,
            time_remaining: duration,
            duration,
        }
    }
}
// impl Moving {
//     ///Checks the orientation that is present during the movement
//     pub fn moving_orientation(&self) -> Direction {
//         let dx = self.end_tile_position.data[0] - self.start_tile_position.data[0];
//         let dy = self.end_tile_position.data[1] - self.start_tile_position.data[1];

//         if dx != 0 {
//             if dx > 0 {
//                 return Direction::East;
//             } else if dx < 0 {
//                 return Direction::West;
//             } else {
//                 panic!("unreachable");
//             }
//         } else if dy != 0 {
//             if dy > 0 {
//                 return Direction::North;
//             } else if dy < 0 {
//                 return Direction::South;
//             } else {
//                 panic!("unreachable");
//             }
//         } else {
//             panic!("Moving dx and dy simultaneously is not permitted.");
//         }
//     }
// }
