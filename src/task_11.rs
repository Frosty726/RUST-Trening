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

// TODO: Добавьте определение и реализацию Filter.
type checker = fn(u8, &str) -> bool;

struct Filter {
    logger : StderrLogger,
    check : checker,
}

impl Filter {
    fn new(logger : StderrLogger, check : checker) -> Self {
        Filter { logger, check }
    }
}

impl Logger for Filter {
    fn log(&self, verbosity: u8, message: &str) {
        if (self.check)(verbosity, message) {
            self.logger.log(verbosity, message);
        }
    }
}

pub fn demonstrate() {
    println!("--- Task 11 ---");
    let logger = Filter::new(StderrLogger, |_verbosity, msg| msg.contains("yikes"));
    logger.log(5, "FYI");
    logger.log(1, "yikes, something went wrong");
    logger.log(2, "uhoh");
    println!();
}