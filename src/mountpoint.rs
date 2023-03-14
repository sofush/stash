use crate::common;
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

    let mut mountpoints = file.lines().map(|line| {
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
    }).collect::<anyhow::Result<Vec<Mountpoint>>>()?;

    mountpoints.sort_unstable_by(|a, b| a.mountpoint.cmp(&b.mountpoint));

    let home_trash_dir = common::get_home_trash_dir()?;
    let mut matching: Vec<&mut Mountpoint> = mountpoints
        .iter_mut()
        .filter(|m| home_trash_dir.starts_with(&m.mountpoint))
        .take(1)
        .collect();

    match matching.len() {
        1 => matching[0].contains_home = true,
        _ => bail!(CustomError::HomeTrashNotDetermined(matching.len()))
    }

    Ok(mountpoints)
}
