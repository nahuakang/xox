pub fn error(line: i32, message: &str) {
    report(line, "", message);
}

pub fn report(line: i32, where_: &str, message: &str) {
    eprintln!("[line {}] Error{}: {}", line, where_, message);
    // had_error = true;
    // TODO: had_error isn't implemented yet. Also, use custom Error type.
}
