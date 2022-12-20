//! MIT license.

use serde::ser::SerializeStruct;

#[derive(Clone)]
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

impl Gene {
    pub fn new(
        assembly_name: &str,
        biotype: &str,
        canonical_transcript: &str,
        db_type: &str,
        description: &str,
        display_name: &str,
        dna: &str,
        end: &str,
        id: &str,
        logic_name: &str,
        object_type: &str,
        seq_region_name: &str,
        source: &str,
        species: &str,
        start: &str,
        strand: &str,
        version: &str,
    ) -> Gene {
        Gene {
            assembly_name: String::from(assembly_name),
            biotype: String::from(biotype),
            canonical_transcript: String::from(canonical_transcript),
            db_type: String::from(db_type),
            description: String::from(description),
            display_name: String::from(display_name),
            dna: String::from(dna),
            end: String::from(end),
            id: String::from(id),
            logic_name: String::from(logic_name),
            object_type: String::from(object_type),
            seq_region_name: String::from(seq_region_name),
            source: String::from(source),
            species: String::from(species),
            start: String::from(start),
            strand: String::from(strand),
            version: String::from(version)
        }
    }

    pub fn get_assembly_name(&self) -> &str {
        return &self.assembly_name;
    }

    pub fn get_biotype(&self) -> &str {
        return &self.biotype;
    }

    pub fn get_canonical_transcript(&self) -> &str {
        return &self.canonical_transcript;
    }

    pub fn get_db_type(&self) -> &str {
        return &self.db_type;
    }

    pub fn get_description(&self) -> &str {
        return &self.description;
    }

    pub fn get_display_name(&self) -> &str {
        return &self.display_name;
    }

    pub fn get_dna(&self) -> &str {
        return &self.dna;
    }

    pub fn get_end(&self) -> &str {
        return &self.end;
    }

    pub fn get_id(&self) -> &str {
        return &self.id;
    }

    pub fn get_logic_name(&self) -> &str {
        return &self.logic_name;
    }

    pub fn get_object_type(&self) -> &str {
        return &self.object_type;
    }

    pub fn get_seq_region_name(&self) -> &str {
        return &self.seq_region_name;
    }

    pub fn get_source(&self) -> &str {
        return &self.source;
    }

    pub fn get_species(&self) -> &str {
        return &self.species;
    }

    pub fn get_start(&self) -> &str {
        return &self.start;
    }

    pub fn get_strand(&self) -> &str {
        return &self.strand;
    }

    pub fn get_version(&self) -> &str {
        return &self.version;
    }

    pub async fn lookup(
        client: Option<&awc::Client>,
        ensembl_id: &String,
        query_dna: bool,
        last_request_made: Option<std::time::SystemTime>,
        number_of_requests_made: Option<u32>
    ) -> Result<Gene, Box<dyn std::error::Error>> {
        let genome_browser_result = match client {
            Some(awc_client) => {
                crate::genome_browser::ensembl_search(awc_client,
                    ensembl_id,
                    query_dna,
                    last_request_made,
                    number_of_requests_made).await
            },
            None => {
                crate::genome_browser::ensembl_search(&awc::Client::default(),
                    ensembl_id,
                    query_dna,
                    last_request_made,
                    number_of_requests_made).await
            }
        };

        match genome_browser_result {
            Ok(result) => Ok(result.gene),
            Err(error) => Err(error)
        }
    }
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
