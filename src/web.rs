//!  MIT license.

///  Perform an HTTP request to retrieve a web page's HTML content.
pub async fn get_html_body(client: &awc::Client, url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut client_response = client.get(url).send().await?;
    let response_body = client_response.body().await?;
    let body = std::str::from_utf8(&response_body)?;
    Ok(String::from(body))
}

///  Build and return an Actix Web Client with DER encoded X.509 certificates.
pub fn new_client<'a, T>(certificate_files: T) -> awc::Client
where
    T: IntoIterator<Item = &'a String>,
    T::IntoIter: 'a,
{
    let mut root_store: rustls::RootCertStore = rustls::RootCertStore::empty();

    //  For each certificate file provided,
    for certificate_file in certificate_files {
        //  Read the file,
        match std::fs::read(&certificate_file[..]) {
            Ok(file_contents) => {
                //  If the file opens successfully,
                //  Add the file contents to a buffer and instantiates a rustls certificate object.
                let mut buffer: std::io::BufReader<&[u8]> =
                    std::io::BufReader::new(file_contents.as_slice());
                let certificates: Vec<Vec<u8>> = match rustls_pemfile::certs(&mut buffer) {
                    Ok(certificates) => certificates,
                    Err(error) => {
                        eprintln!("{{\"error\": \"{}\"}}", error);
                        std::process::exit(1);
                    }
                };
                //  For each individual certificate,
                for certificate_bytes in certificates {
                    //  Add the certificate to the root store.
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

    //  Configure rustls.
    let rustls_config: rustls::ClientConfig = rustls::ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(root_store)
        .with_no_client_auth();

    //  Instantiate a thread-safe atomically reference-counting pointer to share the client configuration with the Actix Web Client Rustls Connector and Client
    let client_tls_config: std::sync::Arc<rustls::ClientConfig> =
        std::sync::Arc::new(rustls_config);

    // Return an Actix Web Client object.
    awc::Client::builder()
        .add_default_header((awc::http::header::USER_AGENT, "utils/0,1"))
        .connector(awc::Connector::new().rustls(std::sync::Arc::clone(&client_tls_config)))
        .finish()
}
