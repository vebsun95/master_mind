use engine::{application::{ApplicationConfig, ApplicationState}, game::Game, log_info};

fn main() {
    let application_config = ApplicationConfig {
        application_name: "master_mind",
        x: 0,
        y: 0,
        width: 600,
        height: 800,
    };

    let mut application = ApplicationState::new(application_config, initalize, update, render, on_resize);

    application.run();

}

pub fn initalize(game: &Game) -> bool {
    log_info!("Initalize was called!");
    return true;
}
pub fn update(game: &Game, delta_time: f32) -> bool {
    log_info!("Game update was called.");
    return true;
}
pub fn render(game: &Game, delta_time: f32) -> bool {
    log_info!("Game render was called.");
    return  true;
}
pub fn on_resize(game: &Game, width: u16, height: u16) -> bool {
    log_info!("Game on resize was called.");
    return true;
}