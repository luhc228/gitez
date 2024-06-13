use anyhow::Result;
use std::{
  env,
  io::{BufRead, BufReader},
  path::{Path, PathBuf},
  process::{Command, Stdio},
};

use crate::{get_shell_args, get_shell_name, GitUrl, GlobalConfig};

pub fn clone_repo(url: &str) -> Result<()> {
  let global_config = GlobalConfig::new()?;
  let git_url = GitUrl::new(url)?;
  let repo_path: PathBuf = get_repo_local_path(&git_url, &global_config)?;

  call_git_clone_command(url, &repo_path)?;

  Ok(())
}

fn get_repo_local_path(git_url: &GitUrl, global_config: &GlobalConfig) -> Result<PathBuf> {
  let repo_path: PathBuf;
  if let Some(base_dir) = &global_config.base_dir {
    repo_path = PathBuf::new()
      .join(base_dir)
      .join(&git_url.host)
      .join(&git_url.group)
      .join(&git_url.project);
  } else {
    repo_path = PathBuf::new().join(env::current_dir()?).join(&git_url.project);
  }
  Ok(repo_path)
}

fn call_git_clone_command(url: &str, repo_path: &Path) -> Result<()> {
  let shell_name = get_shell_name();
  let shell_args = get_shell_args(vec![
    "git".to_string(),
    "clone".to_string(),
    url.to_string(),
    repo_path.to_string_lossy().to_string(),
  ]);

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
