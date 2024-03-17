// Libs
use std::{
    io::{stdin, Read},
    panic::set_hook as set_panic_hook,
};

// Enums
/// Defines an exit type to determine if the exit event is an unknown panic or intentional.
pub enum ExitType {
    Custom(String),
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

// Prints a custom message on panics, depending on the payload and debug status.
pub fn set_custom_panic(debug: &'static bool) {
    set_panic_hook(Box::new(|e| {
        if let Some(s) = e.payload().downcast_ref::<ExitType>() {
            match s {
                ExitType::Custom(msg) => {
                    log_err!("{msg}. Exiting UFC Ripper.\n");
                }
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
