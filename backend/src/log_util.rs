// Libs
use colored::Colorize;

// Macros
/// Logs information message, in Blue color
#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        $crate::log_util::print_info(format!($($arg)*).as_str())
    }
}

/// Logs success message, in Green color
#[macro_export]
macro_rules! log_success {
    ($($arg:tt)*) => {
        $crate::log_util::print_success(format!($($arg)*).as_str())
    }
}

/// Logs warning message, in Yellow color
#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {
        $crate::log_util::print_warn(format!($($arg)*).as_str())
    }
}

/// Logs error/critical message, in Red color
#[macro_export]
macro_rules! log_err {
    ($($arg:tt)*) => {
        $crate::log_util::print_err(format!($($arg)*).as_str())
    }
}

/// Enables color support for Windows classic CLI interface, conhost.exe.
/// (Windows will not use the Terminal if the application is launched as administrator).
///
/// # Panics
///
/// The result always returns `Ok(())`, so this never actually panics.
#[cfg(target_os = "windows")]
pub fn enable_win32_conhost_support() {
    colored::control::set_virtual_terminal(true).unwrap();
}

pub fn print_info(msg: &str) {
    println!("{}", msg.bright_cyan());
}

pub fn print_success(msg: &str) {
    println!("{}", msg.bright_green());
}

pub fn print_warn(msg: &str) {
    println!("{}", msg.bright_yellow());
}

pub fn print_err(msg: &str) {
    eprintln!("{}", msg.bright_red());
}
