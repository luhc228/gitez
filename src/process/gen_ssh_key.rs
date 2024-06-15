use crate::{run_piped_command, select_git_user_config};
use anyhow::Result;
use console::style;
use core::str;
use dialoguer::Input;
use dirs::home_dir;
use regex::Regex;
use std::{fs, path::PathBuf};

pub fn gen_ssh_key() -> Result<()> {
  let host_name: String = Input::new()
    .with_prompt("Enter the host name . e.g.: github.com, gitlab.com.")
    .validate_with(|input: &String| -> Result<()> {
      if input.is_empty() {
        return Err(anyhow::anyhow!("The host name cannot be empty"));
      }
      // check if the host name is valid
      let host_name_regex = Regex::new(r"^[a-zA-Z0-9.-]+$")?;
      if !host_name_regex.is_match(input) {
        return Err(anyhow::anyhow!("The host name is not valid"));
      }
      Ok(())
    })
    .interact()?;

  let git_user_config = select_git_user_config()?;

  let ssh_dir = home_dir().expect("home dir is not found").join(".ssh");
  std::fs::create_dir_all(&ssh_dir)?;

  let rsa_path = ssh_dir.join(gen_rsa_filename(&git_user_config.config_id));

  if rsa_path.exists() {
    println!(
      "{}",
      style(format!("🔑 SSH key already exists at: {}", rsa_path.display())).yellow()
    );
    return Ok(());
  }

  run_piped_command(vec![
    "ssh-keygen",
    "-t",
    "rsa",
    "-b",
    "4096",
    "-C",
    &git_user_config.email,
    "-f",
    &rsa_path.to_str().expect("failed to convert path to string"),
  ])?;

  let rsa_pub_path = rsa_path.with_extension("pub");

  let host_config = gen_ssh_host_config(&host_name, &git_user_config.name, &rsa_path.to_string_lossy());
  add_ssh_host_to_config(ssh_dir, &host_config)?;

  let rsa_pub = fs::read_to_string(rsa_pub_path)?;
  println!("🔑 Public SSH Key:\n{}", rsa_pub);
  println!("You can add this key to your git account");

  Ok(())
}

fn gen_rsa_filename(config_id: &str) -> String {
  format!("id_rsa_{}", config_id)
}

fn gen_ssh_host_config(host_name: &str, username: &str, rsa_path: &str) -> String {
  format!(
    r#"
Host {host_name}
  HostName {host_name}
  User {username}
  PreferredAuthentications publickey
  IdentityFile {rsa_path}
"#
  )
}

fn add_ssh_host_to_config(ssh_dir: PathBuf, host_config: &str) -> Result<()> {
  let ssh_config_path = ssh_dir.join("config");
  if !ssh_config_path.exists() {
    fs::write(&ssh_config_path, "")?;
  }
  let mut source = fs::read_to_string(&ssh_config_path)?;
  source.push_str(host_config);
  fs::write(ssh_config_path, source)?;

  Ok(())
}
