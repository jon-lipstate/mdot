use crate::defintions::Direction;
//use amethyst::core::EventChannel;
#[derive(Debug)]
pub enum RequestMovement {
    Move(Direction, usize),
}

//let mut channel = EventChannel::<RequestMovement>::new();
