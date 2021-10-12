
use bracket_lib::prelude::*;
use bracket_state_machine::{DeltaTime, State, StateMachine, Transition, stopwatch::Stopwatch}; 
use std::time::Duration;

struct TitleState(Stopwatch);
struct PlayingState(u32, u8);
struct PausedState;

impl TitleState {
    fn new() -> Self {
        TitleState(Stopwatch::new(Duration::new(3, 0)))
    }
}

impl State for TitleState {
    fn update(&mut self, dt: DeltaTime) -> Option<Transition> {
        if self.0.is_over() {
            Some(Transition::Push(Box::new(PlayingState(0, 0))))
        } else {
            self.0.tick(dt);
            None
        }
    }

    fn draw(&self, term: &mut BTerm) {
        term.cls_bg(RED);
        term.print_color_centered(20, BLUE, RED, "This is the title screen");
        term.print_color_centered(25, BLUE, RED, "(1)");
    }

    fn handle_input(&mut self, input: VirtualKeyCode) -> Option<bracket_state_machine::Transition> {
        None
    }
}

impl State for PlayingState {
    fn update(&mut self, dt: DeltaTime) -> Option<Transition> {
        self.1 = (self.1 + 1) % 3;
        if self.1 == 0 {
            self.0 = (self.0 + 1) % 50;
        }

        None
    }

    fn handle_input(&mut self, input: VirtualKeyCode) -> Option<Transition> {
        match input {
            VirtualKeyCode::Space => Some(Transition::Push(Box::new(PausedState))),
            VirtualKeyCode::Escape => Some(Transition::Clear),
            _ => None,
        }
    }

    fn draw(&self, term: &mut BTerm) {
        let h = term.height_pixels;
        term.print_color_centered((self.0 + h - 1) % h, GREEN, BLACK, "               ");
        term.print_color_centered(self.0, GREEN, BLACK, " PLAYING STATE ");
        term.print_color_centered((self.0 + 1) % h, GREEN, BLACK, "               ");
    }

    fn is_transparent(&self) -> bool {
        false
    }
}

impl State for PausedState {
    fn handle_input(&mut self, input: VirtualKeyCode) -> Option<Transition> {
        if input == VirtualKeyCode::Space {
            return Some(Transition::Pop)
        }
        None
    }

    fn update(&mut self, dt: DeltaTime) -> Option<Transition> {
        None
    }

    fn draw(&self, term: &mut BTerm) {
        term.print_centered(0, "PAUSED");
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("State Machine Sample")
        .with_fps_cap(24.0)
        .build()
        .expect("failed to build a BTerm");

    let state_machine = StateMachine::new(TitleState::new());
    main_loop(context, state_machine)
}