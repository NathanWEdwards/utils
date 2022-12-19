//! MIT license.

pub async fn get_html_body(client: &awc::Client, url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut client_response = client.get(url).send().await?;
    let response_body = client_response.body().await?;
    let body = std::str::from_utf8(&response_body)?;
    Ok(String::from(body))
}

pub fn new_client<'a, T>(certificate_files: T) -> awc::Client
where
    T: IntoIterator<Item = &'a String>,
    T::IntoIter: 'a,
{
    let mut root_store: rustls::RootCertStore = rustls::RootCertStore::empty();

    for certificate_file in certificate_files {
        match std::fs::read(&certificate_file[..]) {
            Ok(file_contents) => {
                let mut buffer: std::io::BufReader<&[u8]> =
                    std::io::BufReader::new(file_contents.as_slice());
                let certificates: Vec<Vec<u8>> = match rustls_pemfile::certs(&mut buffer) {
                    Ok(certificates) => certificates,
                    Err(error) => {
                        eprintln!("{{\"error\": \"{}\"}}", error);
                        std::process::exit(1);
                    }
                };
                for certificate_bytes in certificates {
                    let certificate: rustls::Certificate = rustls::Certificate(certificate_bytes);
                    let add_to_root_store_result = root_store.add(&certificate);
                    if add_to_root_store_result.is_err() {
                        eprintln!("{{\"error\": \"{:#?}\"}}", add_to_root_store_result.err());
                        std::process::exit(1);
                    }
                }
            }
            Err(error) => {
                eprintln!(
                    "{{\"file\": \"{}\", \"error\":\"{}\"}}",
                    certificate_file, error
                );
                std::process::exit(1);
            }
        }
    }
    let rustls_config: rustls::ClientConfig = rustls::ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(root_store)
        .with_no_client_auth();
    let client_tls_config: std::sync::Arc<rustls::ClientConfig> =
        std::sync::Arc::new(rustls_config);
    awc::Client::builder()
        .add_default_header((awc::http::header::USER_AGENT, "utils/0,1"))
        .connector(awc::Connector::new().rustls(std::sync::Arc::clone(&client_tls_config)))
        .finish()
}
