// Libs
use std::{
    fmt::Display,
    io::{stdin, Read},
    panic::{panic_any, set_hook as set_panic_hook},
};

// Enums
/// Defines an exit type to determine if the exit event is an unknown panic or intentional.
pub enum ExitType {
    Custom(String),
    Quit(),
}

// Structs
/// Halts the process on exit, when used in the main function.
///
/// An instance of `ExitHandler` should be placed at the very first line of the program to be effective.
pub struct ExitHandler;

impl Drop for ExitHandler {
    fn drop(&mut self) {
        log_warn!("Press Enter key to exit..");

        let _ = stdin().read(&mut [0u8]);
    }
}

// Traits
/// Implements custom unwrap functionality which quits the application when fails
pub trait QuitUnwrap<T> {
    /// Unwraps the value or quits the application with a custom message
    fn unwrap_or_quit(self, msg: &str) -> T;
}

// Implements `QuitUnwrap` for `Result`
impl<T, E: Display> QuitUnwrap<T> for Result<T, E> {
    fn unwrap_or_quit(self, msg: &str) -> T {
        match self {
            Ok(val) => val,
            Err(err) => quit(Some(format!("Error: {err}\n{msg}").as_str())),
        }
    }
}

// Implements `QuitUnwrap` for `Option`
impl<T> QuitUnwrap<T> for Option<T> {
    fn unwrap_or_quit(self, msg: &str) -> T {
        match self {
            Some(val) => val,
            None => quit(Some(msg)),
        }
    }
}

/// Prints a custom message on panics, depending on the payload and debug status.
pub fn set_custom_panic(debug: &'static bool) {
    set_panic_hook(Box::new(|e| {
        if let Some(s) = e.payload().downcast_ref::<ExitType>() {
            match s {
                ExitType::Custom(msg) => {
                    log_err!("{msg}. Exiting UFC Ripper.\n");
                }
                ExitType::Quit() => {} // TODO: Exit code needs to be `0` here
            }
        } else if *debug {
            log_err!(
                "An unknown error occurred:\n{:#?}\n\nExiting UFC Ripper.\n",
                e
            );
        } else {
            log_err!("An unknown error occurred. Exiting UFC Ripper.\n");
        }
    }));
}

/// Gracefully quits the application by properly unwinding, with a specific `ExitType`.
pub fn quit(message: Option<&str>) -> ! {
    if let Some(msg) = message {
        panic_any(ExitType::Custom(msg.to_string()))
    } else {
        panic_any(ExitType::Quit())
    }
}
