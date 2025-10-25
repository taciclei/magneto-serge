#![deny(clippy::all)]

use napi::bindgen_prelude::*;
use napi_derive::napi;
use magneto_serge::{MagnetoProxy as RustProxy, ProxyMode as RustProxyMode};
use std::sync::Once;

static INIT: Once = Once::new();

/// Initialize tracing subscriber (called automatically on first use)
fn init_tracing() {
  INIT.call_once(|| {
    tracing_subscriber::fmt()
      .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
      .init();
  });
}

#[napi]
pub enum ProxyMode {
  Auto,
  Record,
  Replay,
  Passthrough,
}

impl From<ProxyMode> for RustProxyMode {
  fn from(mode: ProxyMode) -> Self {
    match mode {
      ProxyMode::Auto => RustProxyMode::Auto,
      ProxyMode::Record => RustProxyMode::Record,
      ProxyMode::Replay => RustProxyMode::Replay,
      ProxyMode::Passthrough => RustProxyMode::Passthrough,
    }
  }
}

#[napi]
pub struct MagnetoProxy {
  inner: RustProxy,
}

#[napi]
impl MagnetoProxy {
  #[napi(constructor)]
  pub fn new(cassette_dir: String) -> Result<Self> {
    init_tracing();
    tracing::info!("Creating MagnetoProxy with cassette_dir: {}", cassette_dir);
    RustProxy::new_internal(&cassette_dir)
      .map(|inner| Self { inner })
      .map_err(|e| Error::from_reason(e.to_string()))
  }

  #[napi]
  pub fn set_port(&self, port: u16) {
    self.inner.set_port(port);
  }

  #[napi]
  pub fn set_mode(&self, mode: ProxyMode) {
    self.inner.set_mode(mode.into());
  }

  #[napi]
  pub fn start_recording(&self, cassette_name: String) -> Result<()> {
    tracing::info!("NAPI: start_recording called for cassette: {}", cassette_name);
    let result = self.inner.start_recording_internal(cassette_name);
    if result.is_err() {
      tracing::error!("NAPI: start_recording failed: {:?}", result);
    } else {
      tracing::info!("NAPI: start_recording succeeded");
    }
    result.map_err(|e| Error::from_reason(e.to_string()))
  }

  #[napi]
  pub fn stop_recording(&self) -> Result<()> {
    self.inner.stop_recording_internal()
      .map_err(|e| Error::from_reason(e.to_string()))
  }

  #[napi]
  pub fn replay(&self, cassette_name: String) -> Result<()> {
    self.inner.replay_internal(cassette_name)
      .map_err(|e| Error::from_reason(e.to_string()))
  }

  #[napi]
  pub fn shutdown(&self) -> Result<()> {
    self.inner.shutdown_internal()
      .map_err(|e| Error::from_reason(e.to_string()))
  }

  #[napi]
  pub fn get_port(&self) -> u16 {
    self.inner.port()
  }
}

#[napi]
pub fn version() -> String {
  magneto_serge::version()
}
