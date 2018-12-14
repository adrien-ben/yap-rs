pub mod components; 
pub mod systems;
pub mod resources;

#[derive(Eq, PartialEq)]
pub enum State {
    Idle,
    Running,
}

impl Default for State {
    fn default() -> Self {
        State::Idle
    }
}