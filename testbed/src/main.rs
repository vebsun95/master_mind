use engine::{application::{ApplicationConfig, ApplicationState}, game::Game};

fn main() {
    let application_config = ApplicationConfig {
        application_name: "master_mind",
        x: 0,
        y: 0,
        width: 600,
        height: 800,
    };

    let mut application = ApplicationState::create(application_config, initalize, update, render, on_resize);

    application.run();

}

pub fn initalize(game: &Game) -> bool {
    println!("Game initalize was called.");
    return true;
}
pub fn update(game: &Game, delta_time: f32) -> bool {
    println!("Game update was called.");
    return true;
}
pub fn render(game: &Game, delta_time: f32) -> bool {
    println!("Game render was called.");
    return  true;
}
pub fn on_resize(game: &Game, width: u16, height: u16) -> bool {
    println!("Game on resize was called.");
    return true;
}