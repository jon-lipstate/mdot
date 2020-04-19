use crate::{
    components::{Animation, InputComponent, TilePosition},
    constants,
    defintions::{Direction, UserActions},
};
use amethyst::{
    core::Time,
    core::{math::Vector2, transform::Transform},
    ecs::*,
    renderer::SpriteRender,
};

pub struct AnimationSystem;
impl<'s> System<'s> for AnimationSystem {
    type SystemData = (
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, Animation>,
        Read<'s, Time>,
        Entities<'s>,
    );

    fn run(&mut self, (mut sprite_renders, mut animations, time, entities): Self::SystemData) {
        let dt = time.delta_seconds();
        let mut expired_animations: Vec<Entity> = Vec::new();
        for (sprite_render, entity, anim) in
            (&mut sprite_renders, &entities, &mut animations).join()
        {
            anim.elapsed += dt;
            if anim.elapsed > anim.duration {
                expired_animations.push(entity);
            }
            let pct_cmp = anim.elapsed / anim.duration;
            let mut index = 0;
            while anim.sprite_data.len() - 1 > index && anim.sprite_data[index].1 < pct_cmp {
                index += 1;
            }
            sprite_render.sprite_number = anim.sprite_data[index].0;
        }
        for e in expired_animations {
            animations.remove(e).unwrap();
        }
    }
}
