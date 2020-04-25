use crate::{
    components::{InputComponent, Moving, Orientation, PlayerComponent, TilePosition},
    resources::{AnimationContainer, AnimationSequence, SpritesContainer},
};
use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    controls::{CursorHideSystem, MouseFocusUpdateSystemDesc},
    core::{
        math::Vector3,
        timing::Time,
        transform::{Transform, TransformBundle},
        Parent,
    },
    ecs::prelude::{Entity, WorldExt},
    input::{
        get_key, is_close_requested, is_key_down, InputBundle, StringBindings, VirtualKeyCode,
    },
    prelude::*,
    renderer::{
        camera::Camera,
        light,
        palette::{LinSrgba, Srgb},
        plugins::{RenderShaded3D, RenderToWindow},
        rendy::mesh::{Normal, Position, Tangent, TexCoord},
        shape::Shape,
        types,
        visibility::BoundingSphere,
        ImageFormat, RenderingBundle, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture,
    },
    ui::{UiCreator, UiFinder, UiText},
    utils::{application_root_dir, fps_counter::FpsCounter},
    window::ScreenDimensions,
    Error,
};

#[derive(Default)]
pub struct PlayState {
    //sprite_sheet_handle: Option<Handle<SpriteSheet>>,
    //pub name: String,
    ui_root: Option<Entity>,
    fps_display: Option<Entity>,
    paused: bool,
}
// impl Default for PlayState {
//     // fn default() -> Self {
//     //     Self {
//     //         name: "hi there".to_string(),
//     //         ui_root = Some(world.exec(|mut creator: UiCreator<'_>| creator.create("config/play.ron", ()))),
//     //     }
//     // }
// }

impl SimpleState for PlayState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        world.register::<crate::components::Collidable>();
        let dimensions = (*world.read_resource::<ScreenDimensions>()).clone();
        init_camera(world, &dimensions);
        let m = crate::resources::Map::default();
        log::info!("{:?}", m.map.layers);
        let map: tiled::Map = m.map;

        let ssh = load_sprite_sheet(world);

        //Pull out property & animation data:
        let mut animation_container = AnimationContainer::new();
        let mut p_data: Vec<(u32, Vec<String>)> = Vec::new(); // may be an issue when multiple tilesets
        for tileset in map.tilesets {
            for tile in tileset.tiles {
                //Property Data
                let mut properties = Vec::new();
                for p in tile.properties {
                    match p.1 {
                        tiled::PropertyValue::BoolValue(val) => {
                            if val {
                                properties.push(p.0);
                            }
                        }
                        _ => (),
                    }
                }
                p_data.push((tile.id, properties));
                //Animation Data
                if let Some(animation) = tile.animation {
                    animation_container.add(AnimationSequence {
                        starting_tile_id: tile.id,
                        frames: animation,
                    });
                }

                // Tile {
                //     id: 0,
                //     images: [],
                //     properties: {
                //         "Collision": BoolValue(false)
                //         },
                //     objectgroup: None,
                //     animation: Some([
                //             Frame { tile_id: 91, duration: 450 },
                //             Frame { tile_id: 92, duration: 650 }
                //         ]),
                //     tile_type: None,
                //     probability: 1.0
                //     },
            }
        }
        //what components we want out of this:
        // tile_position, collision, sprite
        let mut layer_number = 0;
        for layer in map.layers {
            //every LayerTile is populated in the map, it has gid 0 = None
            for yr in 0..map.height as usize {
                for x in 0..map.width as usize {
                    let layer_tile = layer.tiles[yr][x];
                    if layer_tile.gid == 0 {
                        continue;
                    }
                    let tile_pos = TilePosition {
                        x,
                        y: map.height as usize - yr - 1,
                        z: layer_number,
                    };
                    let sprite_id = layer_tile.gid as usize - 1; //.tmx files are One-Indexed
                    log::info!("{:?},{:?}", sprite_id, tile_pos);
                    let sprite = get_tile(sprite_id, ssh.clone());
                    let mut tile = world
                        .create_entity()
                        .with(tile_pos.create_transform())
                        .with(tile_pos)
                        .with(sprite);
                    if let Some(pd) = p_data.iter().find(|p| p.0 == layer_tile.gid) {
                        for prop in &pd.1 {
                            match prop as &str {
                                "Collision" => {
                                    tile = tile.with(crate::components::Collidable::default());
                                }
                                "Interactable" => {
                                    log::info!("interactable component not implemented.")
                                }
                                _ => (),
                            }
                        }
                    }
                    let _tile_entity = tile.build();
                }
            }
            // Layer {
            //     name: "tree",
            //     opacity: 1.0,
            //     visible: true,
            //     tiles: [
            //        [
            //             LayerTile { gid: 46, flip_h: false, flip_v: false, flip_d: false },
            //             LayerTile { gid: 0, flip_h: false, flip_v: false, flip_d: false }
            //         ]
            //     ],
            //     properties: {},
            //     layer_index: 1
            // }
            layer_number += 1;
        }
        world.insert(animation_container);
        //tiled
        world.register::<crate::components::TilePosition>();
        world.insert(crate::resources::Map::default());
        let sprites = SpritesContainer::new(&world);
        world.insert(sprites);
        // let ssh = load_sprite_sheet(world);
        // //load_tile(world, ssh.clone());
        let player_sprite = get_tile(324, ssh.clone());
        let mut player_transform = Transform::default();
        player_transform.set_translation_z(0.1);
        //
        // self.ui_root =
        //     Some(world.exec(|mut creator: UiCreator<'_>| creator.create("ui/play.ron", ())));
        //player
        world
            .create_entity()
            //.with(Motion::default())
            .with(TilePosition::default())
            .with(Orientation::default())
            .with(PlayerComponent::default())
            .with(InputComponent::default())
            // .with(crate::components::CreatureTag::default())
            .with(player_transform)
            .with(player_sprite)
            .build();
    }
    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        if let Some(root_entity) = self.ui_root {
            data.world
                .delete_entity(root_entity)
                .expect("Failed to remove Game UI");
        }

        self.ui_root = None;
        self.fps_display = None;
    }
    // fn handle_event(
    //     &mut self,
    //     mut _data: StateData<'_, GameData<'_, '_>>,
    //     event: StateEvent,
    // ) -> SimpleTrans {
    //     if let StateEvent::Window(event) = &event {
    //         // Check if the window should be closed
    //         if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
    //             return Trans::Quit;
    //         }

    //         // Listen to any key events
    //         if let Some(event) = get_key(&event) {
    //             //log::info!("handling key event: {:?}", event);
    //         }
    //         // https://book.amethyst.rs/stable/pong-tutorial/pong-tutorial-03.html#capturing-user-input
    //     }

    //     // Keep going
    //     Trans::None
    // }
    fn update(&mut self, state_data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let StateData { world, .. } = state_data;
        //log::warn!("playState.update()");

        // this cannot happen in 'on_start', as the entity might not be fully
        // initialized/registered/created yet.
        if self.fps_display.is_none() {
            world.exec(|finder: UiFinder<'_>| {
                if let Some(entity) = finder.find("fps") {
                    self.fps_display = Some(entity);
                }
            });
        }

        // it is important that the 'paused' field is actually pausing your game.
        // Make sure to also pause your running systems.
        if !self.paused {
            let mut ui_text = world.write_storage::<UiText>();

            if let Some(fps_display) = self.fps_display.and_then(|entity| ui_text.get_mut(entity)) {
                if world.read_resource::<Time>().frame_number() % 5 == 0 {
                    let fps = world.read_resource::<FpsCounter>().sampled_fps();
                    fps_display.text = format!("FPS: {:.*}", 1, fps);
                }
            }
        }
        Trans::None
    }
}

fn init_camera(world: &mut World, dimensions: &ScreenDimensions) {
    // Center the camera in the middle of the screen, and let it cover
    // the entire screen
    let mut transform = Transform::default();
    transform.set_translation_xyz(dimensions.width() * 0.5, dimensions.height() * 0.5, 10.);

    world
        .create_entity()
        .with(Camera::standard_2d(dimensions.width(), dimensions.height()))
        .with(transform)
        .build();
}

fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "sprites/master16.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "sprites/master16.ron",            // Here we load the associated ron file
        SpriteSheetFormat(texture_handle), // We pass it the texture we want it to use
        (),
        &sprite_sheet_store,
    )
}
fn get_tile(tile: usize, sprite_sheet_handle: Handle<SpriteSheet>) -> SpriteRender {
    SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: tile,
    }
}
fn load_tile(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let mut local_transform = Transform::default();
    local_transform.set_translation_xyz(500.0, 500.0, 0.0);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 540,
    };

    world
        .create_entity()
        .with(sprite_render)
        .with(local_transform)
        .build();
}
