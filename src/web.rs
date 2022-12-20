//! MIT license.

/// Perform an HTTP request to retrieve a web page's HTML content.
/// 
/// # Example
/// 
/// ```
/// let expected: &str = "ENTRY       7105              CDS       T01001
/// SYMBOL      TSPAN6, T245, TM4SF6, TSPAN-6
/// NAME        (RefSeq) tetraspanin 6
/// ORTHOLOGY   K17295  tetraspanin-6
/// ORGANISM    hsa  Homo sapiens (human)
/// BRITE       KEGG Orthology (KO) [BR:hsa00001]
///              09180 Brite Hierarchies
///               09183 Protein families: signaling and cellular processes
///                04147 Exosome [BR:hsa04147]
///                 7105 (TSPAN6)
///             Exosome [BR:hsa04147]
///              Exosomal proteins
///               Exosomal proteins of colorectal cancer cells
///                7105 (TSPAN6)
/// POSITION    X:complement(100627108..100637104)
/// MOTIF       Pfam: Tetraspanin 4HB_MCP_1 Phage_holin_3_6
/// DBLINKS     NCBI-GeneID: 7105
///             NCBI-ProteinID: NP_003261
///             OMIM: 300191
///             HGNC: 11858
///             Ensembl: ENSG00000000003
///             Pharos: O43657(Tbio)
///             UniProt: O43657
/// AASEQ       245
///             MASPSRRLQTKPVITCFKSVLLIYTFIFWITGVILLAVGIWGKVSLENYFSLLNEKATNV
///             PFVLIATGTVIILLGTFGCFATCRASAWMLKLYAMFLTLVFLVELVAAIVGFVFRHEIKN
///             SFKNNYEKALKQYNSTGDYRSHAVDKIQNTLHCCGVTDYRDWTDTNYYSEKGFPKSCCKL
///             EDCTPQRDADKVNNEGCFIKVMTIIESEMGVVAGISFGVACFQLIGIFLAYCLSRAITNN
///             QYEIV
/// NTSEQ       738
///             atggcgtccccgtctcggagactgcagactaaaccagtcattacttgtttcaagagcgtt
///             ctgctaatctacacttttattttctggatcactggcgttatccttcttgcagttggcatt
///             tggggcaaggtgagcctggagaattacttttctcttttaaatgagaaggccaccaatgtc
///             cccttcgtgctcattgctactggtaccgtcattattcttttgggcacctttggttgtttt
///             gctacctgccgagcttctgcatggatgctaaaactgtatgcaatgtttctgactctcgtt
///             tttttggtcgaactggtcgctgccatcgtaggatttgttttcagacatgagattaagaac
///             agctttaagaataattatgagaaggctttgaagcagtataactctacaggagattataga
///             agccatgcagtagacaagatccaaaatacgttgcattgttgtggtgtcaccgattataga
///             gattggacagatactaattattactcagaaaaaggatttcctaagagttgctgtaaactt
///             gaagattgtactccacagagagatgcagacaaagtaaacaatgaaggttgttttataaag
///             gtgatgaccattatagagtcagaaatgggagtcgttgcaggaatttcctttggagttgct
///             tgcttccaactgattggaatctttctcgcctactgcctctctcgtgccataacaaataac
///             cagtatgagatagtgtaa
/// ///
/// ";
///
/// let runtime = actix_web::rt::Runtime::new().unwrap();
/// assert_eq!(
/// runtime.block_on(async {
///     let client: awc::Client = awc::Client::default();
///     let body: String = utils::web::get_html_body(&client, "https://rest.kegg.jp/get/hsa:TSPAN6").await.unwrap();
///     body
///  }), expected);
/// ```
pub async fn get_html_body(client: &awc::Client, url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut client_response = client.get(url).send().await?;
    let response_body = client_response.body().await?;
    let body = std::str::from_utf8(&response_body)?;
    Ok(String::from(body))
}

/// Build and return an Actix Web Client with DER encoded X.509 certificates.
/// 
/// # Example
///  
/// ```
/// let first_certificate_contents: &str = "
/// -----BEGIN CERTIFICATE-----
/// MIIF3jCCA8agAwIBAgIQAf1tMPyjylGoG7xkDjUDLTANBgkqhkiG9w0BAQwFADCB
/// iDELMAkGA1UEBhMCVVMxEzARBgNVBAgTCk5ldyBKZXJzZXkxFDASBgNVBAcTC0pl
/// cnNleSBDaXR5MR4wHAYDVQQKExVUaGUgVVNFUlRSVVNUIE5ldHdvcmsxLjAsBgNV
/// BAMTJVVTRVJUcnVzdCBSU0EgQ2VydGlmaWNhdGlvbiBBdXRob3JpdHkwHhcNMTAw
/// MjAxMDAwMDAwWhcNMzgwMTE4MjM1OTU5WjCBiDELMAkGA1UEBhMCVVMxEzARBgNV
/// BAgTCk5ldyBKZXJzZXkxFDASBgNVBAcTC0plcnNleSBDaXR5MR4wHAYDVQQKExVU
/// aGUgVVNFUlRSVVNUIE5ldHdvcmsxLjAsBgNVBAMTJVVTRVJUcnVzdCBSU0EgQ2Vy
/// dGlmaWNhdGlvbiBBdXRob3JpdHkwggIiMA0GCSqGSIb3DQEBAQUAA4ICDwAwggIK
/// AoICAQCAEmUXNg7D2wiz0KxXDXbtzSfTTK1Qg2HiqiBNCS1kCdzOiZ/MPans9s/B
/// 3PHTsdZ7NygRK0faOca8Ohm0X6a9fZ2jY0K2dvKpOyuR+OJv0OwWIJAJPuLodMkY
/// tJHUYmTbf6MG8YgYapAiPLz+E/CHFHv25B+O1ORRxhFnRghRy4YUVD+8M/5+bJz/
/// Fp0YvVGONaanZshyZ9shZrHUm3gDwFA66Mzw3LyeTP6vBZY1H1dat//O+T23LLb2
/// VN3I5xI6Ta5MirdcmrS3ID3KfyI0rn47aGYBROcBTkZTmzNg95S+UzeQc0PzMsNT
/// 79uq/nROacdrjGCT3sTHDN/hMq7MkztReJVni+49Vv4M0GkPGw/zJSZrM233bkf6
/// c0Plfg6lZrEpfDKEY1WJxA3Bk1QwGROs0303p+tdOmw1XNtB1xLaqUkL39iAigmT
/// Yo61Zs8liM2EuLE/pDkP2QKe6xJMlXzzawWpXhaDzLhn4ugTncxbgtNMs+1b/97l
/// c6wjOy0AvzVVdAlJ2ElYGn+SNuZRkg7zJn0cTRe8yexDJtC/QV9AqURE9JnnV4ee
/// UB9XVKg+/XRjL7FQZQnmWEIuQxpMtPAlR1n6BB6T1CZGSlCBst6+eLf8ZxXhyVeE
/// Hg9j1uliutZfVS7qXMYoCAQlObgOK6nyTJccBz8NUvXt7y+CDwIDAQABo0IwQDAd
/// BgNVHQ4EFgQUU3m/WqorSs9UgOHYm8Cd8rIDZsswDgYDVR0PAQH/BAQDAgEGMA8G
/// A1UdEwEB/wQFMAMBAf8wDQYJKoZIhvcNAQEMBQADggIBAFzUfA3P9wF9QZllDHPF
/// Up/L+M+ZBn8b2kMVn54CVVeWFPFSPCeHlCjtHzoBN6J2/FNQwISbxmtOuowhT6KO
/// VWKR82kV2LyI48SqC/3vqOlLVSoGIG1VeCkZ7l8wXEskEVX/JJpuXior7gtNn3/3
/// ATiUFJVDBwn7YKnuHKsSjKCaXqeYalltiz8I+8jRRa8YFWSQEg9zKC7F4iRO/Fjs
/// 8PRF/iKz6y+O0tlFYQXBl2+odnKPi4w2r78NBc5xjeambx9spnFixdjQg3IM8WcR
/// iQycE0xyNN+81XHfqnHd4blsjDwSXWXavVcStkNr/+XeTWYRUc+ZruwXtuhxkYze
/// Sf7dNXGiFSeUHM9h4ya7b6NnJSFd5t0dCy5oGzuCr+yDZ4XUmFF0sbmZgIn/f3gZ
/// XHlKYC6SQK5MNyosycdiyA5d9zZbyuAlJQG03RoHnHcAP9Dc1ew91Pq7P8yF1m9/
/// qS3fuQL39ZeatTXaw2ewh0qpKJ4jjv9cJ2vhsE/zB+4ALtRZh8tSQZXq9EfX7mRB
/// VXyNWQKV3WKdwrnuWih0hKWbt5DHDAff9Yk2dDLWKMGwsAvgnEzDHNb842m1R0aB
/// L6KCq9NjRHDEjf8tM7qtj3u1cIiuPhnPQCjY/MiQu12ZIvVS5ljFH4gxQ+6IHdfG
/// jjxDah2nGN59PRbxYvnKkKj9
/// -----END CERTIFICATE-----";
/// 
/// let second_certificate_contents: &str = "
/// -----BEGIN CERTIFICATE-----
/// MIIDxTCCAq2gAwIBAgIBADANBgkqhkiG9w0BAQsFADCBgzELMAkGA1UEBhMCVVMx
/// EDAOBgNVBAgTB0FyaXpvbmExEzARBgNVBAcTClNjb3R0c2RhbGUxGjAYBgNVBAoT
/// EUdvRGFkZHkuY29tLCBJbmMuMTEwLwYDVQQDEyhHbyBEYWRkeSBSb290IENlcnRp
/// ZmljYXRlIEF1dGhvcml0eSAtIEcyMB4XDTA5MDkwMTAwMDAwMFoXDTM3MTIzMTIz
/// NTk1OVowgYMxCzAJBgNVBAYTAlVTMRAwDgYDVQQIEwdBcml6b25hMRMwEQYDVQQH
/// EwpTY290dHNkYWxlMRowGAYDVQQKExFHb0RhZGR5LmNvbSwgSW5jLjExMC8GA1UE
/// AxMoR28gRGFkZHkgUm9vdCBDZXJ0aWZpY2F0ZSBBdXRob3JpdHkgLSBHMjCCASIw
/// DQYJKoZIhvcNAQEBBQADggEPADCCAQoCggEBAL9xYgjx+lk09xvJGKP3gElY6SKD
/// E6bFIEMBO4Tx5oVJnyfq9oQbTqC023CYxzIBsQU+B07u9PpPL1kwIuerGVZr4oAH
/// /PMWdYA5UXvl+TW2dE6pjYIT5LY/qQOD+qK+ihVqf94Lw7YZFAXK6sOoBJQ7Rnwy
/// DfMAZiLIjWltNowRGLfTshxgtDj6AozO091GB94KPutdfMh8+7ArU6SSYmlRJQVh
/// GkSBjCypQ5Yj36w6gZoOKcUcqeldHraenjAKOc7xiID7S13MMuyFYkMlNAJWJwGR
/// tDtwKj9useiciAF9n9T521NtYJ2/LOdYq7hfRvzOxBsDPAnrSTFcaUaz4EcCAwEA
/// AaNCMEAwDwYDVR0TAQH/BAUwAwEB/zAOBgNVHQ8BAf8EBAMCAQYwHQYDVR0OBBYE
/// FDqahQcQZyi27/a9BUFuIMGU2g/eMA0GCSqGSIb3DQEBCwUAA4IBAQCZ21151fmX
/// WWcDYfF+OwYxdS2hII5PZYe096acvNjpL9DbWu7PdIxztDhC2gV7+AJ1uP2lsdeu
/// 9tfeE8tTEH6KRtGX+rcuKxGrkLAngPnon1rpN5+r5N9ss4UXnT3ZJE95kTXWXwTr
/// gIOrmgIttRD02JDHBHNA7XIloKmf7J6raBKZV8aPEjoJpL1E/QYVN8Gb5DKj7Tjo
/// 2GTzLH4U/ALqn83/B2gX2yKQOC16jdFU8WnjXzPKej17CuPKf1855eJ1usV2GDPO
/// LPAvTK33sefOT6jEm0pUBsV/fdUID+Ic/n4XuKxe9tQWskMJDE32p2u0mYRlynqI
/// 4uJEvlz36hz1
/// -----END CERTIFICATE-----";
/// 
/// let runtime = actix_web::rt::Runtime::new().unwrap();
/// assert_eq!(
/// runtime.block_on(async {
///     use std::io::Write;
///     let mut first_certificate_file: tempfile::NamedTempFile = tempfile::NamedTempFile::new().unwrap();
///     let mut second_certificate_file: tempfile::NamedTempFile = tempfile::NamedTempFile::new().unwrap();
///     first_certificate_file.write_all(first_certificate_contents.as_bytes());
///     second_certificate_file.write_all(second_certificate_contents.as_bytes());
///     let certificate_files: Vec<&std::path::Path> = vec![first_certificate_file.path(), second_certificate_file.path()];
///     let client: awc::Client = utils::web::new_client(certificate_files);
///     let response = client.get("https://api.genome.ucsc.edu/getData/sequence?genome=hg38;chrom=chrX;start=1;end=2").send().await.unwrap();
///     first_certificate_file.close();
///     second_certificate_file.close();
///     response.status()
/// }), 200);
/// ```
pub fn new_client<'a, T>(certificate_files: T) -> awc::Client
where
    T: IntoIterator<Item = &'a std::path::Path>,
    T::IntoIter: 'a,
{
    let mut root_store: rustls::RootCertStore = rustls::RootCertStore::empty();

    //  For each certificate file provided,
    for certificate_file in certificate_files {
        //  Read the file,
        match std::fs::read(certificate_file) {
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
                    certificate_file.display(), error
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
