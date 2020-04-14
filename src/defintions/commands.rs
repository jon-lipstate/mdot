use crate::defintions::Direction;
pub enum Commands {
    Move(Direction),
    Melee,
    GetLocation, //: /loc
}
