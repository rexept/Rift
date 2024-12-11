use crate::{eww, monitor};
use anyhow::Result;
use std::env;
use tokio::io::AsyncReadExt;
use tokio::net::UnixStream;
use tokio::process::Command;

enum HdmiConnection {
    Connected,
    Disconnected,
}

impl HdmiConnection {
    pub fn to_bool(&self) -> bool {
        match self {
            HdmiConnection::Connected => true,
            HdmiConnection::Disconnected => false,
        }
    }

    pub fn from_string(s: &str) -> Result<Self> {
        match s {
            "connected" => Ok(HdmiConnection::Connected),
            "disconnected" => Ok(HdmiConnection::Disconnected),
            _ => Err(anyhow::anyhow!("Invalid HDMI connection state")),
        }
    }
}

async fn hdmi_connected() -> Result<bool> {
    // get contents of /sys/class/drm/card1-HDMI-A-1/status
    let status = Command::new("cat")
        .arg("/sys/class/drm/card1-HDMI-A-1/status")
        .output()
        .await?;

    if status.status.code().unwrap() != 0 {
        return Err(anyhow::anyhow!("Failed to get HDMI connection status"));
    }

    let status: HdmiConnection =
        HdmiConnection::from_string(String::from_utf8(status.stdout)?.as_str().trim())?;

    return Ok(status.to_bool());
}

pub async fn connect_to_hyprland_socket() -> Result<()> {
    // Get the instance signature and runtime directory from the environment variables
    let instance_signature = env::var("HYPRLAND_INSTANCE_SIGNATURE")?;
    let runtime_directory = env::var("XDG_RUNTIME_DIR")?;

    // Create the path to the socket
    let socket_path = format!("{}/hypr/{}/.socket2.sock", runtime_directory, instance_signature);

    // Create a UnixStream
    let mut stream = UnixStream::connect(&socket_path).await?;

    // Buffer to store incoming messages
    let mut buffer = vec![0; 1024];
    let mut is_connected: Option<bool> = None;

    loop {
        // Read from the socket
        match stream.read(&mut buffer).await {
            Ok(size) => {
                if size == 0 {
                    break; // End of file (connection closed)
                }

                if is_connected.is_none() {
                    is_connected = Some(hdmi_connected().await?);
                }

                let connected = hdmi_connected().await?;
                // When it detects a change in HDMI status
                if connected != is_connected.unwrap() {
                    if !connected {
                        monitor::laptop_screen_on();
                    }
                    if connected {
                        eww::open_eww_widget().await?;
                    }
                    is_connected = Some(connected);
                }
            }
            Err(e) => {
                eprintln!("Failed to read from socket: {}", e);
                break;
            }
        }
    }

    return Ok(());
}

pub async fn disconnect_from_hyprland_socket() -> () {
    // Make better?
    let _ = Command::new("pkill").arg("rift").spawn();
}

pub async fn restart() -> Result<()> {
    disconnect_from_hyprland_socket().await;
    connect_to_hyprland_socket().await
}

pub async fn status() -> Result<()> {
    let exists = Command::new("pgrep")
        .arg("-x")
        .arg("rift")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .await?;

    match exists.code() {
        Some(0) => println!("Active"),
        _ => println!("Inactive"),
    };
    return Ok(());
}
