use clap::Parser;
use gitez::{
  add_git_user_config, apply_git_user_config, clone_repo, get_git_user_config_id, include_git_config,
  list_git_user_config, remove_git_user_config, set_base_dir, Cli, Commands, GlobalConfig, UserConfigSubCommands,
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
      UserConfigSubCommands::Apply { config_id } => {
        let config_id = get_git_user_config_id(config_id)?;
        apply_git_user_config(config_id)?
      }
      UserConfigSubCommands::List => {
        list_git_user_config()?;
      }
      UserConfigSubCommands::Remove { config_id } => {
        let config_id = get_git_user_config_id(config_id)?;
        remove_git_user_config(config_id)?;
      }
      UserConfigSubCommands::AddInclude => {
        include_git_config()?;
      }
    },
  }

  Ok(())
}
