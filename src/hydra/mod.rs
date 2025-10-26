//! Hydra Core Vocabulary Implementation
//!
//! This module provides support for Hydra (Hypermedia-Driven Web APIs) and JSON-LD
//! (JavaScript Object Notation for Linked Data), enabling RESTful APIs to be
//! self-descriptive and machine-readable.
//!
//! # Hydra Core Vocabulary
//!
//! Hydra is a lightweight vocabulary to create hypermedia-driven Web APIs.
//! By specifying a number of concepts commonly used in Web APIs, it enables
//! a server to advertise valid state transitions to a client.
//!
//! ## Key Concepts
//!
//! - **ApiDocumentation**: Entry point describing the entire API
//! - **Class**: Represents a resource type (e.g., Cassette, Interaction)
//! - **Collection**: Paginated list of resources
//! - **Operation**: Actions that can be performed (GET, POST, PUT, DELETE)
//! - **Link**: Hypermedia links between resources
//!
//! # JSON-LD
//!
//! JSON-LD is a method of encoding Linked Data using JSON. It allows data to be
//! serialized in a way that is similar to traditional JSON, while also being
//! machine-readable as RDF.
//!
//! ## Example
//!
//! ```json
//! {
//!   "@context": "http://www.w3.org/ns/hydra/context.jsonld",
//!   "@id": "/api/cassettes/test",
//!   "@type": "Cassette",
//!   "name": "test",
//!   "version": "1.0",
//!   "hydra:operation": [
//!     {
//!       "@type": "hydra:Operation",
//!       "hydra:method": "GET",
//!       "hydra:returns": "Cassette"
//!     }
//!   ]
//! }
//! ```
//!
//! # References
//!
//! - Hydra Core Vocabulary: <https://www.hydra-cg.com/spec/latest/core/>
//! - JSON-LD 1.1: <https://www.w3.org/TR/json-ld11/>
//! - W3C Hydra Namespace: <http://www.w3.org/ns/hydra/core#>

pub mod collection;
pub mod context;
pub mod documentation;
pub mod error;
pub mod operation;
pub mod resources;
pub mod response;
pub mod vocabulary;

// Re-exports for convenience
pub use collection::{HydraCollection, HydraSearch, HydraView};
pub use context::JsonLdContext;
pub use documentation::{ApiDocumentation, SupportedClass, SupportedProperty};
pub use error::HydraError;
pub use operation::HydraOperation;
pub use resources::{CassetteResource, InteractionResource, TemplateResource};
pub use response::HydraResponse;
pub use vocabulary::{HydraClass, HydraLink, HydraProperty};

/// Hydra namespace URI
pub const HYDRA_NAMESPACE: &str = "http://www.w3.org/ns/hydra/core#";

/// RDF namespace URI
pub const RDF_NAMESPACE: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#";

/// RDFS namespace URI
pub const RDFS_NAMESPACE: &str = "http://www.w3.org/2000/01/rdf-schema#";

/// XSD namespace URI
pub const XSD_NAMESPACE: &str = "http://www.w3.org/2001/XMLSchema#";

/// Schema.org namespace URI
pub const SCHEMA_NAMESPACE: &str = "http://schema.org/";

/// Magneto-Serge vocabulary namespace URI
pub const MAGNETO_NAMESPACE: &str = "http://magneto-serge.dev/vocab#";
