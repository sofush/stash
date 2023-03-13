use crate::mountpoint::{self, Mountpoint};
use std::path::PathBuf;

pub struct TrashDir {
    mountpoint: Mountpoint,
    path: PathBuf,
}

impl TrashDir {
    pub fn from(mountpoint: Mountpoint) -> TrashDir {
        todo!();
    }

    pub fn all() -> anyhow::Result<Vec<TrashDir>> {
        Ok(mountpoint::mountpoints()?
           .into_iter()
           .map(|mountpoint| TrashDir::from(mountpoint))
           .collect())
    }
}
