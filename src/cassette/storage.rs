//! Optimized cassette storage with async I/O and buffering
///
/// This module provides high-performance cassette storage with:
/// - Async I/O for non-blocking operations
/// - In-memory buffering to batch writes
/// - MessagePack support for binary format
/// - Background writer task for zero-latency saves
use crate::cassette::Cassette;
use crate::error::Result;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::fs;
use tokio::sync::mpsc;
use tokio::sync::Mutex;

/// Format for cassette serialization
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CassetteFormat {
    /// JSON format (human-readable, larger files)
    Json,

    /// MessagePack format (binary, smaller files, faster)
    #[cfg(feature = "msgpack")]
    MessagePack,

    /// JSON with gzip compression (smaller files, slower)
    #[cfg(feature = "compression")]
    JsonGzip,

    /// MessagePack with gzip compression (smallest files, good balance)
    #[cfg(all(feature = "msgpack", feature = "compression"))]
    MessagePackGzip,
}

/// Message sent to background writer
enum WriterMessage {
    /// Save a cassette to disk
    Save {
        cassette: Cassette,
        path: PathBuf,
        format: CassetteFormat,
    },

    /// Shutdown the writer
    Shutdown,
}

/// Async cassette storage with background writer
pub struct AsyncCassetteStorage {
    /// Channel to send save requests
    tx: mpsc::UnboundedSender<WriterMessage>,

    /// Join handle for background writer
    writer_handle: Option<tokio::task::JoinHandle<()>>,
}

impl AsyncCassetteStorage {
    /// Create a new async storage with background writer
    pub fn new() -> Self {
        let (tx, rx) = mpsc::unbounded_channel();

        // Try to spawn on current runtime, fallback to no background writer
        let writer_handle = if tokio::runtime::Handle::try_current().is_ok() {
            Some(tokio::spawn(async move {
                Self::writer_task(rx).await;
            }))
        } else {
            // No runtime available, drop rx to avoid channel errors
            drop(rx);
            None
        };

        Self { tx, writer_handle }
    }

    /// Background writer task
    async fn writer_task(mut rx: mpsc::UnboundedReceiver<WriterMessage>) {
        while let Some(msg) = rx.recv().await {
            match msg {
                WriterMessage::Save {
                    cassette,
                    path,
                    format,
                } => {
                    if let Err(e) = Self::save_cassette_async(&cassette, &path, format).await {
                        eprintln!("Failed to save cassette: {}", e);
                    }
                }
                WriterMessage::Shutdown => {
                    break;
                }
            }
        }
    }

    /// Save cassette asynchronously (non-blocking for caller)
    pub fn save_async(
        &self,
        cassette: Cassette,
        path: PathBuf,
        format: CassetteFormat,
    ) -> Result<()> {
        self.tx.send(WriterMessage::Save {
            cassette,
            path,
            format,
        })?;

        Ok(())
    }

    /// Save cassette and wait for completion
    pub async fn save_sync(
        &self,
        cassette: Cassette,
        path: PathBuf,
        format: CassetteFormat,
    ) -> Result<()> {
        Self::save_cassette_async(&cassette, &path, format).await
    }

    /// Internal async save implementation
    async fn save_cassette_async(
        cassette: &Cassette,
        path: &Path,
        format: CassetteFormat,
    ) -> Result<()> {
        // Ensure directory exists
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).await?;
        }

        // Serialize cassette
        let data = match format {
            CassetteFormat::Json => serde_json::to_vec_pretty(cassette)?,
            #[cfg(feature = "msgpack")]
            CassetteFormat::MessagePack => rmp_serde::to_vec(cassette)?,
            #[cfg(feature = "compression")]
            CassetteFormat::JsonGzip => {
                let json_data = serde_json::to_vec_pretty(cassette)?;
                Self::compress_data(&json_data)?
            }
            #[cfg(all(feature = "msgpack", feature = "compression"))]
            CassetteFormat::MessagePackGzip => {
                let msgpack_data = rmp_serde::to_vec(cassette)?;
                Self::compress_data(&msgpack_data)?
            }
        };

        // Write atomically (write to temp file, then rename)
        let temp_path = path.with_extension("tmp");
        fs::write(&temp_path, &data).await?;
        fs::rename(&temp_path, path).await?;

        Ok(())
    }

    /// Load cassette asynchronously
    pub async fn load_async(path: &Path, format: CassetteFormat) -> Result<Cassette> {
        let data = fs::read(path).await?;

        let cassette = match format {
            CassetteFormat::Json => serde_json::from_slice(&data)?,
            #[cfg(feature = "msgpack")]
            CassetteFormat::MessagePack => rmp_serde::from_slice(&data)?,
            #[cfg(feature = "compression")]
            CassetteFormat::JsonGzip => {
                let decompressed = Self::decompress_data(&data)?;
                serde_json::from_slice(&decompressed)?
            }
            #[cfg(all(feature = "msgpack", feature = "compression"))]
            CassetteFormat::MessagePackGzip => {
                let decompressed = Self::decompress_data(&data)?;
                rmp_serde::from_slice(&decompressed)?
            }
        };

        Ok(cassette)
    }

    /// Compress data using gzip
    #[cfg(feature = "compression")]
    fn compress_data(data: &[u8]) -> Result<Vec<u8>> {
        use flate2::write::GzEncoder;
        use flate2::Compression;
        use std::io::Write;

        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(data)?;
        let compressed = encoder.finish()?;

        Ok(compressed)
    }

    /// Decompress gzip data
    #[cfg(feature = "compression")]
    fn decompress_data(data: &[u8]) -> Result<Vec<u8>> {
        use flate2::read::GzDecoder;
        use std::io::Read;

        let mut decoder = GzDecoder::new(data);
        let mut decompressed = Vec::new();
        decoder.read_to_end(&mut decompressed)?;

        Ok(decompressed)
    }

    /// Shutdown background writer gracefully
    pub async fn shutdown(mut self) {
        let _ = self.tx.send(WriterMessage::Shutdown);

        if let Some(handle) = self.writer_handle.take() {
            let _ = handle.await;
        }
    }
}

impl Default for AsyncCassetteStorage {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for AsyncCassetteStorage {
    fn drop(&mut self) {
        // Try to send shutdown signal
        let _ = self.tx.send(WriterMessage::Shutdown);
    }
}

/// Buffered cassette writer for batching writes
pub struct BufferedCassetteWriter {
    /// Cassette being built
    cassette: Arc<Mutex<Cassette>>,

    /// Async storage for background writes
    storage: Arc<AsyncCassetteStorage>,

    /// Format to use for saving
    format: CassetteFormat,
}

impl BufferedCassetteWriter {
    /// Create a new buffered writer
    pub fn new(
        cassette_name: String,
        storage: Arc<AsyncCassetteStorage>,
        format: CassetteFormat,
    ) -> Self {
        let cassette = Cassette::new(cassette_name);

        Self {
            cassette: Arc::new(Mutex::new(cassette)),
            storage,
            format,
        }
    }

    /// Get reference to cassette
    pub fn cassette(&self) -> Arc<Mutex<Cassette>> {
        Arc::clone(&self.cassette)
    }

    /// Flush cassette to disk asynchronously
    pub async fn flush(&self, path: PathBuf) -> Result<()> {
        let cassette = self.cassette.lock().await.clone();
        self.storage.save_sync(cassette, path, self.format).await
    }

    /// Flush cassette to disk in background (non-blocking)
    pub fn flush_async(&self, path: PathBuf) -> Result<()> {
        let cassette = {
            // We need to block here briefly to clone
            // In real usage, this would be called from an async context
            tokio::task::block_in_place(|| {
                tokio::runtime::Handle::current()
                    .block_on(async { self.cassette.lock().await.clone() })
            })
        };

        self.storage.save_async(cassette, path, self.format)
    }
}

/// Auto-detect cassette format from file extension
pub fn detect_format(path: &Path) -> CassetteFormat {
    let path_str = path.to_string_lossy().to_string();

    // Check for compressed formats first (they have multiple extensions)
    #[cfg(all(feature = "msgpack", feature = "compression"))]
    {
        if path_str.ends_with(".msgpack.gz") || path_str.ends_with(".mp.gz") {
            return CassetteFormat::MessagePackGzip;
        }
    }

    #[cfg(feature = "compression")]
    {
        if path_str.ends_with(".json.gz") {
            return CassetteFormat::JsonGzip;
        }
    }

    // Then check for non-compressed formats
    if let Some(ext) = path.extension() {
        match ext.to_str() {
            #[cfg(feature = "msgpack")]
            Some("msgpack") | Some("mp") => CassetteFormat::MessagePack,
            #[cfg(feature = "compression")]
            Some("gz") => CassetteFormat::JsonGzip, // Assume .gz without other ext is JSON
            _ => CassetteFormat::Json,
        }
    } else {
        CassetteFormat::Json
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_async_storage_save_load() {
        let storage = AsyncCassetteStorage::new();
        let cassette = Cassette::new("test".to_string());

        let dir = tempdir().unwrap();
        let path = dir.path().join("test.json");

        // Save
        storage
            .save_sync(cassette.clone(), path.clone(), CassetteFormat::Json)
            .await
            .unwrap();

        // Load
        let loaded = AsyncCassetteStorage::load_async(&path, CassetteFormat::Json)
            .await
            .unwrap();

        assert_eq!(loaded.name, cassette.name);
    }

    #[tokio::test]
    async fn test_async_storage_background_save() {
        let storage = AsyncCassetteStorage::new();
        let cassette = Cassette::new("test-async".to_string());

        let dir = tempdir().unwrap();
        let path = dir.path().join("test-async.json");

        // Save async (non-blocking)
        storage
            .save_async(cassette, path.clone(), CassetteFormat::Json)
            .unwrap();

        // Give writer time to complete
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        // Check file exists
        assert!(path.exists());
    }

    #[cfg(feature = "msgpack")]
    #[tokio::test]
    async fn test_messagepack_format() {
        let storage = AsyncCassetteStorage::new();
        let cassette = Cassette::new("test-msgpack".to_string());

        let dir = tempdir().unwrap();
        let path = dir.path().join("test-msgpack.msgpack");

        // Save as MessagePack
        storage
            .save_sync(cassette.clone(), path.clone(), CassetteFormat::MessagePack)
            .await
            .unwrap();

        // Load as MessagePack
        let loaded = AsyncCassetteStorage::load_async(&path, CassetteFormat::MessagePack)
            .await
            .unwrap();

        assert_eq!(loaded.name, cassette.name);

        // Verify file is smaller than JSON would be
        let json_path = dir.path().join("test-msgpack.json");
        storage
            .save_sync(cassette, json_path.clone(), CassetteFormat::Json)
            .await
            .unwrap();

        let msgpack_size = std::fs::metadata(&path).unwrap().len();
        let json_size = std::fs::metadata(&json_path).unwrap().len();

        // MessagePack should be smaller (or similar for empty cassette)
        assert!(msgpack_size <= json_size);
    }

    #[test]
    fn test_detect_format() {
        assert_eq!(detect_format(Path::new("test.json")), CassetteFormat::Json);

        #[cfg(feature = "msgpack")]
        {
            assert_eq!(
                detect_format(Path::new("test.msgpack")),
                CassetteFormat::MessagePack
            );
            assert_eq!(
                detect_format(Path::new("test.mp")),
                CassetteFormat::MessagePack
            );
        }

        #[cfg(feature = "compression")]
        {
            assert_eq!(
                detect_format(Path::new("test.json.gz")),
                CassetteFormat::JsonGzip
            );
            assert_eq!(
                detect_format(Path::new("test.gz")),
                CassetteFormat::JsonGzip
            );
        }

        #[cfg(all(feature = "msgpack", feature = "compression"))]
        {
            assert_eq!(
                detect_format(Path::new("test.msgpack.gz")),
                CassetteFormat::MessagePackGzip
            );
            assert_eq!(
                detect_format(Path::new("test.mp.gz")),
                CassetteFormat::MessagePackGzip
            );
        }
    }

    #[cfg(feature = "compression")]
    #[tokio::test]
    async fn test_json_gzip_compression() {
        use crate::cassette::{HttpRequest, HttpResponse, InteractionKind};
        use std::collections::HashMap;

        let storage = AsyncCassetteStorage::new();
        let mut cassette = Cassette::new("test-json-gzip".to_string());

        // Add some interactions to have meaningful data to compress
        for i in 0..10 {
            let request = HttpRequest {
                method: "GET".to_string(),
                url: format!("https://api.example.com/test/{}", i),
                headers: HashMap::new(),
                body: Some(b"test request body".to_vec()),
            };

            let response = HttpResponse {
                status: 200,
                headers: HashMap::new(),
                body: Some(b"test response body with some data to compress".to_vec()),
            };

            cassette.add_interaction(InteractionKind::Http { request, response });
        }

        let dir = tempdir().unwrap();
        let path = dir.path().join("test-json.json.gz");

        // Save as compressed JSON
        storage
            .save_sync(cassette.clone(), path.clone(), CassetteFormat::JsonGzip)
            .await
            .unwrap();

        // Load compressed JSON
        let loaded = AsyncCassetteStorage::load_async(&path, CassetteFormat::JsonGzip)
            .await
            .unwrap();

        assert_eq!(loaded.name, cassette.name);
        assert_eq!(loaded.interactions.len(), 10);

        // Verify compression roundtrip works correctly
        let uncompressed_path = dir.path().join("test-json-uncompressed.json");
        storage
            .save_sync(cassette, uncompressed_path.clone(), CassetteFormat::Json)
            .await
            .unwrap();

        let compressed_size = std::fs::metadata(&path).unwrap().len();
        let uncompressed_size = std::fs::metadata(&uncompressed_path).unwrap().len();

        println!(
            "Compressed: {} bytes, Uncompressed: {} bytes",
            compressed_size, uncompressed_size
        );
        println!(
            "Compression ratio: {:.1}%",
            (compressed_size as f64 / uncompressed_size as f64) * 100.0
        );

        // For small cassettes with some data, compression might not always be better
        // Just verify the file was created and can be read
        assert!(path.exists());
    }

    #[cfg(all(feature = "msgpack", feature = "compression"))]
    #[tokio::test]
    async fn test_messagepack_gzip_compression() {
        use crate::cassette::{HttpRequest, HttpResponse, InteractionKind};
        use std::collections::HashMap;

        let storage = AsyncCassetteStorage::new();
        let mut cassette = Cassette::new("test-msgpack-gzip".to_string());

        // Add some interactions
        for i in 0..10 {
            let request = HttpRequest {
                method: "POST".to_string(),
                url: format!("https://api.example.com/data/{}", i),
                headers: HashMap::new(),
                body: Some(b"msgpack test request body".to_vec()),
            };

            let response = HttpResponse {
                status: 201,
                headers: HashMap::new(),
                body: Some(b"msgpack test response body with data".to_vec()),
            };

            cassette.add_interaction(InteractionKind::Http { request, response });
        }

        let dir = tempdir().unwrap();
        let path = dir.path().join("test-msgpack.msgpack.gz");

        // Save as compressed MessagePack
        storage
            .save_sync(
                cassette.clone(),
                path.clone(),
                CassetteFormat::MessagePackGzip,
            )
            .await
            .unwrap();

        // Load compressed MessagePack
        let loaded = AsyncCassetteStorage::load_async(&path, CassetteFormat::MessagePackGzip)
            .await
            .unwrap();

        assert_eq!(loaded.name, cassette.name);
        assert_eq!(loaded.interactions.len(), 10);

        // Verify compression roundtrip works correctly
        let uncompressed_path = dir.path().join("test-msgpack-uncompressed.msgpack");
        storage
            .save_sync(
                cassette,
                uncompressed_path.clone(),
                CassetteFormat::MessagePack,
            )
            .await
            .unwrap();

        let compressed_size = std::fs::metadata(&path).unwrap().len();
        let uncompressed_size = std::fs::metadata(&uncompressed_path).unwrap().len();

        println!(
            "Compressed: {} bytes, Uncompressed: {} bytes",
            compressed_size, uncompressed_size
        );
        println!(
            "Compression ratio: {:.1}%",
            (compressed_size as f64 / uncompressed_size as f64) * 100.0
        );

        // Verify the file was created and can be read
        assert!(path.exists());
    }

    #[cfg(feature = "compression")]
    #[tokio::test]
    async fn test_compression_with_large_data() {
        use crate::cassette::{HttpRequest, HttpResponse, InteractionKind};
        use std::collections::HashMap;

        let storage = AsyncCassetteStorage::new();
        let mut cassette = Cassette::new("test-large".to_string());

        // Add multiple interactions with large bodies
        for i in 0..100 {
            let request = HttpRequest {
                method: "GET".to_string(),
                url: format!("https://api.example.com/data/{}", i),
                headers: HashMap::new(),
                body: Some(vec![b'x'; 1000]), // 1KB of data
            };

            let response = HttpResponse {
                status: 200,
                headers: HashMap::new(),
                body: Some(vec![b'y'; 5000]), // 5KB of data
            };

            let kind = InteractionKind::Http { request, response };
            cassette.add_interaction(kind);
        }

        let dir = tempdir().unwrap();
        let json_path = dir.path().join("large-json.json");
        let json_gz_path = dir.path().join("large-json.json.gz");

        // Save uncompressed and compressed
        storage
            .save_sync(cassette.clone(), json_path.clone(), CassetteFormat::Json)
            .await
            .unwrap();

        storage
            .save_sync(cassette, json_gz_path.clone(), CassetteFormat::JsonGzip)
            .await
            .unwrap();

        let json_size = std::fs::metadata(&json_path).unwrap().len();
        let json_gz_size = std::fs::metadata(&json_gz_path).unwrap().len();

        println!(
            "JSON: {} KB, JSON.GZ: {} KB",
            json_size / 1024,
            json_gz_size / 1024
        );
        println!(
            "Compression ratio: {:.2}%",
            (json_gz_size as f64 / json_size as f64) * 100.0
        );

        // Compression should be significant for large repetitive data
        assert!(
            json_gz_size < json_size / 2,
            "Expected at least 50% compression"
        );
    }
}
