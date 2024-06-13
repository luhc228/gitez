use crate::{GitUserConfig, GlobalConfig};
use anyhow::{Ok, Result};
use console::style;
use dialoguer::Input;
use regex::Regex;

pub fn add() -> Result<()> {
  let config_id: String = Input::new()
    .with_prompt("Enter the id of your user config. e.g.: github, gitlab, etc.")
    .validate_with(|input: &String| -> Result<()> {
      if input.is_empty() {
        return Err(anyhow::anyhow!("The user config id cannot be empty"));
      }
      let global_config = GlobalConfig::new()?;
      for git_user_config in &global_config.git_user_configs {
        if git_user_config.config_id == *input {
          return Err(anyhow::anyhow!("The user config id already exists"));
        }
      }
      Ok(())
    })
    .interact()?;
  let name: String = Input::new()
    .with_prompt("Enter your git username")
    .validate_with(|input: &String| -> Result<()> {
      if input.is_empty() {
        return Err(anyhow::anyhow!("The username cannot be empty"));
      }
      Ok(())
    })
    .interact()?;

  let email: String = Input::new()
    .with_prompt("Enter your git email")
    .validate_with(|input: &String| -> Result<()> {
      if input.is_empty() {
        return Err(anyhow::anyhow!("The email cannot be empty"));
      }
      let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$")?;
      if !email_regex.is_match(input) {
        return Err(anyhow::anyhow!("The email is not valid"));
      }
      Ok(())
    })
    .interact()?;

  GitUserConfig::new(&config_id, &name, &email).add()?;

  println!(
    "Git user config {}({}<{}>) added successfully!",
    style(&config_id).bold(),
    style(&name).bold(),
    style(&email).bold()
  );

  Ok(())
}
