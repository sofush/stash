use crate::errors::CustomError;
use std::path::PathBuf;
use anyhow::bail;

pub struct Mountpoint {
    pub filesystem: PathBuf,
    pub mountpoint: PathBuf,
    pub contains_home: bool,
}

pub fn mountpoints() -> anyhow::Result<Vec<Mountpoint>> {
    let file = std::fs::read_to_string("/proc/mounts")?;

    file.lines().map(|line| {
        let mut splits = line.split(' ');
        let device = splits.next();
        let mountpoint = splits.next();

        match (device, mountpoint) {
            (Some(d), Some(m)) => Ok(Mountpoint {
                filesystem: d.into(),
                mountpoint: m.into(),
                contains_home: false,
            }),
            _ => bail!(CustomError::FileParseFailed("/proc/mounts".into())),
        }
    }).collect()
}
