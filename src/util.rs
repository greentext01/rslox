use std::io::Write;

pub fn flush_stdout() {
    std::io::stdout().flush().expect("Couldn't flush stdout.");
}
