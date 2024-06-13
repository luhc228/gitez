use crate::GlobalConfig;
use anyhow::Result;
use console::style;

pub fn set_base_dir(base_dir: &str) -> Result<()> {
  let mut config = GlobalConfig::new()?;
  config.set_base_dir(base_dir)?;

  println!(
    "Base directory has been set to {} successfully.",
    style(base_dir).bold()
  );

  Ok(())
}
