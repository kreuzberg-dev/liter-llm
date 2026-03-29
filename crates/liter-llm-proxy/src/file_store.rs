use std::str::FromStr;

use bytes::Bytes;
use opendal::Operator;

use crate::config::FileStorageConfig;

/// OpenDAL-backed file storage for the proxy server.
///
/// Supports any OpenDAL backend (memory, S3, GCS, local filesystem, etc.)
/// through the `FileStorageConfig` backend configuration.
pub struct FileStore {
    operator: Operator,
    prefix: String,
}

impl FileStore {
    /// Build a `FileStore` from proxy file storage configuration.
    ///
    /// Parses the backend scheme from `config.backend`, builds an OpenDAL
    /// operator with the provided `backend_config`, and stores `config.prefix`
    /// for path prefixing.
    ///
    /// # Errors
    ///
    /// Returns an error string if the scheme is unknown or the operator
    /// cannot be constructed.
    pub fn from_config(config: &FileStorageConfig) -> Result<Self, String> {
        let scheme = opendal::Scheme::from_str(&config.backend)
            .map_err(|e| format!("unknown storage backend '{}': {e}", config.backend))?;

        let operator = Operator::via_iter(scheme, config.backend_config.clone())
            .map_err(|e| format!("failed to build storage operator for '{}': {e}", config.backend))?;

        Ok(Self {
            operator,
            prefix: config.prefix.clone(),
        })
    }

    /// Resolve the full path by prepending the prefix to the key.
    fn full_path(&self, key: &str) -> String {
        format!("{}{key}", self.prefix)
    }

    /// Write data to the store under the given key.
    ///
    /// # Errors
    ///
    /// Returns an error string if the write operation fails.
    pub async fn write(&self, key: &str, data: Bytes) -> Result<(), String> {
        let path = self.full_path(key);
        self.operator
            .write(&path, data)
            .await
            .map(|_| ())
            .map_err(|e| format!("failed to write '{path}': {e}"))
    }

    /// Read data from the store for the given key.
    ///
    /// # Errors
    ///
    /// Returns an error string if the key does not exist or the read fails.
    pub async fn read(&self, key: &str) -> Result<Bytes, String> {
        let path = self.full_path(key);
        let buf = self
            .operator
            .read(&path)
            .await
            .map_err(|e| format!("failed to read '{path}': {e}"))?;
        Ok(buf.to_bytes())
    }

    /// Delete a key from the store.
    ///
    /// # Errors
    ///
    /// Returns an error string if the delete operation fails.
    pub async fn delete(&self, key: &str) -> Result<(), String> {
        let path = self.full_path(key);
        self.operator
            .delete(&path)
            .await
            .map_err(|e| format!("failed to delete '{path}': {e}"))
    }

    /// List keys under an optional prefix (relative to the store prefix).
    ///
    /// Returns the full key paths (without the store prefix).
    ///
    /// # Errors
    ///
    /// Returns an error string if the list operation fails.
    pub async fn list(&self, prefix: Option<&str>) -> Result<Vec<String>, String> {
        let scan_prefix = match prefix {
            Some(p) => format!("{}{p}", self.prefix),
            None => self.prefix.clone(),
        };
        let entries = self
            .operator
            .list(&scan_prefix)
            .await
            .map_err(|e| format!("failed to list '{scan_prefix}': {e}"))?;

        let store_prefix_len = self.prefix.len();
        let keys: Vec<String> = entries
            .into_iter()
            .filter(|entry| !entry.path().ends_with('/'))
            .filter_map(|entry| {
                let path = entry.path();
                if path.len() > store_prefix_len {
                    Some(path[store_prefix_len..].to_string())
                } else {
                    None
                }
            })
            .collect();

        Ok(keys)
    }

    /// Check whether a key exists in the store.
    ///
    /// # Errors
    ///
    /// Returns an error string if the existence check fails.
    pub async fn exists(&self, key: &str) -> Result<bool, String> {
        let path = self.full_path(key);
        self.operator
            .exists(&path)
            .await
            .map_err(|e| format!("failed to check existence of '{path}': {e}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::FileStorageConfig;

    fn memory_store() -> FileStore {
        let config = FileStorageConfig::default();
        FileStore::from_config(&config).expect("memory backend should build")
    }

    #[tokio::test]
    async fn write_then_read_returns_same_data() {
        let store = memory_store();
        let data = Bytes::from_static(b"hello world");
        store
            .write("test.txt", data.clone())
            .await
            .expect("write should succeed");
        let read_data = store.read("test.txt").await.expect("read should succeed");
        assert_eq!(read_data, data);
    }

    #[tokio::test]
    async fn read_nonexistent_key_returns_error() {
        let store = memory_store();
        let result = store.read("does-not-exist.txt").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn delete_then_exists_returns_false() {
        let store = memory_store();
        store
            .write("to-delete.txt", Bytes::from_static(b"data"))
            .await
            .expect("write should succeed");
        assert!(
            store.exists("to-delete.txt").await.expect("exists check"),
            "key should exist after write"
        );
        store.delete("to-delete.txt").await.expect("delete should succeed");
        assert!(
            !store.exists("to-delete.txt").await.expect("exists check"),
            "key should not exist after delete"
        );
    }

    #[tokio::test]
    async fn list_returns_written_keys() {
        let store = memory_store();
        store.write("a.txt", Bytes::from_static(b"aaa")).await.expect("write a");
        store.write("b.txt", Bytes::from_static(b"bbb")).await.expect("write b");

        let mut keys = store.list(None).await.expect("list should succeed");
        keys.sort();
        assert_eq!(keys, vec!["a.txt", "b.txt"]);
    }

    #[tokio::test]
    async fn exists_returns_false_for_missing_key() {
        let store = memory_store();
        let result = store.exists("nope.txt").await.expect("exists check");
        assert!(!result);
    }

    #[tokio::test]
    async fn exists_returns_true_after_write() {
        let store = memory_store();
        store
            .write("present.txt", Bytes::from_static(b"here"))
            .await
            .expect("write");
        let result = store.exists("present.txt").await.expect("exists check");
        assert!(result);
    }

    #[tokio::test]
    async fn overwrite_replaces_data() {
        let store = memory_store();
        store
            .write("file.txt", Bytes::from_static(b"original"))
            .await
            .expect("write");
        store
            .write("file.txt", Bytes::from_static(b"replaced"))
            .await
            .expect("overwrite");
        let data = store.read("file.txt").await.expect("read");
        assert_eq!(data, Bytes::from_static(b"replaced"));
    }

    #[test]
    fn from_config_rejects_unknown_backend() {
        let config = FileStorageConfig {
            backend: "nonexistent_xyz".to_string(),
            prefix: "test/".to_string(),
            backend_config: std::collections::HashMap::new(),
        };
        let result = FileStore::from_config(&config);
        assert!(result.is_err());
    }
}
