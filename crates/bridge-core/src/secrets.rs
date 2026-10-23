//! Unix private local files. The application user and administrators are trusted.
use crate::error::BridgeError;
use std::{
    fs::{self, DirBuilder, File, OpenOptions},
    io::{self, Read, Write},
    os::unix::fs::{DirBuilderExt, MetadataExt, OpenOptionsExt, PermissionsExt},
    path::{Component, Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
};

/// Deliberately has no serialization or Display implementation.
pub struct Secret(String);
impl std::fmt::Debug for Secret {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Secret([REDACTED])")
    }
}
impl Secret {
    pub fn expose(&self) -> &str {
        &self.0
    }
}
/// Load at most 64 KiB of UTF-8 secret text from a private regular local file.
pub fn load_secret(path: &Path) -> Result<Secret, BridgeError> {
    let bytes = read_private(path, 65536)?;
    let value = String::from_utf8(bytes).map_err(|_| BridgeError::Validation)?;
    if value.trim().is_empty() {
        return Err(BridgeError::Validation);
    }
    Ok(Secret(value))
}
fn absolute(path: &Path) -> Result<PathBuf, BridgeError> {
    if path.as_os_str().is_empty()
        || path.to_string_lossy().contains("://")
        || path.components().any(|p| matches!(p, Component::ParentDir))
    {
        return Err(BridgeError::Validation);
    }
    if path.is_absolute() {
        Ok(path.to_owned())
    } else {
        Ok(std::env::current_dir()
            .map_err(|_| BridgeError::Storage)?
            .join(path))
    }
}
fn no_symlinks(path: &Path) -> Result<(), BridgeError> {
    for ancestor in path.ancestors() {
        match fs::symlink_metadata(ancestor) {
            Ok(m) if m.file_type().is_symlink() => return Err(BridgeError::Forbidden),
            Ok(_) => {}
            Err(e) if e.kind() == io::ErrorKind::NotFound => {}
            Err(_) => return Err(BridgeError::Storage),
        }
    }
    Ok(())
}
fn private_metadata(path: &Path, directory: bool) -> Result<fs::Metadata, BridgeError> {
    let m = fs::symlink_metadata(path).map_err(|_| BridgeError::Storage)?;
    if (directory && !m.is_dir())
        || (!directory && !m.is_file())
        || m.file_type().is_symlink()
        || m.permissions().mode() & 0o777 != if directory { 0o700 } else { 0o600 }
        || (!directory && m.nlink() != 1)
    {
        return Err(BridgeError::Forbidden);
    }
    Ok(m)
}
pub(crate) fn read_private(path: &Path, limit: u64) -> Result<Vec<u8>, BridgeError> {
    let path = absolute(path)?;
    no_symlinks(&path)?;
    private_metadata(path.parent().ok_or(BridgeError::Validation)?, true)?;
    let before = private_metadata(&path, false)?;
    let file = File::open(&path).map_err(|_| BridgeError::Storage)?;
    let after = file.metadata().map_err(|_| BridgeError::Storage)?;
    if before.dev() != after.dev() || before.ino() != after.ino() {
        return Err(BridgeError::Forbidden);
    }
    let mut bytes = Vec::new();
    file.take(limit + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| BridgeError::Storage)?;
    if bytes.len() as u64 > limit {
        return Err(BridgeError::Validation);
    }
    Ok(bytes)
}
fn private_directory(path: &Path) -> Result<(), BridgeError> {
    no_symlinks(path)?;
    if !path.exists() {
        let parent = path.parent().ok_or(BridgeError::Validation)?;
        if !parent.exists() {
            private_directory(parent)?;
        }
        DirBuilder::new()
            .mode(0o700)
            .create(path)
            .map_err(|_| BridgeError::Storage)?;
        File::open(parent)
            .and_then(|d| d.sync_all())
            .map_err(|_| BridgeError::Storage)?;
    }
    private_metadata(path, true)?;
    Ok(())
}
static NEXT: AtomicU64 = AtomicU64::new(0);
pub(crate) fn write_private_atomic(path: &Path, bytes: &[u8]) -> Result<(), BridgeError> {
    replace_with(path, |f| f.write_all(bytes))
}
fn replace_with(
    path: &Path,
    write: impl FnOnce(&mut File) -> io::Result<()>,
) -> Result<(), BridgeError> {
    let path = absolute(path)?;
    no_symlinks(&path)?;
    let parent = path.parent().ok_or(BridgeError::Validation)?;
    private_directory(parent)?;
    if fs::symlink_metadata(&path).is_ok() {
        private_metadata(&path, false)?;
    }
    let mut created = None;
    for _ in 0..64 {
        let temp = parent.join(format!(
            ".config-{}-{}.tmp",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        match OpenOptions::new()
            .write(true)
            .create_new(true)
            .mode(0o600)
            .open(&temp)
        {
            Ok(file) => {
                created = Some((temp, file));
                break;
            }
            Err(e) if e.kind() == io::ErrorKind::AlreadyExists => continue,
            Err(_) => return Err(BridgeError::Storage),
        }
    }
    let (temp, mut file) = created.ok_or(BridgeError::Storage)?;
    let result = (|| {
        write(&mut file).map_err(|_| BridgeError::Storage)?;
        file.sync_all().map_err(|_| BridgeError::Storage)?;
        no_symlinks(&path)?;
        if fs::symlink_metadata(&path).is_ok() {
            private_metadata(&path, false)?;
        }
        fs::rename(&temp, &path).map_err(|_| BridgeError::Storage)?;
        // A failure here means the replacement is visible but crash durability is unknown.
        File::open(parent)
            .and_then(|d| d.sync_all())
            .map_err(|_| BridgeError::Storage)
    })();
    if result.is_err() {
        let _ = fs::remove_file(temp);
    }
    result
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn partial_write_failure_keeps_previous_file_and_removes_temporary() {
        let dir = std::env::temp_dir().join(format!("p07-failure-{}", std::process::id()));
        DirBuilder::new().mode(0o700).create(&dir).unwrap();
        let dir = dir.canonicalize().unwrap();
        let path = dir.join("config.json");
        write_private_atomic(&path, b"previous").unwrap();
        let result = replace_with(&path, |f| {
            f.write_all(b"partial")?;
            Err(io::Error::from(io::ErrorKind::StorageFull))
        });
        assert_eq!(result, Err(BridgeError::Storage));
        assert_eq!(fs::read(&path).unwrap(), b"previous");
        assert_eq!(fs::read_dir(&dir).unwrap().count(), 1);
        fs::remove_dir_all(dir).unwrap();
    }
    #[test]
    fn failed_rename_keeps_previous_file() {
        let dir = std::env::temp_dir().join(format!("p07-rename-{}", std::process::id()));
        DirBuilder::new().mode(0o700).create(&dir).unwrap();
        let dir = dir.canonicalize().unwrap();
        let path = dir.join("config.json");
        write_private_atomic(&path, b"previous").unwrap();
        let result = replace_with(&path, |f| {
            f.write_all(b"replacement")?;
            let temp = fs::read_dir(&dir)?
                .map(|e| e.unwrap().path())
                .find(|p| p != &path)
                .unwrap();
            fs::remove_file(temp)?;
            Ok(())
        });
        assert_eq!(result, Err(BridgeError::Storage));
        assert_eq!(fs::read(&path).unwrap(), b"previous");
        assert_eq!(fs::read_dir(&dir).unwrap().count(), 1);
        fs::remove_dir_all(dir).unwrap();
    }
}
