use crate::common;
use crate::mountpoint::{self, Mountpoint};
use std::path::PathBuf;
use std::rc::Rc;

pub struct Topdir {
    /// .Trash/$uid
    shared: PathBuf,

    /// .Trash-$uid
    personal: PathBuf,
}

pub enum Dir {
    Home(Rc<Mountpoint>, PathBuf),
    Topdir(Rc<Mountpoint>, Topdir),
}

impl Dir {
    pub fn new(mountpoint: Rc<Mountpoint>, uid: u32) -> anyhow::Result<Dir> {
        if mountpoint.contains_home {
            Ok(Dir::Home(mountpoint, common::get_home_trash_dir()?))
        } else {
            let topdir = mountpoint.mountpoint.clone();

            Ok(Dir::Topdir(mountpoint, Topdir {
                shared: topdir.join(".Trash"),
                personal: topdir.join(format!(".Trash-{uid}")),
            }))
        }
    }

    pub fn all() -> anyhow::Result<Vec<anyhow::Result<Dir>>> {
        let uid = common::user_id()?;

        Ok(mountpoint::mountpoints()?
            .into_iter()
            .map(|mountpoint| Dir::new(Rc::from(mountpoint), uid))
            .collect())
    }

    pub fn paths(&self) -> Vec<PathBuf> {
        match self {
            Dir::Home(_, path) => vec![path.clone()],
            Dir::Topdir(_, topdir) => vec![topdir.shared.clone(), topdir.personal.clone()],
        }
    }
}
