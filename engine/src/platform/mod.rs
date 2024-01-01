mod platform_linux;
mod platform_windows;

#[cfg(target_os = "windows")]
type InternalState = self::platform_windows::InternalState;
#[cfg(target_os = "linux")]
type InternalState = self::platform_linux::InternalState;
pub struct ApplicationState{
    internal_state: InternalState,
}

trait ApplicationStateTraits {
    fn new(x: i16, y: i16, width: u16, height: u16, application_name: &String) -> ApplicationState;
}
