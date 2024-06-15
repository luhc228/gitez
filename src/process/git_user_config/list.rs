use crate::GitUserConfig;
use anyhow::{Ok, Result};
use console::style;

pub fn list() -> Result<()> {
  let git_user_configs = GitUserConfig::get_all()?;
  git_user_configs.iter().enumerate().for_each(|(index, config)| {
    println!(
      "{}. {}: {}<{}>",
      index + 1,
      style(&config.config_id).bold(),
      &config.name,
      &config.email
    );
  });

  Ok(())
}
