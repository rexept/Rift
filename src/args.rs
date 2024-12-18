use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct RiftArgs {
    #[clap(subcommand)]
    pub entity_type: EntityType,
}

#[derive(Debug, Subcommand, PartialEq)]
pub enum EntityType {
    #[clap(author, version, about)]
    /// Manage the daemon
    Daemon(DaemonCommand),
    #[clap(author, version, about)]
    /// Open the menu
    Menu,
    #[clap(author, version, about)]
    /// Switch outputs
    Output(OutputCommand),
}

#[derive(Debug, Args, PartialEq)]
pub struct DaemonCommand {
    #[clap(subcommand)]
    pub command: DaemonSubcommand,
}

#[derive(Debug, Args, PartialEq)]
pub struct OutputCommand {
    #[clap(subcommand)]
    pub command: OutputSubcommand,
}

#[derive(Debug, Subcommand, PartialEq)]
pub enum DaemonSubcommand {
    #[clap(author, version, about)]
    /// Start the daemon
    Start,
    #[clap(author, version, about)]
    /// Stop the daemon
    Stop,
    #[clap(author, version, about)]
    /// Restart the daemon
    Restart,
    #[clap(author, version, about)]
    /// Print the status of the daemon
    Status,
}

#[derive(Debug, Subcommand, PartialEq)]
pub enum OutputSubcommand {
    #[clap(author, version, about)]
    /// Mirror your outputs
    Mirror,
    #[clap(author, version, about)]
    /// Extend your outputs
    Extend,
    #[clap(author, version, about)]
    /// Replace your outputs
    Replace,
}
