// Macros
/// Logs information message, in Blue color
#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        println!("{}", $crate::Colorize::bright_cyan(
            format!($($arg)*).as_str()
        ))
    };
}

/// Logs success message, in Green color
#[macro_export]
macro_rules! log_success {
    ($($arg:tt)*) => {
        println!("{}", $crate::Colorize::bright_green(
            format!($($arg)*).as_str()
        ))
    };
}

/// Logs warning message, in Yellow color
#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {
        println!("{}", $crate::Colorize::bright_yellow(
            format!($($arg)*).as_str()
        ))
    };
}

/// Logs error/critical message, in Red color
#[macro_export]
macro_rules! log_err {
    ($($arg:tt)*) => {
        eprintln!("{}", $crate::Colorize::bright_red(
            format!($($arg)*).as_str()
        ))
    };
}

/// Enables color support for Windows classic CLI interface, conhost.exe.
/// (Windows will not use the Terminal if the application is launched as administrator).
#[cfg(target_os = "windows")]
pub fn enable_win32_conhost_support() {
    colored::control::set_virtual_terminal(true).ok();
}
