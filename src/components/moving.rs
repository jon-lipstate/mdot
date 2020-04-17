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
pub struct Moving {
    pub start_tile_position: Vector2<isize>,
    pub direction: Direction,
    pub value: isize,
    pub time_remaining: f32,
    pub duration: f32,
}
impl Default for Moving {
    fn default() -> Self {
        Self {
            start_tile_position: Vector2::zeros(),
            direction: Direction::South,
            value: 0,
            time_remaining: 0.0,
            duration: 0.0,
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
