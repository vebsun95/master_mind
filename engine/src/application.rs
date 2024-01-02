use crate::{platform::PlatformState, game::{Game, Initalize, OnResize, Update, Render}};

static mut INITALIZED: bool = false;
pub struct ApplicationConfig {
    pub application_name: &'static str,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
}
pub struct ApplicationState {
    game: Game,
    is_running: bool,
    is_suspended: bool,
    witdh: u16,
    height: u16,
    internal_state: PlatformState,
}

impl ApplicationState {
    pub fn create(application_config: ApplicationConfig, initalize: Initalize, update: Update, render: Render, on_resize: OnResize) -> ApplicationState {
        if unsafe { INITALIZED } {
            panic!("ApplicationState::crate called more than once.");
        }

        let internal_state = PlatformState::new(
            application_config.x,
            application_config.y,
            application_config.width,
            application_config.height,
            application_config.application_name,
        );

        let game = Game {
            initalize,
            update,
            render,
            on_resize,
        };
        
        (game.on_resize)(&game, application_config.width, application_config.height);

        unsafe { INITALIZED = true; }

        return  ApplicationState {
            game,
            is_running: true,
            is_suspended: false,
            witdh: application_config.width,
            height: application_config.height,
            internal_state,
        };
    }
    pub fn run(self: &mut ApplicationState) {

        if !(self.game.initalize)(&self.game) {
            panic!("Failed to initalize game.");
        }

        while self.is_running {

            if self.internal_state.pump_messages() {
                self.is_running = false;
            }

            if !self.is_suspended {

                if !(self.game.update)(&self.game, 0.0) {
                    self.is_running = false;
                }

                if !(self.game.render)(&self.game, 0.0) {
                    self.is_running = false;
                }
            }
        }
    }
}
