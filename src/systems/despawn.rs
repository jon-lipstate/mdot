use crate::{components::Despawn, constants};
use amethyst::{
    core::Time,
    core::{math::Vector2, transform::Transform},
    ecs::*,
    renderer::SpriteRender,
};

pub struct DespawnSystem;
impl<'s> System<'s> for DespawnSystem {
    type SystemData = (ReadStorage<'s, Despawn>, Read<'s, Time>, Entities<'s>);

    fn run(&mut self, (despawns, time, entities): Self::SystemData) {
        let current_time = time.absolute_time_seconds();
        for (despawn, entity) in (&despawns, &entities).join() {
            log::info!("ct: {:?}", current_time);
            if despawn.remove_at_time < current_time {
                match entities.delete(entity) {
                    Ok(_) => (),
                    Err(e) => panic!(e),
                }
            }
        }
    }
}
