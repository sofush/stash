use std::path::{Path, PathBuf};
use crate::{trash_info::Entry, Dir};

pub fn list(trash_dir: &Path) -> anyhow::Result<Vec<anyhow::Result<Entry>>> {
    let info = trash_dir.join("info");

    if !info.exists() {
        return Ok(vec![]);
    }

    let entries = std::fs::read_dir(info)?
        .map(|file| -> anyhow::Result<Entry> {
            let file = file?;
            Entry::from_file(&file.path())
        })
        .collect::<Vec<anyhow::Result<Entry>>>();

    Ok(entries)
}

pub fn list_all() -> anyhow::Result<Vec<anyhow::Result<Entry>>> {
        let dirs = Dir::all()?
        .into_iter()
        .filter_map(|d| d.ok())
        .collect::<Vec<Dir>>();

    let paths = dirs
        .iter()
        .flat_map(|d| d.paths())
        .collect::<Vec<PathBuf>>();

    let mut listings = vec![];

    for path in paths {
        let mut entries = list(&path)?;
        listings.append(&mut entries);
    }

    Ok(listings)
}
