use anyhow::Result;
use std::process::Command;

fn close_eww() -> Result<()> {
    let home = std::env::var("HOME")?;
    let _ = Command::new("eww")
        .arg("-c")
        .arg(format!("{}/.config/eww/rift", home))
        .arg("close")
        .arg("rift")
        .spawn();
    return Ok(());
}

const MONITOR_CONTENTS: [&str; 7] = [
    "eDP-1,1920x1080@60,1920x0,1",
    "HDMI-A-1,3840x2160,0x0,1",
    "HDMI-A-1,1920x1080,0x0,1",
    "HDMI-A-1,2560x1080,0x0,1",
    "eDP-1,disable",
    "HDMI-A-1,preferred,auto,1,mirror,eDP-1",
    "eDP-1,preferred,auto,1,mirror,HDMI-A-1",
];

pub fn mirror_eww() -> Result<()> {
    let _ = Command::new("hyprctl")
        .arg("keyword")
        .arg("monitor")
        .arg(MONITOR_CONTENTS[0])
        .spawn();

    let _ = Command::new("hyprctl")
        .arg("keyword")
        .arg("monitor")
        .arg(MONITOR_CONTENTS[5])
        .spawn();

    close_eww()?;
    return Ok(());
}

pub fn extend_eww() -> Result<()> {
    let _ = Command::new("hyprctl")
        .arg("keyword")
        .arg("monitor")
        .arg(MONITOR_CONTENTS[0])
        .spawn();

    let _ = Command::new("hyprctl")
        .arg("keyword")
        .arg("monitor")
        .arg(MONITOR_CONTENTS[2])
        .spawn();

    close_eww()?;
    return Ok(());
}

pub fn replace_eww() -> Result<()> {
    let _ = Command::new("hyprctl")
        .arg("keyword")
        .arg("monitor")
        .arg(MONITOR_CONTENTS[2])
        .spawn();

    let _ = Command::new("hyprctl")
        .arg("keyword")
        .arg("monitor")
        .arg(MONITOR_CONTENTS[4])
        .spawn();

    close_eww()?;
    return Ok(());
}

pub fn mirror() -> Result<()> {
    let _ = Command::new("hyprctl")
        .arg("keyword")
        .arg("monitor")
        .arg(MONITOR_CONTENTS[0])
        .spawn();

    let _ = Command::new("hyprctl")
        .arg("keyword")
        .arg("monitor")
        .arg(MONITOR_CONTENTS[5])
        .spawn();

    return Ok(());
}

pub fn extend() -> Result<()> {
    let _ = Command::new("hyprctl")
        .arg("keyword")
        .arg("monitor")
        .arg(MONITOR_CONTENTS[0])
        .spawn();

    let _ = Command::new("hyprctl")
        .arg("keyword")
        .arg("monitor")
        .arg(MONITOR_CONTENTS[2])
        .spawn();

    return Ok(());
}

pub fn replace() -> Result<()> {
    let _ = Command::new("hyprctl")
        .arg("keyword")
        .arg("monitor")
        .arg(MONITOR_CONTENTS[2])
        .spawn();

    let _ = Command::new("hyprctl")
        .arg("keyword")
        .arg("monitor")
        .arg(MONITOR_CONTENTS[4])
        .spawn();

    return Ok(());
}

pub fn laptop_screen_on() -> () {
    let _ = Command::new("hyprctl")
        .arg("keyword")
        .arg("monitor")
        .arg(MONITOR_CONTENTS[0])
        .spawn();
}
