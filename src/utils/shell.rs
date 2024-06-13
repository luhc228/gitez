pub fn get_shell_name() -> String {
  // windows
  if cfg!(windows) {
    "cmd".to_string()
  } else {
    // linux / macOS
    "sh".to_string()
  }
}

pub fn get_shell_args(args: Vec<String>) -> Vec<String> {
  // windows
  if cfg!(windows) {
    vec!["/c".to_string(), args.join(" ")]
  } else {
    // linux / macOS
    vec!["-c".to_string(), args.join(" ")]
  }
}
