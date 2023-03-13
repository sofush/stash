use std::path::PathBuf;
use uuid::Uuid;
use stash_rs::trash_info::Entry;
use std::io::Write;

fn create_test_dir() -> anyhow::Result<PathBuf> {
    let uuid = Uuid::new_v4().to_string();
    let path = PathBuf::from(format!("/tmp/stash-test-dir/{uuid}"));

    std::fs::create_dir_all(path.join("info"))?;
    std::fs::create_dir_all(path.join("files"))?;

    Ok(path)
}

#[cfg(test)]
mod tests {
    use stash_rs::list;

    use super::*;

    #[test]
    fn listing_empty_directory_contains_no_entries() -> anyhow::Result<()> {
        let path = create_test_dir()?;
        let entries = stash_rs::list(&path)?;

        assert_eq!(0, entries.len());

        Ok(())
    }

    #[test]
    fn listing_directory_with_one_valid_entry_contains_one() -> anyhow::Result<()> {
        let path = create_test_dir()?;
        let entry = Entry::new(&path, None)?;
        let filename = format!("{}.trashinfo", Uuid::new_v4().to_string());

        let mut file = std::fs::File::options()
            .create_new(true)
            .write(true)
            .open(path.join("info").join(filename))?;

        write!(file, "{}", entry.to_string())?;

        let entries = list(&path)?;
        assert_eq!(1, entries.len());

        Ok(())
    }
}
