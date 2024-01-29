use crate::{
    event::{EventSystem, OnResizeEvent}, game::{Game, Initalize, OnResize, Render, Update}, log_info, platform::{pump_messages, PlatformState}
};
pub struct ApplicationConfig {
    pub application_name: &'static str,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
}
pub struct ApplicationState {
    pub event_system: EventSystem,

    pub (crate) internal_state: PlatformState,

    game: Game,
    is_running: bool,
    is_suspended: bool,
    witdh: u16,
    height: u16,
}

impl ApplicationState {
    pub fn new(
        application_config: ApplicationConfig,
        initalize: Initalize,
        update: Update,
        render: Render,
        on_resize: OnResize,
    ) -> ApplicationState {

        let internal_state = PlatformState::new(&application_config);

        let mut event_system = EventSystem::new();

        let game = Game {
            initalize,
            update,
            render,
            on_resize,
        };

        (game.on_resize)(&game, application_config.width, application_config.height);

        return ApplicationState {
            event_system,
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

            self.event_system.resize.fire(&OnResizeEvent {width: 0, height: 0});

            if !pump_messages() {
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