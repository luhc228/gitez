use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

use crate::GitUserConfig;

const DIR: &str = ".gitez";
const CONFIG_FILENAME: &str = "config.json";

pub struct GlobalDir(pub PathBuf);

impl GlobalDir {
  pub fn new() -> Self {
    let global_dir = Self::global_dir();
    if !global_dir.exists() {
      fs::create_dir(&global_dir)
        .map_err(|err| anyhow::anyhow!("Unable to create {}. Reason: {}", global_dir.display(), err))
        .expect("Unable to create global directory");
    }
    Self(global_dir)
  }
  fn global_dir() -> PathBuf {
    match dirs::home_dir() {
      // The path is ～/.gitez
      Some(home_dir) => home_dir.join(DIR),
      None => PathBuf::from(DIR),
    }
  }
}

impl Default for GlobalDir {
  fn default() -> Self {
    Self::new()
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GlobalConfig {
  pub base_dir: Option<String>,
  pub git_user_configs: Vec<GitUserConfig>,
}

impl GlobalConfig {
  pub fn init() -> Result<()> {
    Self::init_config()?;
    Ok(())
  }

  fn init_config() -> Result<()> {
    let config_path = GlobalConfig::config_path();
    if !config_path.exists() {
      fs::write(&config_path, "{ \"git_user_configs\": [] }")
        .map_err(|err| anyhow::anyhow!("Unable to create {}. Reason: {}", config_path.display(), err))?;
    }
    Ok(())
  }

  pub fn config_path() -> PathBuf {
    let global_dir = GlobalDir::new();
    global_dir.0.join(CONFIG_FILENAME)
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
