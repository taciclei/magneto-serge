//! # magneto-serge-test
//!
//! Test framework integration for magneto-serge with automatic cassette management.
//!
//! ## Features
//!
//! - `#[magneto_test]` attribute macro for automatic proxy setup/teardown
//! - Auto-naming cassettes from test function names
//! - Support for custom cassette names and modes
//! - Compatible with `#[tokio::test]` and async tests
//!
//! ## Usage
//!
//! ```rust,ignore
//! use magneto_serge_test::magneto_test;
//!
//! #[magneto_test]
//! async fn test_api_call() {
//!     // Proxy auto-started with cassette "test_api_call"
//!     // Configure your HTTP client to use http://localhost:8888
//!     let response = reqwest::get("http://api.example.com/users").await?;
//!     assert_eq!(response.status(), 200);
//!     // Proxy auto-stopped, cassette saved to ./cassettes/test_api_call.json
//! }
//!
//! #[magneto_test(cassette = "shared_cassette", mode = "replay")]
//! async fn test_with_options() {
//!     // Use shared cassette in replay mode
//! }
//! ```

use proc_macro::TokenStream;
use quote::quote;
use syn::parse::Parser;
use syn::{parse_macro_input, Expr, ExprLit, ItemFn, Lit, Meta, MetaNameValue};

/// Attribute macro for automatic magneto-serge proxy setup in tests.
///
/// # Basic Usage
///
/// ```rust,ignore
/// #[magneto_test]
/// async fn test_my_api() {
///     // Cassette auto-named "test_my_api"
///     // Proxy runs on localhost:8888
///     let response = reqwest::get("http://api.example.com").await?;
///     assert_eq!(response.status(), 200);
/// }
/// ```
///
/// # Custom Cassette Name
///
/// ```rust,ignore
/// #[magneto_test(cassette = "custom_name")]
/// async fn test_with_custom_cassette() {
///     // Uses cassette "custom_name"
/// }
/// ```
///
/// # Custom Mode
///
/// ```rust,ignore
/// #[magneto_test(mode = "replay")]
/// async fn test_replay_only() {
///     // Strict replay mode (fails if cassette missing)
/// }
///
/// #[magneto_test(mode = "record")]
/// async fn test_force_record() {
///     // Always record (overwrites existing cassette)
/// }
///
/// #[magneto_test(mode = "auto")]
/// async fn test_auto_mode() {
///     // Record if cassette missing, else replay (default)
/// }
/// ```
///
/// # Custom Cassette Directory
///
/// ```rust,ignore
/// #[magneto_test(cassette_dir = "./my_cassettes")]
/// async fn test_custom_dir() {
///     // Saves cassette to ./my_cassettes/test_custom_dir.json
/// }
/// ```
///
/// # Custom Port
///
/// ```rust,ignore
/// #[magneto_test(port = 9999)]
/// async fn test_custom_port() {
///     // Proxy runs on localhost:9999
/// }
/// ```
///
/// # Combined Options
///
/// ```rust,ignore
/// #[magneto_test(
///     cassette = "integration_test",
///     mode = "replay",
///     cassette_dir = "./fixtures",
///     port = 9000
/// )]
/// async fn test_with_all_options() {
///     // Full control over proxy configuration
/// }
/// ```
#[proc_macro_attribute]
pub fn magneto_test(args: TokenStream, input: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(input as ItemFn);

    // Parse args as a list of Meta items
    let args_parser = syn::punctuated::Punctuated::<Meta, syn::Token![,]>::parse_terminated;
    let args = match args_parser.parse(args.clone().into()) {
        Ok(args) => args,
        Err(_) => syn::punctuated::Punctuated::new(),
    };

    // Parse attributes
    let mut cassette_name: Option<String> = None;
    let mut cassette_dir: Option<String> = None;
    let mut mode: Option<String> = None;
    let mut port: Option<u16> = None;

    for meta in args {
        if let Meta::NameValue(MetaNameValue { path, value, .. }) = meta {
            let ident = path.get_ident().map(|i| i.to_string());

            match (ident.as_deref(), &value) {
                (
                    Some("cassette"),
                    Expr::Lit(ExprLit {
                        lit: Lit::Str(s), ..
                    }),
                ) => {
                    cassette_name = Some(s.value());
                }
                (
                    Some("cassette_dir"),
                    Expr::Lit(ExprLit {
                        lit: Lit::Str(s), ..
                    }),
                ) => {
                    cassette_dir = Some(s.value());
                }
                (
                    Some("mode"),
                    Expr::Lit(ExprLit {
                        lit: Lit::Str(s), ..
                    }),
                ) => {
                    mode = Some(s.value());
                }
                (
                    Some("port"),
                    Expr::Lit(ExprLit {
                        lit: Lit::Int(i), ..
                    }),
                ) => {
                    port = i.base10_parse().ok();
                }
                _ => {}
            }
        }
    }

    // Get function name for default cassette name
    let fn_name = &input_fn.sig.ident;
    let cassette = cassette_name.unwrap_or_else(|| fn_name.to_string());
    let cassette_dir_str = cassette_dir.unwrap_or_else(|| "./cassettes".to_string());
    let mode_str = mode.unwrap_or_else(|| "auto".to_string());
    let port_num = port.unwrap_or(8888);

    // Original function attributes and visibility
    let attrs = &input_fn.attrs;
    let vis = &input_fn.vis;
    let sig = &input_fn.sig;
    let block = &input_fn.block;

    // Generate the wrapper
    let expanded = quote! {
        #(#attrs)*
        #[::tokio::test]
        #vis #sig {
            use ::std::sync::Arc;
            use ::std::path::PathBuf;

            // Create cassette directory
            let cassette_dir = PathBuf::from(#cassette_dir_str);
            ::std::fs::create_dir_all(&cassette_dir).expect("Failed to create cassette directory");

            // Create proxy
            let proxy = ::magneto_serge::MagnetoProxy::new(&cassette_dir)
                .expect("Failed to create MagnetoProxy");

            // Parse mode
            let mode = match #mode_str {
                "record" => ::magneto_serge::ProxyMode::Record,
                "replay" => ::magneto_serge::ProxyMode::Replay,
                "auto" => ::magneto_serge::ProxyMode::Auto,
                "passthrough" => ::magneto_serge::ProxyMode::Passthrough,
                _ => ::magneto_serge::ProxyMode::Auto,
            };

            // Start proxy with cassette
            proxy.set_mode(mode).expect("Failed to set proxy mode");
            proxy.set_port(#port_num).expect("Failed to set proxy port");

            // Start recording/replaying
            match mode {
                ::magneto_serge::ProxyMode::Record => {
                    proxy.start_recording(#cassette).expect("Failed to start recording");
                }
                ::magneto_serge::ProxyMode::Replay => {
                    proxy.start_replay(#cassette).expect("Failed to start replay");
                }
                ::magneto_serge::ProxyMode::Auto => {
                    // Check if cassette exists
                    let cassette_path = cassette_dir.join(format!("{}.json", #cassette));
                    if cassette_path.exists() {
                        proxy.start_replay(#cassette).expect("Failed to start replay");
                    } else {
                        proxy.start_recording(#cassette).expect("Failed to start recording");
                    }
                }
                ::magneto_serge::ProxyMode::Passthrough => {
                    proxy.start_passthrough().expect("Failed to start passthrough");
                }
            }

            // Run the test
            let result = async #block.await;

            // Stop proxy and save cassette
            if matches!(mode, ::magneto_serge::ProxyMode::Record) {
                proxy.stop_recording().expect("Failed to stop recording");
            } else if matches!(mode, ::magneto_serge::ProxyMode::Replay) {
                proxy.stop_replay().expect("Failed to stop replay");
            } else if matches!(mode, ::magneto_serge::ProxyMode::Auto) {
                let cassette_path = cassette_dir.join(format!("{}.json", #cassette));
                if cassette_path.exists() {
                    proxy.stop_replay().expect("Failed to stop replay");
                } else {
                    proxy.stop_recording().expect("Failed to stop recording");
                }
            } else if matches!(mode, ::magneto_serge::ProxyMode::Passthrough) {
                proxy.stop_passthrough().expect("Failed to stop passthrough");
            }

            result
        }
    };

    TokenStream::from(expanded)
}
