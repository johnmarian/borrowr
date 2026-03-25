/// Creates the parent directory of the given path if it does not already exist.
///
/// Does nothing if the path has no parent component (e.g. `"/"`).
pub fn ensure_data_dir(path: &str) -> std::io::Result<()> {
    if let Some(parent) = std::path::Path::new(path).parent() {
        std::fs::create_dir_all(parent)?;
    }
    Ok(())
}

/// Resolves the path to the SQLite database file.
///
/// # Environment variables
///
/// | Variable      | Purpose                                      | Default                  |
/// |---------------|----------------------------------------------|--------------------------|
/// | `BORROWR_DB`  | Full path to the database file               | `$HOME/.borrowr/borrowr.db` |
/// | `HOME`        | Used to construct the default path           | Must be set (always is on macOS/Linux) |
///
/// # Examples
///
/// ```
/// # use borrowr::config::db_path;
/// let path = db_path(None, "/home/alice".to_string());
/// assert_eq!(path, "/home/alice/.borrowr/borrowr.db");
///
/// // Override: use a custom path
/// let path = db_path(Some("/data/my.db".to_string()), "/home/alice".to_string());
/// assert_eq!(path, "/data/my.db");
/// ```
pub fn db_path(borrowr_db: Option<String>, home: String) -> String {
    borrowr_db.unwrap_or_else(|| format!("{}/.borrowr/borrowr.db", home))
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TempDir(std::path::PathBuf);
    impl TempDir {
        fn new(name: &str) -> Self {
            let path = std::env::temp_dir().join(name);
            let _ = std::fs::remove_dir_all(&path);
            Self(path)
        }
        fn path(&self) -> &std::path::Path {
            &self.0
        }
    }
    impl Drop for TempDir {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn ensure_data_dir_creates_missing_directory() {
        let dir = TempDir::new("borrowr_test_ensure_data_dir");
        let path = dir.path().join("subdir").join("borrowr.db");
        ensure_data_dir(path.to_str().unwrap()).unwrap();
        assert!(path.parent().unwrap().exists());
    }

    #[test]
    fn ensure_data_dir_succeeds_when_path_has_no_parent() {
        assert!(ensure_data_dir("/").is_ok());
    }

    #[test]
    fn db_path_defaults_to_home() {
        assert_eq!(
            db_path(None, "/home/alice".to_string()),
            "/home/alice/.borrowr/borrowr.db"
        );
    }

    #[test]
    fn db_path_uses_borrowr_db_when_set() {
        assert_eq!(
            db_path(
                Some("/custom/path.db".to_string()),
                "/home/alice".to_string()
            ),
            "/custom/path.db"
        );
    }
}
