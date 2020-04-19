use crate::{
    components::{InputComponent, Moving, Orientation, TilePosition},
    constants,
    defintions::Direction,
};
use amethyst::{
    core::Time,
    core::{math::Vector2, transform::Transform},
    ecs::*,
};

pub struct MotionSystem;
impl<'s> System<'s> for MotionSystem {
    type SystemData = (
        WriteStorage<'s, Moving>,
        WriteStorage<'s, TilePosition>,
        WriteStorage<'s, Orientation>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
        Entities<'s>,
    );

    fn run(
        &mut self,
        (
            mut movements,
            mut tile_positions,
            mut orientations,
            mut transforms,
            time,
            entities,
        ): Self::SystemData,
    ) {
        let delta_seconds = time.delta_seconds();
        let mut entities_to_remove: Vec<Entity> = Vec::new();
        for (motion, tile_pos, orientation, transform, entity) in (
            &mut movements,
            &mut tile_positions,
            &mut orientations,
            &mut transforms,
            &entities,
        )
            .join()
        {
            //Face the correct direction
            if orientation.direction != motion.direction {
                orientation.direction = motion.direction.clone();
                //REQUEST ROTATE ANIMATION - maybe do if value is zero? otherwise frame 1 can always turn player
            }
            //Check if out of map bounds:
            //TODO
            //Move to Specified Tile:
            // let start_x = tile_pos.position.data[0] as f32 * constants::PLAYER_MOVE;
            // let start_y = tile_pos.position.data[1] as f32 * constants::PLAYER_MOVE;
            if motion.time_remaining > 0. {
                let move_pixels = motion.value as f32 * constants::PLAYER_MOVE / 768.
                    * motion.duration
                    / delta_seconds;
                //TODO REFACTOR THIS TO LERP TO END STATE - ERROR PROPEGATION FROM START WILL ACCUMULATE
                match motion.direction {
                    Direction::North => {
                        transform.prepend_translation_y(move_pixels);
                    }
                    Direction::South => {
                        transform.prepend_translation_y(-move_pixels);
                    }
                    Direction::East => {
                        transform.prepend_translation_x(move_pixels);
                    }
                    Direction::West => {
                        transform.prepend_translation_x(-move_pixels);
                    }
                }
                if (motion.time_remaining - motion.duration).abs() < 1e-4 {
                    match motion.direction {
                        Direction::North => tile_pos.position.data[1] += motion.value as isize,
                        Direction::South => tile_pos.position.data[1] -= motion.value as isize,
                        Direction::East => tile_pos.position.data[0] += motion.value as isize,
                        Direction::West => tile_pos.position.data[0] -= motion.value as isize,
                    };
                    //log::info!("Moved to {:?}", tile_pos.position.data);
                }
                motion.time_remaining -= delta_seconds;
            } else {
                entities_to_remove.push(entity);
            }
        }
        for e in entities_to_remove {
            movements.remove(e);
        }
    }
}
