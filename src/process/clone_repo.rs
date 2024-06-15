use crate::{run_piped_command, GitUrl, GlobalConfig};
use anyhow::Result;
use console::style;
use std::{
  env,
  path::{Path, PathBuf},
};

pub fn clone_repo(url: &str) -> Result<()> {
  let global_config = GlobalConfig::new()?;
  let git_url = GitUrl::new(url)?;
  let repo_path: PathBuf = get_repo_local_path(&git_url, &global_config)?;

  call_git_clone_command(url, &repo_path)?;
  println!("\n🎉 Cloned repository Successfully!\n");
  println!(
    "You can enter the repository by running the command:\n\n{}",
    style(format!("cd {}", repo_path.to_string_lossy())).bold().green()
  );

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
  run_piped_command(vec!["git", "clone", url, repo_path.to_str().unwrap()])?;
  Ok(())
}
