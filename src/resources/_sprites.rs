pub enum Character {
    Naked,
    Boy,
    BoyWithSword
    Girl,
    GirlWithSword
    Skeleton,
    Slime,
    Spider1,
    Spider2,
}
pub enum Animation {
    Walk,
    SwordAttack
    PunchAttack,
    None,
}

pub struct CharacterSprite {
    pub character: Character,
    

}

pub fn get_sprite_sequence(orientation: Direction, character: Character, animation: Animation) -> Vec<usize> {
    let sprite_ids = vec::new();
    let g_s = Vec::new() {708,709};
    //let g_w = 



    return sprite_ids;
}