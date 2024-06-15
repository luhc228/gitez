use anyhow::Ok;

use super::select_git_user_config;

pub fn get_git_user_config_id(raw_config_id: Option<String>) -> anyhow::Result<String> {
  let config_id: String = match raw_config_id {
    Some(config_id) => config_id,
    None => {
      let git_user_config = select_git_user_config()?;
      git_user_config.config_id
    }
  };

  Ok(config_id)
}
