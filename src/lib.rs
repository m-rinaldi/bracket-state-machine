#![allow(unused)]

use bracket_lib::prelude::*;
use std::time::Duration;

#[derive(Copy, Clone)]
pub struct DeltaTime(Duration);

impl DeltaTime {
    pub fn from_millis(millis: f32) -> Self {
        // TODO
        DeltaTime(Duration::from_millis(millis as u64))
    }
}

pub enum Transition {
    Switch(Box<dyn State>),
    Push(Box<dyn State>),
    Pop,
    Reset,
    // TODO Sequence(Vec<Transition>)
}


pub trait State {
    #[must_use]
    fn handle_input(&mut self, input: VirtualKeyCode) -> Option<Transition>;

    #[must_use]
    fn update(&mut self, dt: DeltaTime) -> Option<Transition>;

    fn draw(&self, term: &mut BTerm);
}

pub struct StateMachine {
    states: Vec<Box<dyn State>>,
    pending_ops: Vec<Transition>,
}

impl StateMachine {
    // TODO implement From<State>
    /// creates a state machine with an initial state
    pub fn new<T: State + 'static>(init_state: T) -> Self {
        StateMachine {
            states: vec![Box::new(init_state)],
            pending_ops: vec![],
        }
    }

    pub fn is_empty(&self) -> bool {
        self.states.is_empty()
    }

    pub fn handle_input(&mut self, input: VirtualKeyCode) {
        if let Some(state) = self.states.last_mut() {
            if let Some(trans) = state.handle_input(input) {
                self.pending_ops.push(trans);
            }
        }
    }

    // TODO fn switch()

    pub fn update(&mut self, dt: DeltaTime) {
        if let Some(top_state) = self.states.last_mut() {
            let trans = top_state.update(dt);
            if let Some(trans) = trans {
                self.pending_ops.push(trans);
            }
        }
    }

    pub fn draw(&self, mut screen: &mut BTerm) {
        for state in self.states.iter() {
            state.draw(screen);
        }
    }

    // TODO write these individual operations

    fn apply_pending_trans(&mut self) {
        let pending_ops = self.pending_ops.drain(..);
        for pending_op in pending_ops {
            match pending_op {
                Transition::Switch(new_state) => {
                    self.states.pop();
                    self.states.push(new_state);
                }
                Transition::Push(new_state) => {
                    self.states.push(new_state)
                }
                Transition::Pop => {
                    self.states.pop();
                }
                Transition::Reset => {
                    self.states.clear();
                }
            };
        }
    }
}

impl GameState for StateMachine {
    fn tick(&mut self, ctx: &mut BTerm) {
        if (self.is_empty()) {
            ctx.quit();
        }

        if let Some(key) = ctx.key {
            self.handle_input(key);
        }

        // TODO time
        self.update(DeltaTime::from_millis(ctx.frame_time_ms));

        ctx.cls();
        self.draw(ctx);

        // TODO perform pending operations
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
