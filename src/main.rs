use clap::Parser;
use gitez::{Cli, Commands, ConfigSubCommands};

fn main() -> anyhow::Result<()> {
  let cli = Cli::parse();

  match cli.command {
    Commands::SetBaseDir(base_dir) => {
      println!("Setting base directory. {}", base_dir.base_dir);
    },
    Commands::Clone(clone) => {
      println!("Cloning repository. {}", clone.url);
    },
    Commands::GenSSHKey => {
      println!("Generating SSH key.");
    },
    Commands::Config(config) => {
      match config {
        ConfigSubCommands::Add => {
          println!("Adding config.");
        },
        ConfigSubCommands::Set { config_name } => {
          println!("Setting config: {}", config_name.unwrap());
        },
        ConfigSubCommands::List => {
          println!("Listing configs.");
        },
        ConfigSubCommands::Remove { config_name } => {
          println!("Removing config: {}", config_name.unwrap());
        },
        ConfigSubCommands::AddInclude => {
          println!("Adding include.");
        },
      }
    },
  }

  Ok(())
}
