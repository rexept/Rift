
pub mod monitor {
    use anyhow::Result;
    use std::env;
    use tokio::fs::{self, File};
    use tokio::io;
    use tokio::process::Command;
    use tokio::time::{sleep, Duration};
    async fn create_eww() -> Result<()> {
        let home = env::var("HOME")?;
        const YUCK_CONTENT: &[u8] = include_bytes!("../templates/template.yuck");
        const SCSS_CONTENT: &[u8] = include_bytes!("../templates/template.scss");

        let mut yuck_content = YUCK_CONTENT;
        let mut scss_content = SCSS_CONTENT;

        // Create the directory if it doesnt exist
        if !path_exists(&format!("{}/.config/eww", home)).await? {
            fs::create_dir_all(format!("{}/.config/eww", home)).await?;
        }
        if !path_exists(&format!("{}/.config/eww/rift", home)).await? {
            fs::create_dir_all(format!("{}/.config/eww/rift", home)).await?;
        }

        // Write files
        let yuck_path = format!("{}/.config/eww/rift/eww.yuck", home);
        let scss_path = format!("{}/.config/eww/rift/eww.scss", home);
        io::copy(&mut yuck_content, &mut File::create(&yuck_path).await?).await?;
        io::copy(&mut scss_content, &mut File::create(&scss_path).await?).await?;

        return Ok(());
    }

    async fn run_eww(eww: String) -> Result<()> {
        let _ = Command::new("sh")
            .arg("-c")
            .arg(format!("{} open-many rift", eww))
            .spawn();
        return Ok(());
    }

    pub async fn open_eww_widget() -> Result<()> {
        // Files and cmd
        let home = env::var("HOME")?;
        let eww = format!("eww -c {}/.config/eww/rift", home);

        if !path_exists(&format!("{}/.config/eww/rift", home)).await? {
            create_eww().await?;
        }
        if !path_exists(&format!("{}/.config/eww/rift/eww.yuck", home)).await? {
            create_eww().await?;
        }
        if !path_exists(&format!("{}/.config/eww/rift/eww.scss", home)).await? {
            create_eww().await?;
        }

        // Run eww daemon if not running already
        if Command::new("sh")
            .arg("-c")
            .arg("pidof eww")
            .output()
            .await?
            .stdout
            .is_empty()
        {
            let _ = Command::new("sh").arg("-c").arg("eww daemon").spawn();
            sleep(Duration::from_secs(1)).await;
        }

        run_eww(eww).await?;
        return Ok(());
    }

    async fn path_exists(path: &str) -> Result<bool> {
        // Use tokio::fs::metadata to get metadata for the path
        match fs::metadata(path).await {
            Ok(_) => Ok(true),
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(false),
            Err(err) => Err(err.into()),
        }
    }
}
