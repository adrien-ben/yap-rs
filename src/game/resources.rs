use piston::input::{RenderArgs, Button};
use super::State;

#[derive(Default)]
pub struct GameState(pub State);

pub struct GameArea {
    pub width: f64,
    pub height: f64,
}

#[derive(Default)]
pub struct DeltaTime(pub f64);

#[derive(Default)]
pub struct RenderEvent(pub Option<RenderArgs>);

#[derive(Default)]
pub struct PressEvent(pub Option<Button>);

#[derive(Default)]
pub struct ReleaseEvent(pub Option<Button>);
