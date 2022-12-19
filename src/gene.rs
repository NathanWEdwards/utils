//! MIT license.

use serde::ser::SerializeStruct;

const ENSEMBL_BASE_URL: &str = "https://rest.ensembl.org";
const UCSC_GENOME_BASE_URL: &str = "https://api.genome.ucsc.edu";

pub struct Gene {
    assembly_name: String,
    biotype: String,
    canonical_transcript: String,
    db_type: String,
    description: String,
    display_name: String,
    dna: String,
    end: String,
    id: String,
    logic_name: String,
    object_type: String,
    seq_region_name: String,
    source: String,
    species: String,
    start: String,
    strand: String,
    version: String,
}

impl serde::ser::Serialize for Gene {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let mut serialized_gene = serializer.serialize_struct("Gene", 17)?;
        serialized_gene.serialize_field("assembly_name", &self.assembly_name)?;
        serialized_gene.serialize_field("biotype", &self.biotype)?;
        serialized_gene.serialize_field("canonical_transcript", &self.canonical_transcript)?;
        serialized_gene.serialize_field("db_type", &self.db_type)?;
        serialized_gene.serialize_field("description", &self.description)?;
        serialized_gene.serialize_field("display_name", &self.display_name)?;
        serialized_gene.serialize_field("dna", &self.dna)?;
        serialized_gene.serialize_field("end", &self.end)?;
        serialized_gene.serialize_field("id", &self.id)?;
        serialized_gene.serialize_field("logic_name", &self.logic_name)?;
        serialized_gene.serialize_field("object_type", &self.object_type)?;
        serialized_gene.serialize_field("seq_region_name", &self.seq_region_name)?;
        serialized_gene.serialize_field("source", &self.source)?;
        serialized_gene.serialize_field("species", &self.species)?;
        serialized_gene.serialize_field("start", &self.start)?;
        serialized_gene.serialize_field("strand", &self.strand)?;
        serialized_gene.serialize_field("version", &self.version)?;
        serialized_gene.end()
    }
}

pub async fn ensembl_search(
    client: &awc::Client,
    ensembl_id: &String,
    query_dna: bool,
) -> Result<Gene, Box<dyn std::error::Error>> {
    let mut resource: String = String::from(ENSEMBL_BASE_URL);
    resource.push_str("/lookup/id/");
    resource.push_str(ensembl_id);

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
                true => dna_search(client, &assembly_name, &start, &end, &seq_region_name)
                    .await
                    .unwrap_or(String::from("")),
                false => String::from(""),
            };

            Ok(Gene {
                assembly_name,
                biotype,
                canonical_transcript,
                db_type,
                description,
                display_name,
                dna,
                end,
                id,
                logic_name,
                object_type,
                seq_region_name,
                source,
                species,
                start,
                strand,
                version,
            })
        }
        Err(error) => {
            eprintln!("{{\"id\":\"{}\", \"error\":\"{}\"}}", ensembl_id, error);
            Err(Box::new(error))
        }
    }
}

pub async fn dna_search(
    client: &awc::Client,
    assembly_name: &String,
    start: &String,
    end: &String,
    chromosome: &String,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut resource = String::from(UCSC_GENOME_BASE_URL);
    resource.push_str("/getData/sequence?");

    match &assembly_name[..] {
        "GRCh38" => resource.push_str("genome=hg38;"),
        _ => eprintln!("Unknown assembly."),
    }

    resource.push_str("chrom=chr");
    resource.push_str(&chromosome[..]);
    resource.push(';');

    resource.push_str("start=");
    resource.push_str(&start[..]);
    resource.push(';');

    resource.push_str("end=");
    resource.push_str(&end[..]);
    resource.push(';');

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

    let json_result: Result<serde_json::Value, serde_json::Error> =
        serde_json::from_str(&json[..]);

    match json_result {
        Ok(json) => {
            let dna = String::from(json["dna"].as_str().unwrap_or(""));
            Ok(dna)
        }
        Err(error) => {
            eprintln!(
                "{{\"assembly_name\":\"{}\", \"start\":{}, \"end\":{}, \"error\":\"{}\"}}",
                assembly_name, start, end, error
            );
            Ok(String::from(""))
        }
    }
}
