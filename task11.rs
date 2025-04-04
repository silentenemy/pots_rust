pub trait Logger {
    /// Помещает в лог сообщения заданного уровня.
    fn log(&self, verbosity: u8, message: &str);
}

struct StderrLogger;

impl Logger for StderrLogger {
    fn log(&self, verbosity: u8, message: &str) {
        eprintln!("verbosity={verbosity}: {message}");
    }
}

struct Filter {
    inner: StderrLogger,
    func: fn(u8, &str) -> bool,
}

impl Logger for Filter {
    fn log(&self, verbosity: u8, message: &str) {
        if (self.func)(verbosity, message) {
            self.inner.log(verbosity, message)
        }
    }
}

// TODO: Добавьте определение и реализацию Filter.

fn main() {
    let logger = Filter { inner: StderrLogger, func: |_verbosity, msg: &str| msg.contains("yikes")};
    logger.log(5, "FYI");
    logger.log(1, "yikes, something went wrong");
    logger.log(2, "uhoh");
}
