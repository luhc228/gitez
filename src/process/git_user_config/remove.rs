use crate::GitUserConfig;
use anyhow::{Ok, Result};
use console::style;

pub fn remove(config_id: String) -> Result<()> {
  GitUserConfig::remove(&config_id)?;

  println!("Git user config {} removed successfully!", style(config_id).bold());

  Ok(())
}
