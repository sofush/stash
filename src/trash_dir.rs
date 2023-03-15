use anyhow::bail;
use std::io::Write;
use path_absolutize::Absolutize;
use crate::common;
use crate::errors::CustomError;
use crate::mountpoint::{self, Mountpoint};
use std::fs::OpenOptions;
use std::os::unix::prelude::MetadataExt;
use std::path::{PathBuf, Path};
use std::rc::Rc;

#[derive(Debug, Clone, Copy)]
pub enum TrashMode {
    /// Simulate what would happen without taking any action
    Simulate,

    /// Moves the file into the trashcan
    Move,

    /// Copies the file into the trashcan, without deleting the original file
    Copy,
}

#[derive(Debug, Clone)]
pub struct Topdir {
    /// .Trash/$uid
    shared: PathBuf,

    /// .Trash-$uid
    personal: PathBuf,
}

#[derive(Debug, Clone)]
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

    pub fn mountpoint(&self) -> Rc<Mountpoint> {
        Rc::clone(match self {
            Dir::Home(m, _) => m,
            Dir::Topdir(m, _) => m,
        })
    }

    pub fn put(&self, file: &Path, query: &str, trash_mode: TrashMode) -> anyhow::Result<()> {
        let abs = file.absolutize()?;

        match self {
            Dir::Home(_, home_trash_dir) => {
                std::fs::create_dir_all(home_trash_dir)?;
                self.trash(&abs, home_trash_dir, trash_mode)
            },
            Dir::Topdir(_, topdirs) => {
                let shared_parent = topdirs.shared.parent().unwrap();
                let mode = shared_parent.metadata()?.mode();
                let is_sticky = (mode & 0o1000).count_ones() != 1;

                // TODO: check this works
                dbg!(&is_sticky);

                if !shared_parent.is_symlink() || is_sticky {
                    if !topdirs.shared.exists() && std::fs::create_dir(&topdirs.shared).is_ok(){
                        self.trash(&abs, &topdirs.shared, trash_mode)?;
                    }
                } else {
                    // TODO: report the failed check to the administrator
                }

                if !topdirs.personal.exists() {
                    if std::fs::create_dir(&topdirs.personal).is_err() {
                        let dir_name = topdirs.personal.to_str().unwrap_or("<error>");
                        bail!(CustomError::CreateDirectoryFailed(dir_name.into(), query.into()))
                    }
                    self.trash(&abs, &topdirs.personal, trash_mode)?;
                }

                Ok(())
            },
        }
    }

    fn trash(&self, file: &Path, trash_dir: &Path, mode: TrashMode) -> anyhow::Result<()> {
        // NOTE: creating entry before moving the file, as required by the specification
        let absolute = file.absolutize()?;
        let root = trash_dir.join("..");
        let root = root.absolutize()?;

        if !absolute.starts_with(&root) {
            bail!(CustomError::FileNotRelative(
                    absolute.to_string_lossy().into(), 
                    trash_dir.to_string_lossy().into())
            )
        }

        let relative_path = absolute.strip_prefix(&root)?;
        let relative_path_str = match relative_path.to_str() {
            Some(str) => str.to_string(),
            None => bail!(CustomError::CouldNotRetrieveFileName(file.to_string_lossy().into())),
        };
        let entry = crate::trash_info::Entry::new(relative_path_str)?;

        let file_name = match file.file_name() {
            Some(name) => match name.to_str() {
                Some(name) => name,
                None => bail!(CustomError::CouldNotRetrieveFileName(name.to_string_lossy().into())),
            },
            None => bail!(CustomError::CouldNotRetrieveFileName(file.to_string_lossy().into())),
        };

        let destination_file_name = self.find_suitible_name(file_name, trash_dir)?;
        let destination_info_path = trash_dir
            .join("info")
            .join(&destination_file_name)
            .join(".trashinfo");
        let destination_files_path = trash_dir
            .join("files")
            .join(&destination_file_name);

        let mut info_file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(destination_info_path)?;

        // NOTE: This file is being created but not used to mitigate a file being overwrited on accident
        let mut _destination_file = OpenOptions::new()
            .create_new(true)
            .open(&destination_files_path)?;

        write!(info_file, "{}", entry.to_string())?;

        match mode {
            TrashMode::Simulate => {
                let from = file.to_string_lossy();
                let to = destination_files_path.to_string_lossy();
                println!("simulate: moving {from} to {to}");
            },
            TrashMode::Move => std::fs::rename(file, destination_files_path)?,
            TrashMode::Copy => { let _ = std::fs::copy(file, destination_files_path)?; },
        }

        Ok(())
    }

    fn find_suitible_name(&self, file_name: &str, trash_dir: &Path) -> anyhow::Result<String> {
        let files_subdir = trash_dir.join("files");
        let info_subdir = trash_dir.join("info");

        if !files_subdir.join(file_name).exists() && !info_subdir.join(file_name).join(".trashinfo").exists() {
            Ok(file_name.into())
        } else {
            let new_stem = (0..u32::MAX)
                .into_iter()
                .find(|i| !info_subdir.join(format!("{}_{}.trashinfo", file_name, i)).exists()
                      && !files_subdir.join(format!("{}_{}", file_name, i)).exists())
                .map(|i| format!("{}_{}", file_name, i));

            match new_stem {
                Some(stem) => Ok(stem),
                None => bail!(CustomError::UnableToFindSuitableName(file_name.into())),
            }
        }
    }
}
