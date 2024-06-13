use crate::GlobalConfig;
use anyhow::Result;

pub fn set_base_dir(base_dir: &str) -> Result<()> {
  let mut config = GlobalConfig::new()?;
  config.set_base_dir(base_dir)?;

  println!("Base directory has been set to {} successfully.", base_dir);

  Ok(())
}
