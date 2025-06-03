mod cli;
mod config;
mod operations;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands, ConfigCommands};
use config::{Config, DbConfig};
use operations::*;

fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut config = Config::load()?;

    match cli.command {
        Commands::Config(sub) => match sub {
            ConfigCommands::Set(args) => {
                let db = DbConfig {
                    db_type: args.db_type,
                    host: args.host,
                    port: args.port,
                    user: args.user,
                    password: args.password,
                };
                config_set(&mut config, args.env, db)?;
            }
            ConfigCommands::Get(args) => {
                if let Some(env) = config_get(&config, &args.env)? {
                    println!("{}", serde_json::to_string_pretty(&env)?);
                } else {
                    println!("Environment not found");
                }
            }
        },
        Commands::Copy(args) => {
            copy_database(args)?;
        }
        Commands::Schedule(args) => {
            schedule_copy(args)?;
        }
        Commands::Resume(args) => {
            resume_task(args)?;
        }
    }
    Ok(())
}
