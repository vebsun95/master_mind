mod platform_linux;
mod platform_windows;

#[cfg(target_os = "windows")]
pub type PlatformState = self::platform_windows::PlateformState;
#[cfg(target_os = "linux")]
pub type PlatformState = self::platform_linux::PlatformState;
