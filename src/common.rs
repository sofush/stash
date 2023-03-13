use std::os::unix::fs::MetadataExt;

pub fn user_id() -> std::io::Result<u32> {
    Ok(std::fs::metadata("/proc/self").map(|m| m.uid())?)
}
