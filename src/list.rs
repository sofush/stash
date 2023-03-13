use std::path::Path;
use crate::trash_info::Entry;

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
