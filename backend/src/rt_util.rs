// Libs
use std::io::{stdin, Read};

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
