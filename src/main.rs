use clap::Parser;
use gitez::{Cli, Commands, ConfigSubCommands, GlobalConfig, set_base_dir};

fn main() -> anyhow::Result<()> {
  GlobalConfig::init_config_file()?;

  let cli = Cli::parse();

  match cli.command {
    Commands::SetBaseDir(opts) => {
      set_base_dir(opts.base_dir)?;
    },
    Commands::Clone(opts) => {
      println!("Cloning repository. {}", opts.url);
    },
    Commands::GenSSHKey => {
      println!("Generating SSH key.");
    },
    Commands::Config(config_subcommands) => {
      match config_subcommands {
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
