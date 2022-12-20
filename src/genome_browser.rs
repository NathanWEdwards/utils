//! MIT license.

const ENSEMBL_BASE_URL: &str = "https://rest.ensembl.org";
const UCSC_GENOME_BASE_URL: &str = "https://api.genome.ucsc.edu";
pub struct GenomeBrowserResponse {
    pub gene: crate::gene::Gene,
    pub timestamp: std::time::SystemTime
}

/// Search genome browsers for gene information.
/// 
/// # Example
/// 
/// ```
/// let runtime = actix_web::rt::Runtime::new().unwrap();
/// assert!(
/// runtime.block_on(async {
///     let client: awc::Client = awc::Client::default();
///     let ensembl_id: String = String::from("ENSG00000155542");
///     let query_for_DNA_please: bool = true;
///     let last_request_made: Option<std::time::SystemTime> = None;
///     let number_of_requests_made: Option<u32> = Some(0);
///     let genome_browser_response = utils::genome_browser::ensembl_search(&client,
///                                                               &ensembl_id,
///                                                               query_for_DNA_please,
///                                                               last_request_made,
///                                                               number_of_requests_made).await.unwrap();
///     String::from(genome_browser_response.gene.get_display_name())
///  }).eq(&String::from("SETD9")));
/// ```
pub async fn ensembl_search(
    client: &awc::Client,
    ensembl_id: &str,
    query_dna: bool,
    last_request_made: Option<std::time::SystemTime>,
    number_of_requests_made: Option<u32>
) -> Result<GenomeBrowserResponse, Box<dyn std::error::Error>> {
    let resource: String = format!("{ENSEMBL_BASE_URL}/lookup/id/{ensembl_id}");

    // A maximum of 5_000 requests can be made per day.
    // If the number of requests made is supplied and exceeds 5_000 then exit.
    if number_of_requests_made.is_some() && number_of_requests_made.unwrap() >= 5_000 {
        eprintln!("{{\"error\": \"The EnsEMBL maximum number of requests (5,000) has been exceeded.\"}}");
        return Err(Box::new(std::io::Error::from(std::io::ErrorKind::ConnectionRefused)));
    }

    // A maximum of one request can be made to the EnsEMBL Genome Browser approximately every 67 milliseconds.
    // Wait 67 milliseconds if the time the last request is made is supplied.
    if last_request_made.is_some() {
        crate::time::sleep_until_time_elapsed(last_request_made.unwrap(), 67 as u64);
    }

    let html = match crate::web::get_html_body(client, &resource[..]).await {
        Ok(html) => html,
        Err(error) => {
            eprintln!("{{\"id\":\"{}\", \"error\":\"{}\"}}", ensembl_id, error);
            String::from("")
        }
    };

    let parsed = crate::html::parse_pre_elements(&html[..]);

    let ensembl_yaml: Result<serde_yaml::Value, serde_yaml::Error> =
        serde_yaml::from_str(&parsed[..]);

    match ensembl_yaml {
        Ok(yaml) => {
            let assembly_name = String::from(yaml["assembly_name"].as_str().unwrap_or(""));
            let biotype = String::from(yaml["biotype"].as_str().unwrap_or(""));
            let canonical_transcript =
                String::from(yaml["canonical_transcript"].as_str().unwrap_or(""));
            let db_type = String::from(yaml["db_type"].as_str().unwrap_or(""));
            let description = String::from(yaml["description"].as_str().unwrap_or(""));
            let display_name = String::from(yaml["display_name"].as_str().unwrap_or(""));
            let id = String::from(yaml["id"].as_str().unwrap_or(""));
            let logic_name = String::from(yaml["logic_name"].as_str().unwrap_or(""));
            let object_type = String::from(yaml["object_type"].as_str().unwrap_or(""));
            let source = String::from(yaml["source"].as_str().unwrap_or(""));
            let species = String::from(yaml["species"].as_str().unwrap_or(""));

            let end = match yaml["end"].as_u64() {
                Some(number) => number.to_string(),
                None => String::from(""),
            };
            let start = match yaml["start"].as_u64() {
                Some(number) => number.to_string(),
                None => String::from(""),
            };
            let version = match yaml["version"].as_u64() {
                Some(number) => number.to_string(),
                None => String::from(""),
            };

            let seq_region_name: String;
            if yaml["seq_region_name"].is_number() {
                seq_region_name = match yaml["seq_region_name"].as_u64() {
                    Some(number) => number.to_string(),
                    None => String::from(""),
                }
            } else if yaml["seq_region_name"].is_string() {
                seq_region_name = String::from(yaml["seq_region_name"].as_str().unwrap_or(""))
            } else {
                seq_region_name = String::from("");
            }

            let strand: String;
            if yaml["strand"].is_number() {
                strand = match yaml["strand"].as_i64() {
                    Some(number) => number.to_string(),
                    None => String::from(""),
                }
            } else if yaml["strand"].is_string() {
                strand = String::from(yaml["strand"].as_str().unwrap_or(""))
            } else {
                strand = String::from("");
            }

            let dna = match query_dna {
                true => {
                    let json: String = ucsc_genome_browser_search(client, &assembly_name, &start, &end, &seq_region_name, last_request_made)
                    .await
                    .unwrap_or(String::from(""));

                    let json_result: Result<serde_json::Value, serde_json::Error> =
                    serde_json::from_str(&json[..]);

                    let dna = match json_result {
                        Ok(json) => {
                            let dna = String::from(json["dna"].as_str().unwrap_or(""));
                            dna
                        }
                        Err(error) => {
                            eprintln!(
                                "{{\"assembly_name\":\"{}\", \"start\":{}, \"end\":{}, \"error\":\"{}\"}}",
                                assembly_name, start, end, error
                            );
                            String::from("")
                        }
                    };

                    dna
                },
                false => String::from(""),
            };

            Ok(GenomeBrowserResponse {
                gene: crate::gene::Gene::new (
                &assembly_name,
                &biotype,
                &canonical_transcript,
                &db_type,
                &description,
                &display_name,
                &dna,
                &end,
                &id,
                &logic_name,
                &object_type,
                &seq_region_name,
                &source,
                &species,
                &start,
                &strand,
                &version,
                ),
                timestamp: std::time::SystemTime::now()
            })
        }
        Err(error) => {
            eprintln!("{{\"id\":\"{}\", \"error\":\"{}\"}}", ensembl_id, error);
            Err(Box::new(error))
        }
    }
}

/// Search the University of California Santa Cruz Genome Browser for DNA strings.
/// 
/// # Example
/// 
/// ```
/// let runtime = actix_web::rt::Runtime::new().unwrap();
/// assert!(
/// runtime.block_on(async {
///     let client: awc::Client = awc::Client::default();
///     let assembly_name = "hg38";
///     let start = "1";
///     let end = "2";
///     let chromosome = "X";
///     let last_request_made: Option<std::time::SystemTime> = None;
///     let ucsc_genome_browser_response: String = utils::genome_browser::ucsc_genome_browser_search(&client,
///                                                                                assembly_name,
///                                                                                start,
///                                                                                end,
///                                                                                chromosome,
///                                                                                last_request_made).await.unwrap();
///     let json: serde_json::Value = serde_json::from_str(&ucsc_genome_browser_response).unwrap();
///     json["start"].as_u64().unwrap_or(0 as u64)
///  }) == 1);
/// ```
pub async fn ucsc_genome_browser_search(
    client: &awc::Client,
    assembly_name: &str,
    start: &str,
    end: &str,
    chromosome: &str,
    last_request_made: Option<std::time::SystemTime>
) -> Result<String, Box<dyn std::error::Error>> {

    let genome: &str = match assembly_name {
        "GRCh38" => "hg38",
        "GRCh37" => "hg19",
        genome => genome
    };

    let resource = format!("{UCSC_GENOME_BASE_URL}/getData/sequence?genome={genome};chrom=chr{chromosome};start={start};end={end};");

    // A maximum of one request can be made to the University of California Santa Cruz Genome Browser approximately every 15_000 milliseconds (15 seconds).
    // Wait 15_000 milliseconds (15 seconds) if the time the last request is made is supplied.
    if last_request_made.is_some() {
        crate::time::sleep_until_time_elapsed(last_request_made.unwrap(), 15_000 as u64);
    }

    let json = match crate::web::get_html_body(client, &resource[..]).await {
        Ok(html) => html,
        Err(error) => {
            eprintln!(
                "{{\"url\": \"{}\", \"assembly_name\":\"{}\", \"start\": \"{}, \"end\":{}, \"error\":\"{}\"}}",
                resource, assembly_name, start, end, error
            );
            String::from("")
        }
    };

    Ok(json)
}
