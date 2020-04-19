use crate::{
    components::{Animation, Despawn, InputComponent, Moving, Orientation, TilePosition},
    constants,
    defintions::{Direction, UserActions},
    resources::SpritesContainer,
};
use amethyst::{
    core::math::Vector2,
    core::{Parent, Time, Transform},
    ecs::*,
    renderer::SpriteRender,
};

pub struct ActionSystem;
impl<'s> System<'s> for ActionSystem {
    type SystemData = (
        ReadStorage<'s, InputComponent>,
        WriteStorage<'s, Moving>,
        ReadStorage<'s, Orientation>,
        ReadStorage<'s, TilePosition>,
        WriteStorage<'s, Animation>,
        WriteStorage<'s, Despawn>,
        Read<'s, Time>,
        Entities<'s>,
        Read<'s, SpritesContainer>,
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Parent>,
    );

    fn run(
        &mut self,
        (
            input_components,
            mut movements,
            orientations,
            tile_positions,
            mut animations,
            mut despawns,
            time,
            entities,
            sprites_container,
            mut sprite_renders,
            mut transforms,
            mut parents,
        ): Self::SystemData,
    ) {
        let delta_seconds = time.delta_seconds();
        let frame = time.frame_number();
        let mut entities_to_remove: Vec<Entity> = Vec::new();
        for (entity, input, tile, orientation) in
            (&entities, &input_components, &tile_positions, &orientations).join()
        {
            //log::info!("action sys");
            if input.action.is_some() {
                log::info!("action {:?}", input.action);
                match input.action.clone().unwrap() {
                    UserActions::Melee => {
                        let xf = match orientation.direction {
                            Direction::North => {
                                Transform::default().move_up(constants::TILE_SIZE).clone()
                            }
                            Direction::South => {
                                Transform::default().move_down(constants::TILE_SIZE).clone()
                            }
                            Direction::East => Transform::default()
                                .move_right(constants::TILE_SIZE)
                                .clone(),
                            Direction::West => {
                                Transform::default().move_left(constants::TILE_SIZE).clone()
                            }
                        };
                        entities
                            .build_entity()
                            .with(sprites_container.sprites[602].clone(), &mut sprite_renders) // hard coded to heart
                            .with(xf.clone(), &mut transforms)
                            .with(Parent::new(entity), &mut parents)
                            .with(
                                Despawn::new(
                                    time.absolute_time_seconds()
                                        + constants::ACTION_DELAY_MS as f64 / 1000.,
                                ),
                                &mut despawns,
                            )
                            .build();
                    }
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
