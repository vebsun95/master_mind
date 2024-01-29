pub struct EventSystem {
    pub resize: ResizeEventRegistry,
    pub application_quit: ApplicationQuitEventRegistry,
}
impl EventSystem {
    pub fn new() -> Self {
        Self {
            resize: ResizeEventRegistry {registered: Vec::new()} ,
            application_quit: ApplicationQuitEventRegistry { registered: Vec::new()},
        }
    } 
}
type ApplicationQuitEventHandler = fn() -> ();
pub struct ApplicationQuitEventRegistry {
    registered: Vec<ApplicationQuitEventHandler>,
}
impl ApplicationQuitEventRegistry {
    pub fn register(self: &mut Self, callback: ApplicationQuitEventHandler) {
        self.registered.push(callback);
    }
    pub(crate) fn fire(self: &Self) {
        for cb in self.registered.iter() {
            cb(); 
        }
    }
}
type ResizeEventHandler = fn(event: &OnResizeEvent) -> ();
pub struct OnResizeEvent {
    pub width: u16,
    pub height: u16,
}
pub struct ResizeEventRegistry {
    registered: Vec<ResizeEventHandler>,
}
impl ResizeEventRegistry {
    pub fn register(self: &mut Self, callback: ResizeEventHandler) {
        self.registered.push(callback);
    }
    pub(crate) fn fire(self: &Self, on_resize_event: &OnResizeEvent) {
        for cb in self.registered.iter() {
            cb(on_resize_event); 
        }
    }
}