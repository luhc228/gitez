use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

use crate::GitUserConfig;

const CONFIG_NAME: &str = ".gitez.json";

#[derive(Debug, Serialize, Deserialize)]
pub struct GlobalConfig {
  pub base_dir: Option<String>,
  pub git_user_configs: Vec<GitUserConfig>,
}

impl GlobalConfig {
  pub fn init() -> Result<()> {
    let config_path = GlobalConfig::config_path();
    if !config_path.exists() {
      fs::write(config_path, "{ git_user_configs: [] }")
        .map_err(|err| anyhow::anyhow!("Unable to create {}. Reason: {}", CONFIG_NAME, err))?;
    }
    Ok(())
  }
  pub fn config_path() -> PathBuf {
    match dirs::home_dir() {
      // The path is ～/.gitez.json
      Some(home_dir) => home_dir.join(CONFIG_NAME),
      None => PathBuf::from(CONFIG_NAME),
    }
  }
  pub fn new() -> Result<Self> {
    let config_path = GlobalConfig::config_path();
    let config_str = fs::read_to_string(config_path)
      .map_err(|err| anyhow::anyhow!("Unable to create {}. Reason: {}", CONFIG_NAME, err))?;
    let config = serde_json::from_str::<GlobalConfig>(&config_str)
      .map_err(|err| anyhow::anyhow!("Unable to parse {}. Reason: {}", CONFIG_NAME, err))?;
    Ok(config)
  }

  pub fn save(&self) -> Result<()> {
    let config_path = GlobalConfig::config_path();
    let config_str = serde_json::to_string_pretty(&self)
      .map_err(|err| anyhow::anyhow!("Unable to serialize {}. Reason: {}", CONFIG_NAME, err))?;
    fs::write(config_path, config_str)
      .map_err(|err| anyhow::anyhow!("Unable to write {}. Reason: {}", CONFIG_NAME, err))?;
    Ok(())
  }
}

impl GlobalConfig {
  pub fn set_base_dir(&mut self, base_dir: &str) -> Result<()> {
    self.base_dir = Some(base_dir.to_string());
    self.save()?;

    Ok(())
  }

  pub fn add_git_user_config(&mut self, git_user_config: GitUserConfig) -> Result<()> {
    self.git_user_configs.push(git_user_config);
    self.save()?;

    Ok(())
  }

  pub fn remove_git_user_config(&mut self, config_name: &str) -> Result<()> {
    self.git_user_configs.retain(|config| config.config_name != config_name);
    self.save()?;

    Ok(())
  }
}
