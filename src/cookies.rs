//! Cookie handling for session preservation in replay mode
//!
//! This module implements RFC 6265 cookie parsing and management to support
//! session-based authentication in recorded cassettes.
//!
//! # Example
//! ```rust
//! use magneto_serge::cookies::{Cookie, CookieJar};
//!
//! let mut jar = CookieJar::new();
//!
//! // Parse Set-Cookie header
//! let cookie = Cookie::parse("session=abc123; Path=/; Secure; HttpOnly").unwrap();
//! jar.store(cookie);
//!
//! // Get cookies for URL
//! let header = jar.get_header_value("https://example.com/api").unwrap();
//! // Returns: "session=abc123"
//! ```

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

/// A single HTTP cookie with all RFC 6265 attributes
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Cookie {
    /// Cookie name
    pub name: String,

    /// Cookie value  
    pub value: String,

    /// Domain attribute (e.g., ".example.com")
    pub domain: Option<String>,

    /// Path attribute (e.g., "/api")
    pub path: Option<String>,

    /// Expires attribute (absolute time)
    pub expires: Option<DateTime<Utc>>,

    /// Max-Age attribute (relative seconds)
    pub max_age: Option<i64>,

    /// Secure flag (HTTPS only)
    pub secure: bool,

    /// HttpOnly flag (not accessible via JavaScript)
    pub http_only: bool,

    /// SameSite attribute
    pub same_site: Option<SameSite>,

    /// When this cookie was created/received
    pub created_at: DateTime<Utc>,
}

/// SameSite attribute values (RFC 6265bis)
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SameSite {
    /// Strict: cookie only sent in first-party context
    Strict,

    /// Lax: cookie sent with top-level navigation
    Lax,

    /// None: cookie sent in all contexts (requires Secure)
    None,
}

impl fmt::Display for SameSite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SameSite::Strict => write!(f, "Strict"),
            SameSite::Lax => write!(f, "Lax"),
            SameSite::None => write!(f, "None"),
        }
    }
}

impl Cookie {
    /// Parse a Set-Cookie header value (RFC 6265)
    ///
    /// # Example
    /// ```
    /// use magneto_serge::cookies::Cookie;
    ///
    /// let cookie = Cookie::parse("session=abc123; Path=/; Secure; HttpOnly").unwrap();
    /// assert_eq!(cookie.name, "session");
    /// assert_eq!(cookie.value, "abc123");
    /// assert!(cookie.secure);
    /// assert!(cookie.http_only);
    /// ```
    pub fn parse(header_value: &str) -> Result<Self, CookieError> {
        let parts: Vec<&str> = header_value.split(';').map(|s| s.trim()).collect();

        if parts.is_empty() {
            return Err(CookieError::InvalidFormat(
                "Empty Set-Cookie header".to_string(),
            ));
        }

        // First part is name=value
        let name_value = parts[0];
        let (name, value) = name_value
            .split_once('=')
            .ok_or_else(|| CookieError::InvalidFormat("Missing '=' in name=value".to_string()))?;

        let mut cookie = Cookie {
            name: name.trim().to_string(),
            value: value.trim().to_string(),
            domain: None,
            path: None,
            expires: None,
            max_age: None,
            secure: false,
            http_only: false,
            same_site: None,
            created_at: Utc::now(),
        };

        // Parse attributes
        for part in &parts[1..] {
            if part.is_empty() {
                continue;
            }

            if let Some((key, val)) = part.split_once('=') {
                let key = key.trim().to_lowercase();
                let val = val.trim();

                match key.as_str() {
                    "domain" => cookie.domain = Some(val.to_string()),
                    "path" => cookie.path = Some(val.to_string()),
                    "expires" => {
                        // Parse HTTP date format (e.g., "Wed, 21 Oct 2025 07:28:00 GMT")
                        cookie.expires = parse_http_date(val).ok();
                    }
                    "max-age" => {
                        cookie.max_age = val.parse::<i64>().ok();
                    }
                    "samesite" => {
                        cookie.same_site = match val.to_lowercase().as_str() {
                            "strict" => Some(SameSite::Strict),
                            "lax" => Some(SameSite::Lax),
                            "none" => Some(SameSite::None),
                            _ => None,
                        };
                    }
                    _ => {} // Unknown attribute, ignore
                }
            } else {
                // Flag attributes (no value)
                let flag = part.trim().to_lowercase();
                match flag.as_str() {
                    "secure" => cookie.secure = true,
                    "httponly" => cookie.http_only = true,
                    _ => {} // Unknown flag, ignore
                }
            }
        }

        Ok(cookie)
    }

    /// Check if cookie is expired
    pub fn is_expired(&self) -> bool {
        let now = Utc::now();

        // Check Expires attribute
        if let Some(expires) = self.expires {
            if expires <= now {
                return true;
            }
        }

        // Check Max-Age attribute (takes precedence over Expires)
        if let Some(max_age) = self.max_age {
            if max_age <= 0 {
                return true;
            }
            let expires_at = self.created_at + Duration::seconds(max_age);
            if expires_at <= now {
                return true;
            }
        }

        false
    }

    /// Check if this cookie matches the given domain
    ///
    /// Implements domain matching as per RFC 6265 Section 5.1.3
    pub fn matches_domain(&self, request_domain: &str) -> bool {
        let cookie_domain = match &self.domain {
            Some(d) => d,
            None => return true, // No domain attribute means match current domain only
        };

        let cookie_domain = cookie_domain.to_lowercase();
        let request_domain = request_domain.to_lowercase();

        // Exact match
        if cookie_domain == request_domain {
            return true;
        }

        // Domain with leading dot matches subdomains
        if cookie_domain.starts_with('.') {
            let domain_without_dot = &cookie_domain[1..];
            if request_domain == domain_without_dot || request_domain.ends_with(&cookie_domain) {
                return true;
            }
        }

        false
    }

    /// Check if this cookie matches the given path
    ///
    /// Implements path matching as per RFC 6265 Section 5.1.4
    pub fn matches_path(&self, request_path: &str) -> bool {
        let cookie_path = match &self.path {
            Some(p) => p,
            None => "/", // Default path is "/"
        };

        // Exact match
        if cookie_path == request_path {
            return true;
        }

        // Cookie path is a prefix of request path
        if request_path.starts_with(cookie_path) {
            // Path must end with "/" or request path must have "/" after prefix
            if cookie_path.ends_with('/') {
                return true;
            }
            if let Some(next_char) = request_path.chars().nth(cookie_path.len()) {
                if next_char == '/' {
                    return true;
                }
            }
        }

        false
    }

    /// Serialize cookie to Cookie header format (name=value)
    pub fn to_header_value(&self) -> String {
        format!("{}={}", self.name, self.value)
    }
}

/// Cookie storage with domain/path scoping
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CookieJar {
    /// Cookies indexed by (domain, path, name)
    cookies: HashMap<CookieKey, Cookie>,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
struct CookieKey {
    domain: String,
    path: String,
    name: String,
}

impl CookieJar {
    /// Create a new empty cookie jar
    pub fn new() -> Self {
        Self {
            cookies: HashMap::new(),
        }
    }

    /// Store a cookie
    pub fn store(&mut self, cookie: Cookie) {
        let key = CookieKey {
            domain: cookie.domain.clone().unwrap_or_else(|| String::from("")),
            path: cookie.path.clone().unwrap_or_else(|| String::from("/")),
            name: cookie.name.clone(),
        };

        self.cookies.insert(key, cookie);
    }

    /// Get cookies that match the given URL
    ///
    /// Returns cookies sorted by:
    /// 1. Path length (longer paths first - more specific)
    /// 2. Creation time (older cookies first)
    pub fn get_matching(&self, url: &str) -> Vec<&Cookie> {
        // Parse URL to extract domain and path
        let (domain, path) = match parse_url(url) {
            Ok((d, p)) => (d, p),
            Err(_) => return Vec::new(),
        };

        let mut matching: Vec<&Cookie> = self
            .cookies
            .values()
            .filter(|cookie| !cookie.is_expired())
            .filter(|cookie| cookie.matches_domain(&domain))
            .filter(|cookie| cookie.matches_path(&path))
            .collect();

        // Sort by path length (descending) then by creation time (ascending)
        matching.sort_by(|a, b| {
            let path_a = a.path.as_deref().unwrap_or("/");
            let path_b = b.path.as_deref().unwrap_or("/");

            match path_b.len().cmp(&path_a.len()) {
                std::cmp::Ordering::Equal => a.created_at.cmp(&b.created_at),
                other => other,
            }
        });

        matching
    }

    /// Get all cookies as Cookie header value (name1=value1; name2=value2)
    pub fn get_header_value(&self, url: &str) -> Option<String> {
        let cookies = self.get_matching(url);

        if cookies.is_empty() {
            return None;
        }

        let header = cookies
            .iter()
            .map(|c| c.to_header_value())
            .collect::<Vec<_>>()
            .join("; ");

        Some(header)
    }

    /// Remove expired cookies
    pub fn purge_expired(&mut self) {
        self.cookies.retain(|_, cookie| !cookie.is_expired());
    }

    /// Clear all cookies
    pub fn clear(&mut self) {
        self.cookies.clear();
    }

    /// Get number of stored cookies
    pub fn len(&self) -> usize {
        self.cookies.len()
    }

    /// Check if jar is empty
    pub fn is_empty(&self) -> bool {
        self.cookies.is_empty()
    }
}

/// Parse HTTP date format (RFC 7231 Section 7.1.1.1)
fn parse_http_date(date_str: &str) -> Result<DateTime<Utc>, chrono::ParseError> {
    // Try multiple formats
    let formats = [
        "%a, %d %b %Y %H:%M:%S GMT",        // RFC 1123
        "%A, %d-%b-%y %H:%M:%S GMT",        // RFC 850 (obsolete)
        "%a %b %d %H:%M:%S %Y",             // ANSI C asctime()
    ];

    for format in &formats {
        if let Ok(dt) = DateTime::parse_from_str(date_str, format) {
            return Ok(dt.with_timezone(&Utc));
        }
    }

    // Fallback: try chrono's default RFC 3339
    DateTime::parse_from_rfc3339(date_str).map(|dt| dt.with_timezone(&Utc))
}

/// Parse URL to extract domain and path
fn parse_url(url: &str) -> Result<(String, String), url::ParseError> {
    let parsed = url::Url::parse(url)?;

    let domain = parsed
        .host_str()
        .ok_or_else(|| url::ParseError::EmptyHost)?
        .to_string();

    let path = parsed.path().to_string();

    Ok((domain, path))
}

/// Cookie-related errors
#[derive(Debug, thiserror::Error)]
pub enum CookieError {
    #[error("Invalid cookie format: {0}")]
    InvalidFormat(String),

    #[error("URL parse error: {0}")]
    UrlParseError(#[from] url::ParseError),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_cookie() {
        let cookie = Cookie::parse("session=abc123").unwrap();
        assert_eq!(cookie.name, "session");
        assert_eq!(cookie.value, "abc123");
        assert!(!cookie.secure);
        assert!(!cookie.http_only);
    }

    #[test]
    fn test_parse_cookie_with_attributes() {
        let cookie = Cookie::parse("token=xyz789; Path=/api; Secure; HttpOnly").unwrap();
        assert_eq!(cookie.name, "token");
        assert_eq!(cookie.value, "xyz789");
        assert_eq!(cookie.path, Some("/api".to_string()));
        assert!(cookie.secure);
        assert!(cookie.http_only);
    }

    #[test]
    fn test_parse_cookie_with_domain() {
        let cookie = Cookie::parse("id=123; Domain=.example.com; Path=/").unwrap();
        assert_eq!(cookie.name, "id");
        assert_eq!(cookie.domain, Some(".example.com".to_string()));
        assert_eq!(cookie.path, Some("/".to_string()));
    }

    #[test]
    fn test_parse_cookie_with_max_age() {
        let cookie = Cookie::parse("temp=abc; Max-Age=3600").unwrap();
        assert_eq!(cookie.max_age, Some(3600));
    }

    #[test]
    fn test_parse_cookie_with_samesite() {
        let cookie = Cookie::parse("csrf=token; SameSite=Strict").unwrap();
        assert_eq!(cookie.same_site, Some(SameSite::Strict));

        let cookie2 = Cookie::parse("tracking=xyz; SameSite=None; Secure").unwrap();
        assert_eq!(cookie2.same_site, Some(SameSite::None));
        assert!(cookie2.secure);
    }

    #[test]
    fn test_cookie_expiration_max_age() {
        let mut cookie = Cookie::parse("temp=123; Max-Age=0").unwrap();
        assert!(cookie.is_expired());

        cookie.max_age = Some(3600);
        assert!(!cookie.is_expired());
    }

    #[test]
    fn test_domain_matching() {
        let cookie = Cookie::parse("id=123; Domain=.example.com").unwrap();

        assert!(cookie.matches_domain("example.com"));
        assert!(cookie.matches_domain("www.example.com"));
        assert!(cookie.matches_domain("api.example.com"));
        assert!(!cookie.matches_domain("other.com"));
    }

    #[test]
    fn test_path_matching() {
        let cookie = Cookie::parse("id=123; Path=/api").unwrap();

        assert!(cookie.matches_path("/api"));
        assert!(cookie.matches_path("/api/users"));
        assert!(cookie.matches_path("/api/"));
        assert!(!cookie.matches_path("/other"));
        assert!(!cookie.matches_path("/ap"));
    }

    #[test]
    fn test_cookie_jar_store_and_retrieve() {
        let mut jar = CookieJar::new();

        let cookie1 = Cookie::parse("session=abc; Domain=.example.com; Path=/").unwrap();
        let cookie2 = Cookie::parse("token=xyz; Domain=.example.com; Path=/api").unwrap();

        jar.store(cookie1);
        jar.store(cookie2);

        assert_eq!(jar.len(), 2);

        // Should match both cookies for /api/users (more specific path first)
        let matches = jar.get_matching("https://example.com/api/users");
        assert_eq!(matches.len(), 2);
        assert_eq!(matches[0].name, "token"); // More specific path (/api) first
        assert_eq!(matches[1].name, "session"); // Less specific path (/) second
    }

    #[test]
    fn test_cookie_jar_header_value() {
        let mut jar = CookieJar::new();

        jar.store(Cookie::parse("session=abc123").unwrap());
        jar.store(Cookie::parse("token=xyz789").unwrap());

        let header = jar.get_header_value("https://example.com/").unwrap();
        assert!(header.contains("session=abc123"));
        assert!(header.contains("token=xyz789"));
        assert!(header.contains("; "));
    }

    #[test]
    fn test_cookie_jar_purge_expired() {
        let mut jar = CookieJar::new();

        jar.store(Cookie::parse("valid=123; Max-Age=3600").unwrap());
        jar.store(Cookie::parse("expired=456; Max-Age=0").unwrap());

        assert_eq!(jar.len(), 2);

        jar.purge_expired();

        assert_eq!(jar.len(), 1);
    }
}
