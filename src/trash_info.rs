use std::path::{PathBuf, Path};
use time::{PrimitiveDateTime, OffsetDateTime, format_description::FormatItem, macros::format_description};
use anyhow::bail;
use crate::errors::CustomError;

const DATETIME_FORMAT: &[FormatItem] = format_description!("[year]-[month]-[day]T[hour]:[minute]:[second]");
const HEADER: &str = "[Trash Info]";

#[derive(Debug, Clone, PartialEq)]
struct Values {
    path: String,
    datetime: PrimitiveDateTime,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Entry {
    location: Option<PathBuf>,
    values: Values,
}

impl Into<String> for Entry {
    fn into(self) -> String {
        self.to_string()
    }
}

impl ToString for Entry {
    fn to_string(&self) -> String {
        let datetime = self.values.datetime.format(DATETIME_FORMAT).unwrap();
        let mut out = String::new();
        out.push_str(&format!("{HEADER}\n"));
        out.push_str(&format!("Path={}\n", self.values.path));
        out.push_str(&format!("DeletionDate={}\n", datetime));
        out
    }
}

impl Entry {
    pub fn new(
        path: &Path, 
        datetime: Option<PrimitiveDateTime>
    ) -> anyhow::Result<Entry>{
        let datetime = match datetime {
            Some(datetime) => datetime,
            None => {
                let tmp = unsafe {
                    use time::util::local_offset::{self, Soundness};
                    local_offset::set_soundness(Soundness::Unsound);
                    OffsetDateTime::now_local()
                }?;
                PrimitiveDateTime::new(tmp.date(), tmp.time())
            },
        };

        let path = match path.to_str() {
            Some(path) => path,
            None => bail!(CustomError::PathInvalidUnicode),
        };

        Ok(Entry {
            location: None,
            values: Values {
                path: path.into(),
                datetime,
            }
        })
    }

    pub fn from_file(path: &Path) -> anyhow::Result<Entry> {
        let file_contents = std::fs::read_to_string(path)?;

        let mut entry = match Entry::parse(&file_contents) {
            Some(entry) => entry,
            None => bail!(CustomError::TrashInfoParseFailure(path.to_string_lossy().into())),
        };

        entry.location = Some(path.to_path_buf());
        Ok(entry)
    }

    pub fn parse(entry: &str) -> Option<Entry> {
        let mut lines = entry.lines();
        let header = lines.next();
        let path = lines.next();
        let datetime = lines.next();

        match (header, path, datetime) {
            (Some(header), Some(path), Some(datetime)) => {
                if header != HEADER {
                    return None;
                }

                let path = match path.split_once('=') {
                    Some((key, value)) if key == "Path" => value,
                    _ => return None,
                };

                let datetime = match datetime.split_once('=') {
                    Some((key, value)) if key == "DeletionDate" => PrimitiveDateTime::parse(value, DATETIME_FORMAT).ok()?,
                    _ => return None,
                };

                Some(Entry {
                    location: None,
                    values: Values {
                        path: path.into(),
                        datetime,
                    }
                })
            },
            _ => return None,
        }
    }

    pub fn file(&self) -> Option<PathBuf> {
        let path = match &self.location {
            Some(path) => path,
            None => return None,
        };

        let stem = path.file_stem()?;
        let root = path.ancestors().nth(2)?;
        Some(root.join("files").join(stem))
    }

    pub fn missing_file(&self) -> bool {
        match self.file() {
            Some(path) => !path.exists(),
            None => return true,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Write;
    use time::{Date, Time};
    use rand::Rng;

    use super::*;

    #[test]
    fn valid_trash_info_entry_parses() -> anyhow::Result<()> {
        const PATH: &str = "/tmp/testfile";

        let mut rand = rand::thread_rng();
        let date = Date::from_ordinal_date(rand.gen_range(2000..=2050), rand.gen_range(1..=365))?;
        let time = Time::from_hms(rand.gen_range(0..=23), rand.gen_range(0..=59), rand.gen_range(0..=59))?;
        let datetime = PrimitiveDateTime::new(date, time);
        let expected = Entry::new(Path::new(PATH), Some(datetime))?;

        let mut out = String::new();
        writeln!(&mut out, "{HEADER}")?;
        writeln!(&mut out, "Path={PATH}")?;
        writeln!(&mut out, "DeletionDate={}", datetime.format(DATETIME_FORMAT)?)?;

        assert_eq!(Some(expected), Entry::parse(&out));
        Ok(())
    }
}
