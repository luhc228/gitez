use crate::GlobalConfig;
use anyhow::Result;
use serde::{Deserialize, Serialize};

mod add;
mod apply;
mod include;
mod list;
mod remove;

pub use add::add as add_git_user_config;
pub use apply::apply as apply_git_user_config;
pub use include::include as include_git_config;
pub use list::list as list_git_user_config;
pub use remove::remove as remove_git_user_config;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GitUserConfig {
  pub config_id: String,
  pub name: String,
  pub email: String,
}

impl GitUserConfig {
  pub fn new(config_id: &str, name: &str, email: &str) -> Self {
    Self {
      config_id: config_id.to_string(),
      name: name.to_string(),
      email: email.to_string(),
    }
  }

  pub fn add(self) -> Result<()> {
    let mut global_config = GlobalConfig::new()?;
    global_config.add_git_user_config(self)?;
    Ok(())
  }

  pub fn get_all() -> Result<Vec<GitUserConfig>> {
    let global_config = GlobalConfig::new()?;
    Ok(global_config.get_git_user_configs())
  }

  pub fn remove(config_id: &str) -> Result<()> {
    let mut global_config = GlobalConfig::new()?;
    global_config.remove_git_user_config(config_id)?;
    Ok(())
  }
}
