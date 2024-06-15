use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

use crate::GitUserConfig;

pub const DIR: &str = ".gitez";
const CONFIG_FILENAME: &str = "config.json";

#[derive(Debug, Serialize, Deserialize)]
pub struct GlobalConfig {
  pub base_dir: Option<String>,
  pub git_user_configs: Vec<GitUserConfig>,
}

impl GlobalConfig {
  pub fn init() -> Result<()> {
    let config_path = GlobalConfig::config_path();
    Self::init_dir()?;
    if !config_path.exists() {
      fs::write(&config_path, "{ \"git_user_configs\": [] }")
        .map_err(|err| anyhow::anyhow!("Unable to create {}. Reason: {}", config_path.display(), err))?;
    }
    Ok(())
  }
  fn init_dir() -> Result<()> {
    let config_path = GlobalConfig::config_path();
    let dir = config_path.parent().unwrap();
    if !dir.exists() {
      fs::create_dir(dir).map_err(|err| anyhow::anyhow!("Unable to create {}. Reason: {}", dir.display(), err))?;
    }
    Ok(())
  }
  pub fn config_path() -> PathBuf {
    match dirs::home_dir() {
      // The path is ～/.gitez/config.json
      Some(home_dir) => home_dir.join(DIR).join(CONFIG_FILENAME),
      None => PathBuf::from(DIR).join(CONFIG_FILENAME),
    }
  }
  pub fn new() -> Result<Self> {
    let config_path = GlobalConfig::config_path();
    let config_str = fs::read_to_string(&config_path)
      .map_err(|err| anyhow::anyhow!("Unable to create {}. Reason: {}", config_path.display(), err))?;
    let config = serde_json::from_str::<GlobalConfig>(&config_str)
      .map_err(|err| anyhow::anyhow!("Unable to parse {}. Reason: {}", config_path.display(), err))?;
    Ok(config)
  }

  pub fn save(&self) -> Result<()> {
    let config_path = GlobalConfig::config_path();
    let config_str = serde_json::to_string_pretty(&self)
      .map_err(|err| anyhow::anyhow!("Unable to serialize {}. Reason: {}", config_path.display(), err))?;
    fs::write(&config_path, config_str)
      .map_err(|err| anyhow::anyhow!("Unable to write {}. Reason: {}", config_path.display(), err))?;
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

  pub fn remove_git_user_config(&mut self, config_id: &str) -> Result<()> {
    self.git_user_configs.retain(|config| config.config_id != config_id);
    self.save()?;

    Ok(())
  }

  pub fn get_git_user_configs(&self) -> Vec<GitUserConfig> {
    self.git_user_configs.clone()
  }
}
