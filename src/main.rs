use anyhow::Result;
use args::{EntityType, OutputSubcommand, RiftArgs};
use clap::Parser;

pub mod args;
pub mod eww;
pub mod monitor;
pub mod socket;

#[tokio::main]
async fn main() -> Result<()> {
    let args = RiftArgs::parse();

    match args.entity_type {
        EntityType::Daemon(daemon) => match daemon.command {
            args::DaemonSubcommand::Start => socket::connect_to_hyprland_socket().await?,
            args::DaemonSubcommand::Stop => socket::disconnect_from_hyprland_socket().await,
            args::DaemonSubcommand::Restart => socket::restart().await?,
            args::DaemonSubcommand::Status => socket::status().await?,
        },
        EntityType::Menu => eww::monitor::open_eww_widget().await?,
        EntityType::Output(output) => match output.command {
            OutputSubcommand::Mirror => monitor::mirror()?,
            OutputSubcommand::Extend => monitor::extend()?,
            OutputSubcommand::Replace => monitor::replace()?,
        },
    }

    return Ok(());
}
