//! REST API for cassette management
//!
//! Provides HTTP endpoints to list, inspect, validate, and delete cassettes.

use crate::cassette::Cassette;
use crate::error::{MatgtoError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use chrono::{DateTime, Utc};

/// Cassette metadata (lightweight, no full interactions)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CassetteMetadata {
    /// Cassette name (without extension)
    pub name: String,

    /// Full file path
    pub path: PathBuf,

    /// File size in bytes
    pub size_bytes: u64,

    /// File size human-readable
    pub size_human: String,

    /// Cassette format version
    pub version: String,

    /// Recording timestamp
    pub recorded_at: DateTime<Utc>,

    /// Number of interactions
    pub interaction_count: usize,

    /// Age in days
    pub age_days: i64,

    /// Cassette format (json, msgpack)
    pub format: String,
}

/// Cassette detailed statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CassetteStats {
    /// Total interactions
    pub total_interactions: usize,

    /// HTTP interactions
    pub http_count: usize,

    /// WebSocket interactions
    pub websocket_count: usize,

    /// HTTP errors (timeouts, DNS failures, etc.)
    pub http_error_count: usize,

    /// Status code distribution
    pub status_codes: HashMap<u16, usize>,

    /// HTTP method distribution
    pub http_methods: HashMap<String, usize>,

    /// Total request body size
    pub total_request_body_bytes: usize,

    /// Total response body size
    pub total_response_body_bytes: usize,

    /// Average response time (if available)
    pub avg_response_time_ms: Option<f64>,
}

/// Validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    /// Is cassette valid?
    pub valid: bool,

    /// Errors found
    pub errors: Vec<String>,

    /// Warnings
    pub warnings: Vec<String>,
}

/// Cassette manager for API operations
pub struct CassetteManager {
    /// Directory where cassettes are stored
    cassette_dir: PathBuf,

    /// Cache of metadata (optional optimization)
    #[allow(dead_code)]
    cache: Option<HashMap<String, CassetteMetadata>>,
}

impl CassetteManager {
    /// Create new cassette manager
    pub fn new(cassette_dir: impl Into<PathBuf>) -> Self {
        Self {
            cassette_dir: cassette_dir.into(),
            cache: None,
        }
    }

    /// List all cassettes in directory
    pub fn list_cassettes(&self) -> Result<Vec<CassetteMetadata>> {
        let mut cassettes = Vec::new();

        if !self.cassette_dir.exists() {
            return Ok(cassettes);
        }

        for entry in fs::read_dir(&self.cassette_dir)? {
            let entry = entry?;
            let path = entry.path();

            // Only process .json and .msgpack files
            if let Some(ext) = path.extension() {
                if ext == "json" || ext == "msgpack" {
                    if let Ok(metadata) = self.get_cassette_metadata(&path) {
                        cassettes.push(metadata);
                    }
                }
            }
        }

        // Sort by name
        cassettes.sort_by(|a, b| a.name.cmp(&b.name));

        Ok(cassettes)
    }

    /// Get metadata for a single cassette
    pub fn get_cassette_metadata(&self, path: &Path) -> Result<CassetteMetadata> {
        let file_metadata = fs::metadata(path)?;
        let size_bytes = file_metadata.len();

        // Load cassette to get internal metadata
        let file = std::fs::File::open(path)?;
        let cassette: Cassette = serde_json::from_reader(file)?;

        let name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();

        let format = path
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();

        let duration = Utc::now() - cassette.recorded_at;
        let age_days = duration.num_days();

        Ok(CassetteMetadata {
            name,
            path: path.to_path_buf(),
            size_bytes,
            size_human: format_bytes(size_bytes),
            version: cassette.version,
            recorded_at: cassette.recorded_at,
            interaction_count: cassette.interactions.len(),
            age_days,
            format,
        })
    }

    /// Get detailed statistics for a cassette
    pub fn get_cassette_stats(&self, name: &str) -> Result<CassetteStats> {
        let cassette = self.load_cassette(name)?;

        let mut stats = CassetteStats {
            total_interactions: cassette.interactions.len(),
            http_count: 0,
            websocket_count: 0,
            http_error_count: 0,
            status_codes: HashMap::new(),
            http_methods: HashMap::new(),
            total_request_body_bytes: 0,
            total_response_body_bytes: 0,
            avg_response_time_ms: None,
        };

        let mut response_times = Vec::new();

        for interaction in &cassette.interactions {
            match &interaction.kind {
                crate::cassette::InteractionKind::Http { request, response } => {
                    stats.http_count += 1;

                    // Status codes
                    *stats.status_codes.entry(response.status).or_insert(0) += 1;

                    // HTTP methods
                    *stats.http_methods.entry(request.method.clone()).or_insert(0) += 1;

                    // Body sizes
                    if let Some(body) = &request.body {
                        stats.total_request_body_bytes += body.len();
                    }
                    if let Some(body) = &response.body {
                        stats.total_response_body_bytes += body.len();
                    }
                }
                crate::cassette::InteractionKind::HttpError { .. } => {
                    stats.http_error_count += 1;
                }
                crate::cassette::InteractionKind::WebSocket { .. } => {
                    stats.websocket_count += 1;
                }
            }

            // Response times
            if let Some(time_ms) = interaction.response_time_ms {
                response_times.push(time_ms as f64);
            }
        }

        // Calculate average response time
        if !response_times.is_empty() {
            let avg: f64 = response_times.iter().sum::<f64>() / response_times.len() as f64;
            stats.avg_response_time_ms = Some(avg);
        }

        Ok(stats)
    }

    /// Validate a cassette
    pub fn validate_cassette(&self, name: &str) -> Result<ValidationResult> {
        let mut result = ValidationResult {
            valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
        };

        // Try to load cassette
        let cassette = match self.load_cassette(name) {
            Ok(c) => c,
            Err(e) => {
                result.valid = false;
                result.errors.push(format!("Failed to load cassette: {}", e));
                return Ok(result);
            }
        };

        // Check version
        if cassette.version != "1.0" && cassette.version != "2.0" {
            result.warnings.push(format!(
                "Unknown cassette version: {} (expected 1.0 or 2.0)",
                cassette.version
            ));
        }

        if cassette.version == "1.0" {
            result.warnings.push("Cassette is v1.0, consider migrating to v2.0 for cookie support".to_string());
        }

        // Check age
        let duration = Utc::now() - cassette.recorded_at;
        let age_days = duration.num_days();
        if age_days > 90 {
            result.warnings.push(format!(
                "Cassette is {} days old (>90 days), consider re-recording",
                age_days
            ));
        }

        // Check size
        if let Ok(metadata) = self.get_cassette_metadata(&self.cassette_path(name)?) {
            if metadata.size_bytes > 10 * 1024 * 1024 {
                // >10 MB
                result.warnings.push(format!(
                    "Cassette is large ({}) - consider enabling filtering",
                    metadata.size_human
                ));
            }
        }

        // Validate interactions
        for (i, interaction) in cassette.interactions.iter().enumerate() {
            match &interaction.kind {
                crate::cassette::InteractionKind::Http { request, response } => {
                    // Check URLs
                    if request.url.is_empty() {
                        result.errors.push(format!("Interaction {}: empty URL", i));
                        result.valid = false;
                    }

                    // Check status codes
                    if response.status < 100 || response.status > 599 {
                        result.errors.push(format!(
                            "Interaction {}: invalid status code {}",
                            i, response.status
                        ));
                        result.valid = false;
                    }
                }
                crate::cassette::InteractionKind::HttpError { request, .. } => {
                    if request.url.is_empty() {
                        result.errors.push(format!("Interaction {}: empty URL in error", i));
                        result.valid = false;
                    }
                }
                crate::cassette::InteractionKind::WebSocket { url, .. } => {
                    if url.is_empty() {
                        result.errors.push(format!("Interaction {}: empty WebSocket URL", i));
                        result.valid = false;
                    }
                }
            }
        }

        Ok(result)
    }

    /// Delete a cassette
    pub fn delete_cassette(&self, name: &str) -> Result<()> {
        let path = self.cassette_path(name)?;

        if !path.exists() {
            return Err(MatgtoError::CassetteNotFound {
                name: name.to_string(),
            });
        }

        fs::remove_file(path)?;

        Ok(())
    }

    /// Load a cassette by name
    pub fn load_cassette(&self, name: &str) -> Result<Cassette> {
        let path = self.cassette_path(name)?;
        let file = std::fs::File::open(&path)
            .map_err(|e| MatgtoError::Io(e))?;
        let cassette: Cassette = serde_json::from_reader(file)
            .map_err(|e| MatgtoError::Serialization(e))?;
        Ok(cassette)
    }

    /// Get cassette file path
    fn cassette_path(&self, name: &str) -> Result<PathBuf> {
        // Try .json first
        let json_path = self.cassette_dir.join(format!("{}.json", name));
        if json_path.exists() {
            return Ok(json_path);
        }

        // Try .msgpack
        let msgpack_path = self.cassette_dir.join(format!("{}.msgpack", name));
        if msgpack_path.exists() {
            return Ok(msgpack_path);
        }

        Err(MatgtoError::CassetteNotFound {
            name: name.to_string(),
        })
    }

    /// Get global statistics
    pub fn global_stats(&self) -> Result<GlobalStats> {
        let cassettes = self.list_cassettes()?;

        let total_count = cassettes.len();
        let total_size_bytes: u64 = cassettes.iter().map(|c| c.size_bytes).sum();

        let oldest = cassettes
            .iter()
            .min_by_key(|c| c.recorded_at)
            .map(|c| c.clone());

        let largest = cassettes
            .iter()
            .max_by_key(|c| c.size_bytes)
            .map(|c| c.clone());

        // Size distribution
        let mut size_dist = SizeDistribution::default();
        for cassette in &cassettes {
            let mb = cassette.size_bytes as f64 / (1024.0 * 1024.0);
            if mb < 1.0 {
                size_dist.under_1mb += 1;
            } else if mb < 10.0 {
                size_dist.mb_1_to_10 += 1;
            } else {
                size_dist.over_10mb += 1;
            }
        }

        // Age distribution
        let mut age_dist = AgeDistribution::default();
        for cassette in &cassettes {
            if cassette.age_days < 7 {
                age_dist.under_7_days += 1;
            } else if cassette.age_days < 30 {
                age_dist.days_7_to_30 += 1;
            } else {
                age_dist.over_30_days += 1;
            }
        }

        Ok(GlobalStats {
            total_count,
            total_size_bytes,
            total_size_human: format_bytes(total_size_bytes),
            oldest_cassette: oldest,
            largest_cassette: largest,
            size_distribution: size_dist,
            age_distribution: age_dist,
        })
    }
}

/// Global cassette statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalStats {
    pub total_count: usize,
    pub total_size_bytes: u64,
    pub total_size_human: String,
    pub oldest_cassette: Option<CassetteMetadata>,
    pub largest_cassette: Option<CassetteMetadata>,
    pub size_distribution: SizeDistribution,
    pub age_distribution: AgeDistribution,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SizeDistribution {
    pub under_1mb: usize,
    pub mb_1_to_10: usize,
    pub over_10mb: usize,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AgeDistribution {
    pub under_7_days: usize,
    pub days_7_to_30: usize,
    pub over_30_days: usize,
}

/// Format bytes to human-readable string
fn format_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} bytes", bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(500), "500 bytes");
        assert_eq!(format_bytes(1024), "1.00 KB");
        assert_eq!(format_bytes(1536), "1.50 KB");
        assert_eq!(format_bytes(1024 * 1024), "1.00 MB");
        assert_eq!(format_bytes(5 * 1024 * 1024), "5.00 MB");
    }
}
