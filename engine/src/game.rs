pub type Initalize = fn(game: &Game) -> bool;
pub type Update = fn(game: &Game, delta_time: f32) -> bool;
pub type Render = fn(game: &Game, delta_time: f32) -> bool;
pub type OnResize = fn(game: &Game, width: u16, height: u16) -> bool;

pub struct Game {
    pub(crate)initalize: Initalize,
    pub(crate)update: Update,
    pub(crate)render: Render,
    pub(crate)on_resize: OnResize,
}