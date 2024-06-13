use anyhow::Ok;
use console::style;
use dialoguer::Select;

use crate::GitUserConfig;

pub fn get_git_user_config_id(raw_config_id: Option<String>) -> anyhow::Result<String> {
  let config_id: String = match raw_config_id {
    Some(config_id) => config_id,
    None => {
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

      let selection = Select::new()
        .with_prompt("Chose the user config you want to remove")
        .items(&items)
        .interact()?;
      let selected_config_id = &git_user_configs[selection].config_id;

      selected_config_id.to_owned()
    }
  };

  Ok(config_id)
}