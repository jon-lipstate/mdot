use crate::constants;
use crate::key_bindings::Direction;
use crate::key_bindings::*;
use amethyst::{
    core::{timing::Time, transform::Transform},
    derive::SystemDesc,
    ecs::{Read, System, SystemData, Write},
    input::{InputHandler, StringBindings},
};
use std::time::Duration;

// struct ActionEntered {
//     Action: ActionBinding,
//     ExpiryTime: Duration,
// }
#[derive(SystemDesc)]
pub struct InputSystem {
    latched_actions: Vec<(ActionBinding, f64)>, //expiry time in f64
}

impl InputSystem {
    ///Iterates on latched_actions & removes any expired action
    fn remove_expired_latches(&mut self, current_time: f64) {
        self.latched_actions.retain(|(action, expiry_time)| {
            return current_time < *expiry_time;
        });
    }
    /// 1. Checks if the key is down
    /// 2. Checks that an action of the same type does not exist in latched_actions.
    /// 3. If new action is unique, pushes into latched_actions
    fn try_latch(&mut self, action: ActionBinding, expiry_time: f64) -> bool {
        //Check that the key is actually down:
        if !input.action_is_down(&action).unwrap() {
            return false;
        }
        let mut can_latch = self.latched_actions.iter().any(|a| match a {
            (action, _) => {
                return false;
            }
            _ => true,
        });
        //When vec is empty, the closure above will return false
        if self.latched_actions.len() == 0 {
            can_latch = true;
        }
        if can_latch {
            self.latched_actions.push((action, expiry_time));
        }
        return can_latch;
    }
}

impl Default for InputSystem {
    fn default() -> Self {
        Self {
            latched_actions: Vec::new(),
        }
    }
}

impl<'s> System<'s> for InputSystem {
    type SystemData = (Read<'s, InputHandler<GameBindingTypes>>, Read<'s, Time>); //, Write<'s, Input>);

    fn run(&mut self, (input, time): Self::SystemData) {
        let t = time.absolute_real_time().as_secs_f64();
        self.remove_expired_latches(t);
        //Movement Actions First:
        let delay = constants::MOVEMENT_DELAY_MS as f64 / 1000.;
        if self.try_latch(ActionBinding::Move(Direction::North), t + delay) {
            log::warn!("INSIDE MOVE(North), EMIT COMMAND");
        }
        if self.try_latch(ActionBinding::Move(Direction::East), t + delay) {
            log::warn!("INSIDE MOVE(East), EMIT COMMAND");
        }
    }
}
