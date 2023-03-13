use std::path::Path;
use crate::trash_info::Entry;

pub fn list(trash_dir: &Path) -> anyhow::Result<Vec<anyhow::Result<Entry>>> {
    let entries = std::fs::read_dir(trash_dir.join("info"))?
        .map(|file| -> anyhow::Result<Entry> {
            let file = file?;
            Entry::from_file(&file.path())
        })
        .collect::<Vec<anyhow::Result<Entry>>>();

    Ok(entries)
}
