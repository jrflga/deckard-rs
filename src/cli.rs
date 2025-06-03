use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "deckard")]
#[command(about = "Database copy tool", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Manage configuration
    #[command(subcommand)]
    Config(ConfigCommands),
    /// Copy databases
    Copy(CopyArgs),
    /// Schedule copy task
    Schedule(ScheduleArgs),
    /// Resume a previous task
    Resume(ResumeArgs),
}

#[derive(Subcommand, Debug)]
pub enum ConfigCommands {
    /// Set environment configuration
    Set(ConfigSetArgs),
    /// Get environment configuration
    Get(ConfigGetArgs),
}

#[derive(Args, Debug)]
pub struct ConfigSetArgs {
    /// Environment name
    pub env: String,
    #[arg(long)]
    pub db_type: String,
    #[arg(long)]
    pub host: String,
    #[arg(long)]
    pub port: u16,
    #[arg(long)]
    pub user: String,
    #[arg(long)]
    pub password: String,
}

#[derive(Args, Debug)]
pub struct ConfigGetArgs {
    pub env: String,
}

#[derive(Args, Debug)]
pub struct CopyArgs {
    #[arg(long)]
    pub source: String,
    #[arg(long)]
    pub target: String,
    #[arg(long)]
    pub tables: Option<String>,
    #[arg(long, value_name = "TABLE")]
    pub table: Option<String>,
    #[arg(long, value_name = "FILTER")]
    pub where_: Option<String>,
    #[arg(long)]
    pub transform: Option<String>,
    #[arg(long)]
    pub dry_run: bool,
    #[arg(long)]
    pub concurrency: Option<usize>,
    #[arg(long)]
    pub integrity_check: bool,
}

#[derive(Args, Debug)]
pub struct ScheduleArgs {
    #[arg(long)]
    pub source: String,
    #[arg(long)]
    pub target: String,
    #[arg(long)]
    pub cron: String,
}

#[derive(Args, Debug)]
pub struct ResumeArgs {
    #[arg(long, value_name = "TASK_ID")]
    pub task_id: String,
}
