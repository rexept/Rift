use anyhow::Result;
use args::{RiftArgs, EntityType, OutputSubcommand};
use clap::Parser;

pub mod args;
pub mod eww;
pub mod monitor;
pub mod socket;

#[tokio::main]
async fn main() -> Result<()> {
    let args = RiftArgs::parse();

    match args.entity_type {
        EntityType::Daemon => socket::connect_to_hyprland_socket().await?,
        EntityType::Menu => eww::open_eww_widget().await?,
        EntityType::Output(output) => match output.command {
            OutputSubcommand::Mirror => monitor::mirror()?,
            OutputSubcommand::Extend => monitor::extend()?,
            OutputSubcommand::Replace => monitor::replace()?,
        },
    }

    return Ok(());
}
