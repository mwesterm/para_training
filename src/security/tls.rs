use std::{fs::File, io::BufReader};

use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::pkcs8_private_keys;

pub fn load_certs(cert_filename: &str, key_filename: &str) -> Result<ServerConfig, String> {
    let cert_file = &mut BufReader::new(File::open(&cert_filename).map_err(|e| e.to_string())?);
    let key_file = &mut BufReader::new(File::open(&key_filename).map_err(|e| e.to_string())?);

    let cert_chain = rustls_pemfile::certs(cert_file)
        .map_err(|e| e.to_string())?
        .into_iter()
        .map(Certificate)
        .collect();
    let mut keys: Vec<PrivateKey> = pkcs8_private_keys(key_file)
        .map_err(|e| e.to_string())?
        .into_iter()
        .map(PrivateKey)
        .collect();

    if keys.is_empty() {
        return Err("Could not locate PKCS 8 private keys.".to_string());
    }

    let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth();
    config
        .with_single_cert(cert_chain, keys.remove(0))
        .map_err(|e| e.to_string())
}
