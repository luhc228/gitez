use crate::{run_piped_command, GitUserConfig};
use anyhow::{Ok, Result};
use git2::Repository;
use std::{env, path::Path};

/// apply git config to current directory
pub fn apply(config_id: String) -> Result<()> {
  // check if it is a git repository
  if !is_git_repo(env::current_dir()?) {
    return Err(anyhow::anyhow!("The current directory is not a git repository"));
  }

  let git_user_configs = GitUserConfig::get_all()?;
  let git_user_config = git_user_configs.iter().find(|c| c.config_id == config_id);
  match git_user_config {
    Some(git_user_config) => {
      run_piped_command(vec!["git", "config", "user.name", git_user_config.name.as_str()])?;
      run_piped_command(vec!["git", "config", "user.email", git_user_config.email.as_str()])?;
      Ok(())
    }
    None => Err(anyhow::anyhow!("Config id: {} is not found", config_id)),
  }
}

fn is_git_repo<P: AsRef<Path>>(path: P) -> bool {
  Repository::open(path).is_ok()
}
