use core::fmt::Arguments;
use simple_cancellation_token::{error, SimpleCancellationTokenLog, SimpleCancellationTokenTool};
use yansi::Paint;

struct SimpleCancellationTokenLogger;

impl SimpleCancellationTokenLogger {
    fn new() -> SimpleCancellationTokenLogger {
        SimpleCancellationTokenLogger {}
    }
}

impl SimpleCancellationTokenLog for SimpleCancellationTokenLogger {
    fn output(self: &Self, args: Arguments) {
        println!("{}", args);
    }
    fn warning(self: &Self, args: Arguments) {
        eprintln!("{}", Paint::yellow(&format!("warning: {}", args)));
    }
    fn error(self: &Self, args: Arguments) {
        eprintln!("{}", Paint::red(&format!("error: {}", args)));
    }
}

fn main() {
    let logger = SimpleCancellationTokenLogger::new();

    if let Err(error) = SimpleCancellationTokenTool::new(&logger).run(std::env::args_os()) {
        error!(logger, "{}", error);
        std::process::exit(1);
    }
}
