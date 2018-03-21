use runtime_status::RuntimeStatus;

pub fn error(runtime_status: &mut RuntimeStatus, line: i32, message: &str) {
  report(runtime_status, line, "", message);
}

fn report(runtime_status: &mut RuntimeStatus, line: i32, location: &str, message: &str) {
  eprintln!("[line {}] Error{}: {}", line, location, message);
  runtime_status.had_error = true;
}