use std::os::unix::fs::MetadataExt;
use std::path::PathBuf;

pub fn user_id() -> std::io::Result<u32> {
    Ok(std::fs::metadata("/proc/self").map(|m| m.uid())?)
}

pub fn get_home_trash_dir() -> anyhow::Result<PathBuf> {
    let xdg_data_home = std::env::var("XDG_DATA_HOME")?;
    Ok(PathBuf::from(format!("{xdg_data_home}/Trash")))
}
