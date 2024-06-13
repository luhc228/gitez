use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct Cli {
  #[command(subcommand)]
  pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
  #[command(subcommand)]
  Config(ConfigSubCommands),
  #[command(
    name = "set-base-dir",
    about = "Set the base directory where your git repositories located."
  )]
  SetBaseDir(SetBaseDirOptions),
  #[command(name = "clone", about = "Clone a repository to the base directory.")]
  Clone(CloneOptions),
  #[command(name = "gen-ssh-key", about = "Generate a new SSH key.")]
  GenSSHKey,
}

#[derive(Parser, Debug)]
pub struct CloneOptions {
  #[arg()]
  pub url: String,
}

#[derive(Parser, Debug)]
pub struct SetBaseDirOptions {
  #[arg()]
  pub base_dir: String,
}

#[derive(Subcommand, Debug)]
pub enum ConfigSubCommands {
  #[command(name = "add", about = "Add a git config.")]
  Add,
  #[command(name = "list", about = "List all git configs.")]
  List,
  #[command(name = "apply", about = "Apply git config to current directory.")]
  Apply {
    #[arg(help = "The name of the config to set.")]
    config_name: Option<String>,
  },
  #[command(name = "set", about = "Remove a git config.")]
  Remove {
    #[arg(help = "The name of the config to set.")]
    config_name: Option<String>,
  },
  #[command(
    name = "add-include",
    about = "Include a config file to the name of the file to be included."
  )]
  AddInclude,
}
