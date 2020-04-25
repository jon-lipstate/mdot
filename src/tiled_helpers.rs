extern crate tiled;

pub fn has_collision(tile_props: &Option<tiled::Properties>) -> bool {
    match tile_props {
        None => return false,
        Some(i) => match i.get("Collision") {
            Some(value) => match value {
                tiled::PropertyValue::BoolValue(val) => return *val,
                _ => log::info!("{:?}", value),
            },
            None => return false,
        },
    }
    return true;
}
