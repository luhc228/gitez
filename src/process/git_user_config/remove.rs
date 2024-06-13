use crate::GitUserConfig;
use anyhow::{Ok, Result};
use console::style;
use dialoguer::Select;

pub fn remove(raw_config_name: Option<String>) -> Result<()> {
  let config_name: String = match raw_config_name {
    Some(config_name) => config_name,
    None => {
      let git_user_configs = GitUserConfig::get_all()?;
      let items = git_user_configs
        .iter()
        .map(|config| format!("{} ({}<{}>)", &config.config_name, &config.name, &config.email))
        .collect::<Vec<String>>();

      let selection = Select::new()
        .with_prompt("Chose the user config you want to remove")
        .items(&items)
        .interact()?;
      let selected_config_name = &git_user_configs[selection].config_name;

      selected_config_name.to_owned()
    }
  };

  GitUserConfig::remove(&config_name)?;

  println!("Git user config {} removed successfully!", style(config_name).bold());

  Ok(())
}
