use crate::GitUserConfig;
use anyhow::{Ok, Result};
use console::style;
use dialoguer::Select;

pub fn select_git_user_config() -> Result<GitUserConfig> {
  let git_user_configs = GitUserConfig::get_all()?;
  let items = git_user_configs
    .iter()
    .map(|config| {
      format!(
        "{}: {}<{}>",
        style(&config.config_id).bold(),
        &config.name,
        &config.email
      )
    })
    .collect::<Vec<String>>();

  if items.is_empty() {
    return Err(anyhow::anyhow!(
      "No git user config found. Please add one by running `gitez user-config add`"
    ));
  }

  let selection = Select::new()
    .with_prompt("choose one of your git user config")
    .items(&items)
    .default(0)
    .interact()?;
  Ok(git_user_configs[selection].to_owned())
}
