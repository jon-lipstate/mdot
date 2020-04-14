// use amethyst::core::Transform;
// use amethyst::derive::SystemDesc;
// use amethyst::ecs::{Entities, Entity, Read, System, SystemData, Write, WriteStorage};
// use amethyst::renderer::SpriteRender;

// use crate::{
//     components::TilePosition,
//     resources::{Layers, Map, SpritesContainer},
//     states::PlayState,
// };

// #[derive(SystemDesc)]
// pub struct MapLoader;

// impl<'s> System<'s> for MapLoader {
//     type SystemData = (
//         WriteStorage<'s, Transform>,
//         WriteStorage<'s, SpriteRender>,
//         WriteStorage<'s, TilePosition>,
//         Write<'s, Map>,
//         Read<'s, SpritesContainer>,
//         Entities<'s>,
//         Read<'s, PlayState>,
//     );
//     fn run(
//         &mut self,
//         (mut transforms, mut sprite_renders, mut tiles_pos, mut map, container, entities, _state): Self::SystemData,
//     ) {
//         if map.needs_update {
//             //&& false {
//             log::warn!("map_loader.update_map()");
//             //log::warn!("State: {}", state.name);
//             // Delete old tiles
//             for entity in map.tile_ent.iter() {
//                 entities
//                     .delete(*entity)
//                     .expect("Failed to delete old map entities");
//             }
//             // Add new tiles
//             let mut ent_list: Vec<Entity> = Vec::new();
//             for (z, layer) in map.map.layers.iter().enumerate() {
//                 if z == Layers::L7 as usize {
//                     // log::warn!("L7 Break!");
//                     break;
//                 }; // Monster Layer, don't draw.
//                 for (x, row) in layer.tiles.iter().rev().enumerate() {
//                     for (y, col) in row.iter().enumerate() {
//                         if col.gid != 0 {
//                             let mut loc = TilePosition::new(y, x, z, col.gid as usize - 1);
//                             // log::warn!("{},{},{}", x, y, z);
//                             // log::warn!("{:?}", loc.gid);
//                             let transform = loc.to_trans();
//                             ent_list.push(
//                                 entities
//                                     .build_entity()
//                                     .with(container.sprites[loc.gid].clone(), &mut sprite_renders)
//                                     .with(transform, &mut transforms)
//                                     .with(loc, &mut tiles_pos)
//                                     .build(),
//                             );
//                         }
//                     }
//                 }
//             }
//             log::warn!("MAP LOADED");
//             map.needs_update = false;
//             map.tile_ent.append(&mut ent_list);
//         }
//     }
// }
