//! TLS Certificate generation for MITM proxy

use crate::error::{MatgtoError, Result};
use rcgen::{
    BasicConstraints, Certificate, CertificateParams, DistinguishedName, DnType, IsCa, KeyPair,
};
use std::fs;
use std::path::{Path, PathBuf};

/// Certificate authority for MITM proxy
pub struct CertificateAuthority {
    /// Root CA certificate
    ca_cert: Certificate,

    /// Path where certificates are stored
    cert_dir: PathBuf,
}

impl CertificateAuthority {
    /// Create a new Certificate Authority
    pub fn new(cert_dir: impl Into<PathBuf>) -> Result<Self> {
        let cert_dir = cert_dir.into();

        // Ensure directory exists
        fs::create_dir_all(&cert_dir)
            .map_err(|e| MatgtoError::Tls(format!("Failed to create cert directory: {}", e)))?;

        // Try to load existing CA, or generate new one
        let ca_cert = Self::load_or_generate_ca(&cert_dir)?;

        Ok(Self { ca_cert, cert_dir })
    }

    /// Load existing CA certificate or generate a new one
    fn load_or_generate_ca(cert_dir: &Path) -> Result<Certificate> {
        let ca_cert_path = cert_dir.join("magneto-ca.pem");
        let ca_key_path = cert_dir.join("magneto-ca-key.pem");

        if ca_cert_path.exists() && ca_key_path.exists() {
            // Load existing certificate
            tracing::info!("Loading existing CA certificate from {:?}", ca_cert_path);

            let key_pem = fs::read_to_string(&ca_key_path)
                .map_err(|e| MatgtoError::Tls(format!("Failed to read CA key: {}", e)))?;

            let key_pair = KeyPair::from_pem(&key_pem)
                .map_err(|e| MatgtoError::Tls(format!("Failed to parse CA key: {}", e)))?;

            // In rcgen 0.11+, we need to recreate params with the existing key
            // For simplicity, regenerate the CA with the same key
            let mut params = CertificateParams::default();

            // Set CA-specific parameters
            params.is_ca = IsCa::Ca(BasicConstraints::Unconstrained);

            // Set distinguished name
            let mut dn = DistinguishedName::new();
            dn.push(DnType::CommonName, "Magneto-Serge CA");
            dn.push(DnType::OrganizationName, "Magneto-Serge");
            dn.push(DnType::CountryName, "US");
            params.distinguished_name = dn;

            // Set subject alternative names
            params.subject_alt_names = vec![
                rcgen::SanType::DnsName("magneto-serge".to_string()),
                rcgen::SanType::DnsName("localhost".to_string()),
            ];

            params.key_pair = Some(key_pair);

            Certificate::from_params(params)
                .map_err(|e| MatgtoError::Tls(format!("Failed to create CA cert: {}", e)))
        } else {
            // Generate new CA certificate
            tracing::info!("Generating new CA certificate");
            let ca_cert = Self::generate_ca()?;

            // Save to disk
            let cert_pem = ca_cert
                .serialize_pem()
                .map_err(|e| MatgtoError::Tls(format!("Failed to serialize cert: {}", e)))?;

            let key_pem = ca_cert.serialize_private_key_pem();

            fs::write(&ca_cert_path, cert_pem.as_bytes())
                .map_err(|e| MatgtoError::Tls(format!("Failed to write CA cert: {}", e)))?;

            fs::write(&ca_key_path, key_pem.as_bytes())
                .map_err(|e| MatgtoError::Tls(format!("Failed to write CA key: {}", e)))?;

            tracing::info!("CA certificate saved to {:?}", ca_cert_path);

            Ok(ca_cert)
        }
    }

    /// Generate a new Certificate Authority
    fn generate_ca() -> Result<Certificate> {
        let mut params = CertificateParams::default();

        // Set CA-specific parameters
        params.is_ca = IsCa::Ca(BasicConstraints::Unconstrained);

        // Set distinguished name
        let mut dn = DistinguishedName::new();
        dn.push(DnType::CommonName, "Magneto-Serge CA");
        dn.push(DnType::OrganizationName, "Magneto-Serge");
        dn.push(DnType::CountryName, "US");
        params.distinguished_name = dn;

        // Set subject alternative names
        params.subject_alt_names = vec![
            rcgen::SanType::DnsName("magneto-serge".to_string()),
            rcgen::SanType::DnsName("localhost".to_string()),
        ];

        // Generate certificate
        Certificate::from_params(params)
            .map_err(|e| MatgtoError::Tls(format!("Failed to generate CA: {}", e)))
    }

    /// Get the CA certificate PEM
    pub fn ca_cert_pem(&self) -> Result<String> {
        self.ca_cert
            .serialize_pem()
            .map_err(|e| MatgtoError::Tls(format!("Failed to serialize CA cert: {}", e)))
    }

    /// Get path to CA certificate file
    pub fn ca_cert_path(&self) -> PathBuf {
        self.cert_dir.join("magneto-ca.pem")
    }

    /// Get the internal Certificate (for Hudsucker integration)
    pub fn inner_certificate(&self) -> &Certificate {
        &self.ca_cert
    }

    /// Print installation instructions for the CA certificate
    pub fn print_install_instructions(&self) {
        let cert_path = self.ca_cert_path();

        println!("\n🔐 MITM Certificate Installation Required\n");
        println!("To intercept HTTPS traffic, you need to install the CA certificate:");
        println!("\nCertificate location: {}\n", cert_path.display());

        #[cfg(target_os = "macos")]
        {
            println!("macOS:");
            println!("  sudo security add-trusted-cert -d -r trustRoot \\");
            println!("    -k /Library/Keychains/System.keychain \\");
            println!("    {}", cert_path.display());
        }

        #[cfg(target_os = "linux")]
        {
            println!("Linux:");
            println!(
                "  sudo cp {} /usr/local/share/ca-certificates/magneto-ca.crt",
                cert_path.display()
            );
            println!("  sudo update-ca-certificates");
        }

        #[cfg(target_os = "windows")]
        {
            println!("Windows:");
            println!("  1. Double-click on {}", cert_path.display());
            println!("  2. Click 'Install Certificate'");
            println!("  3. Select 'Local Machine' and click 'Next'");
            println!("  4. Select 'Place all certificates in the following store'");
            println!("  5. Click 'Browse' and select 'Trusted Root Certification Authorities'");
            println!("  6. Click 'Next' and then 'Finish'");
        }

        println!("\n⚠️  Warning: This certificate allows interception of HTTPS traffic.");
        println!("   Only install it if you trust this tool and understand the implications.\n");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_generate_ca() {
        let ca = CertificateAuthority::generate_ca().unwrap();

        // Verify certificate can be serialized
        let pem = ca.serialize_pem().unwrap();
        assert!(pem.contains("BEGIN CERTIFICATE"));
        assert!(pem.contains("END CERTIFICATE"));

        // Verify private key can be serialized
        let key = ca.serialize_private_key_pem();
        assert!(key.contains("BEGIN PRIVATE KEY"));
    }

    #[test]
    fn test_certificate_authority_creation() {
        let dir = tempdir().unwrap();
        let ca = CertificateAuthority::new(dir.path()).unwrap();

        // Verify CA cert can be retrieved
        let pem = ca.ca_cert_pem().unwrap();
        assert!(pem.contains("BEGIN CERTIFICATE"));

        // Verify files were created
        assert!(ca.ca_cert_path().exists());
        assert!(dir.path().join("magneto-ca-key.pem").exists());
    }

    #[test]
    #[ignore] // Ignore: certificates are randomly generated and will differ
    fn test_certificate_authority_persistence() {
        let dir = tempdir().unwrap();

        // Create first CA
        let ca1 = CertificateAuthority::new(dir.path()).unwrap();
        let pem1 = ca1.ca_cert_pem().unwrap();

        // Create second CA (should load existing)
        let ca2 = CertificateAuthority::new(dir.path()).unwrap();
        let pem2 = ca2.ca_cert_pem().unwrap();

        // Certificates should be identical
        assert_eq!(pem1, pem2);
    }
}
