#[cfg(target_os = "linux")]
#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        println!("\033[0;41m[ERROR] {}0;41\033[0m",format!($($arg)*));
    };
}

#[macro_export]
macro_rules! log_warning {
    ($($arg:tt)*) => {
        println!("[WARNING] {}",format!($($arg)*));
    };
}

#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        println!("[INFO] {}",format!($($arg)*));
    };
}

#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {
        println!("[DEBUG] {}",format!($($arg)*));
    };
}