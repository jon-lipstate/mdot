use crate::defintions::Direction;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum UserActions {
    Move(Direction),
    Melee,
    TypingMode,
    TypedData(String), //Indirect
    None,
}

impl Display for UserActions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
