use std::{fs, path::PathBuf};

use anyhow::{Ok, Result};
use dialoguer::Input;
use ini::Ini;

use crate::{run_piped_command, select_git_user_config, GitUserConfig, GlobalDir};

const GIT_CONFIG_DIR: &str = "git-configs";

pub fn include() -> Result<()> {
  // input the folder path
  let folder_path: String = Input::new().with_prompt("Enter the folder path").interact()?;
  // Select the git user config
  let git_user_config = select_git_user_config()?;

  let git_config_file = create_git_config_file(&git_user_config)?;

  add_include_git_config_to_global(&folder_path, git_config_file)?;

  println!("Git user config included successfully!");

  Ok(())
}

fn create_git_config_file(git_user_config: &GitUserConfig) -> Result<PathBuf> {
  let global_dir = GlobalDir::new();

  let git_config_path = global_dir
    .0
    .join(GIT_CONFIG_DIR)
    .join(format!("{}-config", git_user_config.config_id));
  fs::create_dir_all(git_config_path.parent().expect("failed to get the parent path"))?;
  let mut config = Ini::new();
  config
    .with_section(Some("user"))
    .set("name", &git_user_config.name)
    .set("email", &git_user_config.email);
  config.write_to_file(&git_config_path)?;
  Ok(git_config_path)
}

fn add_include_git_config_to_global(folder_path: &str, git_config_path: PathBuf) -> Result<()> {
  let config_key = format!("includeIf.\"gitdir:{}\".path", git_config_path.display());
  run_piped_command(vec!["git", "config", "--global", &config_key, folder_path])?;
  Ok(())
}
