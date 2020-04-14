use crate::defintions::{Direction, UserActions};
use amethyst::input::BindingTypes;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum AxisBinding {
    Horizontal,
    Vertical,
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

#[derive(Debug)]
pub struct UserInputBindingTypes;

impl BindingTypes for UserInputBindingTypes {
    type Axis = AxisBinding;
    type Action = UserActions;
}
