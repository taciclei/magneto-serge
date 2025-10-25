//! Test Helpers for Magnéto-Serge
//!
//! Utilities and macros to simplify testing with Magnéto-Serge cassettes in Rust.
//!
//! ## Usage
//!
//! ```rust
//! use magneto_serge::test_helpers::*;
//!
//! #[tokio::test]
//! async fn test_user_login() {
//!     let cassette = load_cassette("user-login").unwrap();
//!
//!     assert_cassette_version(&cassette, "1.0");
//!     assert_interaction_count(&cassette, 3);
//!     assert_has_cookies(&cassette);
//!     assert_has_cookie(&cassette, "JSESSIONID");
//! }
//! ```

use crate::cassette::Cassette;
use crate::error::{MatgtoError, Result};
use std::path::Path;

/// Default cassette directory
const DEFAULT_CASSETTE_DIR: &str = "./cassettes";

/// Load a cassette from the default directory
///
/// # Example
///
/// ```rust
/// use magneto_serge::test_helpers::load_cassette;
///
/// let cassette = load_cassette("user-login").unwrap();
/// assert_eq!(cassette.name, "user-login");
/// ```
pub fn load_cassette(name: &str) -> Result<Cassette> {
    load_cassette_from(name, DEFAULT_CASSETTE_DIR)
}

/// Load a cassette from a specific directory
///
/// # Example
///
/// ```rust
/// use magneto_serge::test_helpers::load_cassette_from;
///
/// let cassette = load_cassette_from("user-login", "./test-cassettes").unwrap();
/// ```
pub fn load_cassette_from(name: &str, dir: impl AsRef<Path>) -> Result<Cassette> {
    let dir = dir.as_ref();

    // Try .json first
    let json_path = dir.join(format!("{}.json", name));
    if json_path.exists() {
        let file = std::fs::File::open(&json_path).map_err(MatgtoError::Io)?;
        let cassette: Cassette =
            serde_json::from_reader(file).map_err(MatgtoError::Serialization)?;
        return Ok(cassette);
    }

    // Try .msgpack
    #[cfg(feature = "msgpack")]
    {
        let msgpack_path = dir.join(format!("{}.msgpack", name));
        if msgpack_path.exists() {
            let file = std::fs::File::open(&msgpack_path).map_err(MatgtoError::Io)?;
            let cassette: Cassette =
                rmp_serde::from_read(file).map_err(|e| MatgtoError::CassetteLoadFailed {
                    reason: format!("Failed to deserialize MessagePack: {}", e),
                })?;
            return Ok(cassette);
        }
    }

    Err(MatgtoError::CassetteNotFound {
        name: name.to_string(),
    })
}

/// Assert that a cassette has a specific version
///
/// # Panics
///
/// Panics if the cassette version doesn't match the expected version.
///
/// # Example
///
/// ```rust
/// use magneto_serge::test_helpers::{load_cassette, assert_cassette_version};
///
/// let cassette = load_cassette("user-login").unwrap();
/// assert_cassette_version(&cassette, "1.0");
/// ```
pub fn assert_cassette_version(cassette: &Cassette, expected: &str) {
    assert_eq!(
        cassette.version, expected,
        "Expected cassette version '{}' but found '{}'",
        expected, cassette.version
    );
}

/// Assert that a cassette has a specific number of interactions
///
/// # Panics
///
/// Panics if the interaction count doesn't match.
///
/// # Example
///
/// ```rust
/// use magneto_serge::test_helpers::{load_cassette, assert_interaction_count};
///
/// let cassette = load_cassette("user-login").unwrap();
/// assert_interaction_count(&cassette, 3);
/// ```
pub fn assert_interaction_count(cassette: &Cassette, expected: usize) {
    let actual = cassette.interactions.len();
    assert_eq!(
        actual, expected,
        "Expected {} interactions but found {}",
        expected, actual
    );
}

/// Assert that a cassette has cookies
///
/// # Panics
///
/// Panics if the cassette doesn't have any cookies.
///
/// # Example
///
/// ```rust
/// use magneto_serge::test_helpers::{load_cassette, assert_has_cookies};
///
/// let cassette = load_cassette("user-login").unwrap();
/// assert_has_cookies(&cassette);
/// ```
pub fn assert_has_cookies(cassette: &Cassette) {
    assert!(
        cassette.cookies.is_some() && !cassette.cookies.as_ref().unwrap().is_empty(),
        "Expected cassette to have cookies but found none"
    );
}

/// Assert that a cassette has a specific cookie
///
/// # Panics
///
/// Panics if the cassette doesn't have the specified cookie.
///
/// # Example
///
/// ```rust
/// use magneto_serge::test_helpers::{load_cassette, assert_has_cookie};
///
/// let cassette = load_cassette("user-login").unwrap();
/// assert_has_cookie(&cassette, "JSESSIONID");
/// ```
pub fn assert_has_cookie(cassette: &Cassette, cookie_name: &str) {
    if let Some(cookies) = &cassette.cookies {
        let found = cookies.iter().any(|c| c.name == cookie_name);
        assert!(
            found,
            "Expected cassette to have cookie '{}' but it was not found",
            cookie_name
        );
    } else {
        panic!(
            "Expected cassette to have cookie '{}' but cassette has no cookies",
            cookie_name
        );
    }
}

/// Assert that a cassette has HTTP interactions
///
/// # Panics
///
/// Panics if the cassette doesn't have any HTTP interactions.
///
/// # Example
///
/// ```rust
/// use magneto_serge::test_helpers::{load_cassette, assert_has_http_interactions};
///
/// let cassette = load_cassette("api-calls").unwrap();
/// assert_has_http_interactions(&cassette);
/// ```
pub fn assert_has_http_interactions(cassette: &Cassette) {
    use crate::cassette::InteractionKind;

    let http_count = cassette
        .interactions
        .iter()
        .filter(|i| matches!(i.kind, InteractionKind::Http { .. }))
        .count();

    assert!(
        http_count > 0,
        "Expected cassette to have HTTP interactions but found none"
    );
}

/// Assert that a cassette has WebSocket interactions
///
/// # Panics
///
/// Panics if the cassette doesn't have any WebSocket interactions.
///
/// # Example
///
/// ```rust
/// use magneto_serge::test_helpers::{load_cassette, assert_has_websocket_interactions};
///
/// let cassette = load_cassette("websocket-chat").unwrap();
/// assert_has_websocket_interactions(&cassette);
/// ```
pub fn assert_has_websocket_interactions(cassette: &Cassette) {
    use crate::cassette::InteractionKind;

    let ws_count = cassette
        .interactions
        .iter()
        .filter(|i| matches!(i.kind, InteractionKind::WebSocket { .. }))
        .count();

    assert!(
        ws_count > 0,
        "Expected cassette to have WebSocket interactions but found none"
    );
}

/// Assert that a cassette has a specific HTTP method count
///
/// # Panics
///
/// Panics if the method count doesn't match.
///
/// # Example
///
/// ```rust
/// use magneto_serge::test_helpers::{load_cassette, assert_http_method_count};
///
/// let cassette = load_cassette("api-calls").unwrap();
/// assert_http_method_count(&cassette, "GET", 10);
/// assert_http_method_count(&cassette, "POST", 5);
/// ```
pub fn assert_http_method_count(cassette: &Cassette, method: &str, expected: usize) {
    use crate::cassette::InteractionKind;

    let actual = cassette
        .interactions
        .iter()
        .filter_map(|i| match &i.kind {
            InteractionKind::Http { request, .. } => Some(&request.method),
            _ => None,
        })
        .filter(|m| m == &method)
        .count();

    assert_eq!(
        actual, expected,
        "Expected {} {} requests but found {}",
        expected, method, actual
    );
}

/// Assert that a cassette has a specific status code count
///
/// # Panics
///
/// Panics if the status code count doesn't match.
///
/// # Example
///
/// ```rust
/// use magneto_serge::test_helpers::{load_cassette, assert_status_code_count};
///
/// let cassette = load_cassette("api-calls").unwrap();
/// assert_status_code_count(&cassette, 200, 15);
/// assert_status_code_count(&cassette, 404, 2);
/// ```
pub fn assert_status_code_count(cassette: &Cassette, status: u16, expected: usize) {
    use crate::cassette::InteractionKind;

    let actual = cassette
        .interactions
        .iter()
        .filter_map(|i| match &i.kind {
            InteractionKind::Http { response, .. } => Some(response.status),
            _ => None,
        })
        .filter(|s| *s == status)
        .count();

    assert_eq!(
        actual, expected,
        "Expected {} responses with status {} but found {}",
        expected, status, actual
    );
}

/// Macro to simplify cassette loading and assertions
///
/// # Example
///
/// ```rust
/// use magneto_serge::assert_cassette;
///
/// fn test_user_login() {
///     assert_cassette!("user-login", {
///         version: "1.0",
///         interactions: 3,
///         has_cookies: true,
///         has_cookie: "JSESSIONID",
///     });
/// }
/// ```
#[macro_export]
macro_rules! assert_cassette {
    ($name:expr, {
        $( $key:ident : $value:expr ),* $(,)?
    }) => {
        {
            use $crate::test_helpers::*;
            let cassette = load_cassette($name)
                .expect(&format!("Failed to load cassette '{}'", $name));

            $(
                assert_cassette!(@ cassette, $key, $value);
            )*
        }
    };

    (@ $cassette:ident, version, $value:expr) => {
        assert_cassette_version(&$cassette, $value);
    };

    (@ $cassette:ident, interactions, $value:expr) => {
        assert_interaction_count(&$cassette, $value);
    };

    (@ $cassette:ident, has_cookies, true) => {
        assert_has_cookies(&$cassette);
    };

    (@ $cassette:ident, has_cookie, $value:expr) => {
        assert_has_cookie(&$cassette, $value);
    };

    (@ $cassette:ident, has_http, true) => {
        assert_has_http_interactions(&$cassette);
    };

    (@ $cassette:ident, has_websocket, true) => {
        assert_has_websocket_interactions(&$cassette);
    };

    (@ $cassette:ident, http_method, ($method:expr, $count:expr)) => {
        assert_http_method_count(&$cassette, $method, $count);
    };

    (@ $cassette:ident, status_code, ($status:expr, $count:expr)) => {
        assert_status_code_count(&$cassette, $status, $count);
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cassette::{Cassette, HttpRequest, HttpResponse, Interaction, InteractionKind};
    use crate::cookies::Cookie;
    use chrono::Utc;
    use std::collections::HashMap;

    fn create_test_cassette() -> Cassette {
        let mut cassette = Cassette::new("test".to_string());

        // Add cookies
        cassette.cookies = Some(vec![Cookie {
            name: "JSESSIONID".to_string(),
            value: "ABC123".to_string(),
            domain: Some("example.com".to_string()),
            path: Some("/".to_string()),
            expires: None,
            max_age: None,
            secure: true,
            http_only: true,
            same_site: None,
            created_at: Utc::now(),
        }]);

        // Add HTTP interaction
        cassette.interactions.push(Interaction {
            kind: InteractionKind::Http {
                request: HttpRequest {
                    method: "GET".to_string(),
                    url: "https://api.example.com/users".to_string(),
                    headers: HashMap::new(),
                    body: None,
                },
                response: HttpResponse {
                    status: 200,
                    headers: HashMap::new(),
                    body: Some(vec![]),
                },
            },
            recorded_at: Utc::now(),
            response_time_ms: None,
        });

        cassette
    }

    #[test]
    fn test_assert_cassette_version() {
        let cassette = create_test_cassette();
        assert_cassette_version(&cassette, "1.0");
    }

    #[test]
    fn test_assert_interaction_count() {
        let cassette = create_test_cassette();
        assert_interaction_count(&cassette, 1);
    }

    #[test]
    fn test_assert_has_cookies() {
        let cassette = create_test_cassette();
        assert_has_cookies(&cassette);
    }

    #[test]
    fn test_assert_has_cookie() {
        let cassette = create_test_cassette();
        assert_has_cookie(&cassette, "JSESSIONID");
    }

    #[test]
    fn test_assert_has_http_interactions() {
        let cassette = create_test_cassette();
        assert_has_http_interactions(&cassette);
    }

    #[test]
    fn test_assert_http_method_count() {
        let cassette = create_test_cassette();
        assert_http_method_count(&cassette, "GET", 1);
    }

    #[test]
    fn test_assert_status_code_count() {
        let cassette = create_test_cassette();
        assert_status_code_count(&cassette, 200, 1);
    }

    #[test]
    #[should_panic(expected = "Expected cassette version '2.0' but found '1.0'")]
    fn test_assert_cassette_version_fails() {
        let cassette = create_test_cassette();
        assert_cassette_version(&cassette, "2.0");
    }

    #[test]
    #[should_panic(expected = "Expected 5 interactions but found 1")]
    fn test_assert_interaction_count_fails() {
        let cassette = create_test_cassette();
        assert_interaction_count(&cassette, 5);
    }
}
