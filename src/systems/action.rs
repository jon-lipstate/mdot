use crate::{
    components::{Animation, InputComponent, Moving, Orientation, TilePosition},
    constants,
    defintions::{Direction, UserActions},
};
use amethyst::{
    core::Time,
    core::{math::Vector2, transform::Transform},
    ecs::*,
};

pub struct ActionSystem;
impl<'s> System<'s> for ActionSystem {
    type SystemData = (
        ReadStorage<'s, InputComponent>,
        WriteStorage<'s, Moving>,
        ReadStorage<'s, TilePosition>,
        WriteStorage<'s, Animation>,
        Read<'s, Time>,
        Entities<'s>,
    );

    fn run(
        &mut self,
        (input_components, mut movements, tile_positions, mut animations, time, entities): Self::SystemData,
    ) {
        let delta_seconds = time.delta_seconds();
        let frame = time.frame_number();
        let mut entities_to_remove: Vec<Entity> = Vec::new();
        for (entity, input, tile) in (&entities, &input_components, &tile_positions).join() {
            //log::info!("action sys");
            if input.action.is_some() {
                log::info!("action {:?}", input.action);
                match input.action.clone().unwrap() {
                    UserActions::Move(dir) => {
                        let mvmt = Moving {
                            start_tile_position: tile.position.clone(),
                            direction: dir.clone(),
                            value: 1,
                            time_remaining: constants::ACTION_DELAY_MS as f32 / 1000.,
                            duration: constants::ACTION_DELAY_MS as f32 / 1000.,
                        };
                        movements.insert(entity, mvmt).unwrap();
                        let walk_animation = match dir {
                            Direction::North => {
                                vec![(360, 0.), (361, 0.33), (362, 0.66), (360, 0.99)]
                            }
                            Direction::West => {
                                vec![(336, 0.), (337, 0.33), (338, 0.66), (336, 0.99)]
                            }
                            Direction::East => {
                                vec![(348, 0.), (349, 0.33), (350, 0.66), (348, 0.99)]
                            }
                            Direction::South => {
                                vec![(324, 0.), (325, 0.33), (326, 0.66), (324, 0.99)]
                            }
                        };
                        let anim = Animation::new(
                            constants::ACTION_DELAY_MS as f32 / 1000.,
                            walk_animation,
                        );
                        animations.insert(entity, anim).unwrap();
                    }
                    _ => (),
                }
            }
        }
    }
}
