use crate::{
    components::{Motion, Orientation, TilePosition},
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
        WriteStorage<'s, Motion>,
        WriteStorage<'s, TilePosition>,
        WriteStorage<'s, Orientation>,
        WriteStorage<'s, Transform>,
    );

    fn run(
        &mut self,
        (mut movements, mut tile_positions, mut orientations, mut transforms): Self::SystemData,
    ) {
        for (motion, tile_pos, orientation, transform) in (
            &mut movements,
            &mut tile_positions,
            &mut orientations,
            &mut transforms,
        )
            .join()
        {
            //Motion:
            let dir = &motion.direction;
            let val = motion.value;
            //Face the correct direction
            orientation.direction = motion.direction.clone();
            //Check if out of map bounds:
            //TODO!!
            //Move to Specified Tile:
            if motion.value > 0 {
                match motion.direction {
                    Direction::North => tile_pos.position.data[1] += 1,
                    Direction::South => tile_pos.position.data[1] -= 1,
                    Direction::East => tile_pos.position.data[0] += 1,
                    Direction::West => tile_pos.position.data[0] -= 1,
                }
            }

            // let magnitud = movement.velocity.magnitude();
            // if magnitude > movement.max_movement_speed {
            //     movement.velocity = movement.velocity * (movement.max_movement_speed / magnitude);
            // }
            // transform.prepend_translation_x(movement.velocity.x * delta_time);
            // transform.prepend_translation_y(movement.velocity.y * delta_time);
            // transform.prepend_translation_z(movement.velocity.z * delta_time);
        }
        for (movement, transform) in (&mut movements, &mut transforms).join() {
            // let angle = movement.velocity.y.atan2(movement.velocity.x);
            // transform.set_rotation_2d(angle);
        }
    }
}
