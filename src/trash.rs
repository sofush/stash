use std::path::Path;
use anyhow::bail;
use path_absolutize::Absolutize;
use crate::{Dir, errors::CustomError, trash_dir::TrashMode};

type Query = String;

pub fn trash(query: Query, dirs: Vec<Dir>, trash_mode: TrashMode) -> anyhow::Result<()> {
    let path = Path::new(&query).absolutize()?;

    if !path.exists() {
        bail!(CustomError::BadQueryFileNotFound(query.clone()));
    }

    let mut dirs: Vec<Dir> = dirs
        .into_iter()
        .filter(|dir| {
            match dir {
                Dir::Home(m, _) => m.mountpoint.starts_with(&path),
                Dir::Topdir(m, _) => m.mountpoint.starts_with(&path),
            }
        })
        .collect();

    dirs.sort_unstable_by(|a, b| {
        a.mountpoint().mountpoint.cmp(&b.mountpoint().mountpoint)
    });

    let trash_dir = match dirs.last() {
        Some(dir) => dir,
        None => bail!(CustomError::TrashDirNotFound(query.clone())),
    };

    trash_dir.put(&path, &query, trash_mode)?;

    Ok(())
}
