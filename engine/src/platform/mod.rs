mod platform_linux;
mod platform_windows;

#[cfg(target_os = "windows")]
pub type InternalState = self::platform_windows::InternalState;
#[cfg(target_os = "linux")]
pub type PlatformState = self::platform_linux::PlatformState;
