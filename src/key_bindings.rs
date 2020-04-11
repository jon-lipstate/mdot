use std::fmt::{self, Display};

use amethyst::input::BindingTypes;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum AxisBinding {
    Horizontal,
    Vertical,
}
#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum Direction {
    North,
    East,
    South,
    West,
}
#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum UserActions {
    Move(Direction),
    Melee,
    TypingMode,
    TypedData(String), //Indirect
}
//TODO: SWITCH TO MOVE(NORTH) PER BELOW:
// actions: {
//     UsePowerup(0): [[Key(E)]],
//     UsePowerup(1): [[Key(P)]],
//   },

impl Display for AxisBinding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl Display for UserActions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub struct UserInputBindingTypes;

impl BindingTypes for UserInputBindingTypes {
    type Axis = AxisBinding;
    type Action = UserActions;
}
