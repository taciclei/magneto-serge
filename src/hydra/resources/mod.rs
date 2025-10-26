//! Hydra Resources
//!
//! Resource representations for Hydra API endpoints.

pub mod cassette;
pub mod interaction;
pub mod template;

pub use cassette::CassetteResource;
pub use interaction::InteractionResource;
pub use template::TemplateResource;
