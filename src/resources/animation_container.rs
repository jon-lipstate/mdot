// use amethyst::{
//     assets::{AssetStorage, Loader},
//     prelude::*,
//     renderer::{ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
// };

#[derive(Default)]
pub struct AnimationContainer {
    pub sequences: Vec<AnimationSequence>,
}
pub struct AnimationSequence {
    pub starting_tile_id: u32,
    pub frames: Vec<tiled::Frame>,
}

impl AnimationContainer {
    pub fn new() -> Self {
        Self {
            sequences: Vec::new(),
        }
    }
    pub fn add(&mut self, seq: AnimationSequence) {
        self.sequences.push(seq);
    }
    pub fn get(&self, starting_tile: u32) -> Option<Vec<tiled::Frame>> {
        let search_result = self
            .sequences
            .iter()
            .find(|s| s.starting_tile_id == starting_tile);
        match search_result {
            Some(seq) => return Some((*seq.frames).to_vec()),
            None => return None,
        }
    }
}
