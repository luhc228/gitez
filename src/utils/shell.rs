use anyhow::Result;
use std::{
  io::{BufRead, BufReader},
  process::{Command, Stdio},
};

fn get_shell_name() -> String {
  // windows
  if cfg!(windows) {
    "cmd".to_string()
  } else {
    // linux / macOS
    "sh".to_string()
  }
}

fn get_shell_args(args: Vec<&str>) -> Vec<String> {
  // windows
  if cfg!(windows) {
    ["/c", &args.join(" ")].iter().map(|s| s.to_string()).collect()
  } else {
    // linux / macOS
    ["-c", &args.join(" ")].iter().map(|s| s.to_string()).collect()
  }
}

pub fn run_piped_command(args: Vec<&str>) -> Result<()> {
  let shell_name = get_shell_name();
  let shell_args = get_shell_args(args);

  let mut child = Command::new(shell_name)
    .args(&shell_args)
    .stdout(Stdio::piped())
    .spawn()?;

  if let Some(ref mut stdout) = child.stdout {
    let reader = BufReader::new(stdout);
    for line in reader.lines() {
      println!("{}", line?.as_str());
    }
  } else if let Some(ref mut stderr) = child.stderr {
    let reader = BufReader::new(stderr);
    let error = reader.lines().collect::<Result<Vec<_>, _>>()?.join("\n");
    return Err(anyhow::anyhow!(error));
  }

  Ok(())
}
