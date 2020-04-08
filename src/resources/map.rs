use std::{fs::File, io::BufReader, path::Path};

use amethyst::{
    core::transform::Transform,
    ecs::{Component, DenseVecStorage, Entity, FlaggedStorage},
};

#[derive(Clone, Debug, PartialEq)]
#[allow(dead_code)]
pub enum Layers {
    L1 = 0,
    L2,
    L3,
    L4,
    L5,
    L6,
    L7,
}

pub struct Map {
    pub name: String,
    pub map: tiled::Map,
    pub needs_update: bool, //TODO: Move to Gamestate ??
    pub tile_ent: Vec<Entity>,
}
impl Map {
    pub fn new(map_path: &str) -> Self {
        Self {
            map: get_map(map_path),
            name: map_path.to_string(),
            needs_update: true,
            tile_ent: Vec::new(),
        }
    }
    pub fn replace(&mut self, new_map_path: &str) {
        let map = get_map(new_map_path);
        self.map = map;
    }
}
impl Default for Map {
    fn default() -> Self {
        let map_path = "assets/maps/town.tmx";
        Map::new(map_path)
    }
}

fn get_map(map_path: &str) -> tiled::Map {
    let file = File::open(&Path::new(&map_path)).expect(map_path);
    let reader = BufReader::new(file);
    let tileset_path = &Path::new("assets/sprites/master16.tsx");
    return tiled::parse_with_path(reader, tileset_path).unwrap();
}
