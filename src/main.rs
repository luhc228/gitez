use clap::Parser;
use gitez::{
  add_git_user_config, clone_repo, remove_git_user_config, set_base_dir, Cli, Commands, GlobalConfig,
  UserConfigSubCommands,
};

fn main() -> anyhow::Result<()> {
  GlobalConfig::init()?;

  let cli = Cli::parse();

  match cli.command {
    Commands::SetBaseDir(opts) => {
      set_base_dir(&opts.base_dir)?;
    }
    Commands::Clone(opts) => {
      clone_repo(&opts.url)?;
    }
    Commands::GenSSHKey => {
      println!("Generating SSH key.");
    }
    Commands::UserConfig(user_config_subcommands) => match user_config_subcommands {
      UserConfigSubCommands::Add => {
        add_git_user_config()?;
      }
      UserConfigSubCommands::Apply { config_name } => {
        println!("Setting config: {}", config_name.unwrap());
      }
      UserConfigSubCommands::List => {
        println!("Listing configs.");
      }
      UserConfigSubCommands::Remove { config_name } => {
        remove_git_user_config(config_name)?;
      }
      UserConfigSubCommands::AddInclude => {
        println!("Adding include.");
      }
    },
  }

  Ok(())
}
