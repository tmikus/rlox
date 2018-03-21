pub struct RuntimeStatus {
  pub had_error: bool,
}

impl RuntimeStatus {
  pub fn new() -> RuntimeStatus {
    RuntimeStatus { had_error: false }
  }
}
