## EnsEMBL Search
EnsEMBL search generates a CSV file (comma delimited) of EnsEMBL identifier entries.

<ins>Usage:</ins> **ensembl_search** [OPTIONS] **--certificate** <certificates> **--index** <index> **--file** <file> **--output** <output>

<ins>Options:</ins>

  **-c**, **--certificate** <certificates>  A DER-encoded X.509 file

  **-i**, **--index** <index>               A column index to take the set of values
 
  **-d**, **--delimiter** <delimiter>       The delimiter character that separates each field value (e.g. ',', ';', '\t')
 
  **-f**, **--file** <file>                 The flat file (e.g. CSV, TSV) file path to parse for identifiers
 
  **-n**, **--no-headers**                  A flag that indicates no header row is present
 
  **-O**, **--output** <output>             The output file name and path to write a CSV file.

  **-h**, **--help**                        Print help information

  **-V**, **--version**                     Print version information
### Example
Given the following file `/home/user/data/csv/gene_expressions.csv` with the following entries:

| Gene name | Gene | Tissue region | Transcripts per million |
| --- | --- | --- | --- |
|CLHC1|ENSG00000162994|cerebral cortex|0.9|
|CLHC1|ENSG00000162994|basal ganglia|1.4|
|CLHC1|ENSG00000162994|hippocampal formation|1.5|
|SLC19A2|ENSG00000117479|hippocampal formation|2.0|
|SLC19A2|ENSG00000117479|cerebral cortex|3.5|
|SETD9|ENSG00000155542|midbrain|0.7|
|...|...|...|

The following command will return a list of EnsEMBL identifiers:

```
ensembl_search \
--file "/home/user/data/csv/gene_expressions.csv" \
--index 1 \
--certificate "/home/user/data/certificates/authorities.pem" \
--certificate "/home/user/data/certificates/additional_authorities.pem" \
--output "/home/user/data/csv/EnsEMBL_entries.csv"
```

| assembly_name | biotype | canonical_transcript | db_type | description | display_name | dna | end | id | logic_name | object_type | seq_region_name | source | species | start | strand | version |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
|GRCh38|protein_coding|ENST00000285947.5|core|SET domain containing 9 [Source:HGNC Symbol;Acc:HGNC:28508]|SETD9|GACAGCCGT...|56925532|ENSG00000155542|ensembl_havana_gene_homo_sapiens|Gene|5|ensembl_havana|homo_sapiens|56909260|1|12|
|GRCh38|protein_coding|ENST00000401408.6|core|clathrin heavy chain linker domain containing 1 [Source:HGNC Symbol;Acc:HGNC:26453]|CLHC1|TTTTTATGT...|55232563|ENSG00000162994|ensembl_havana_gene_homo_sapiens|Gene|2|ensembl_havana|homo_sapiens|55172547|-1|16
|GRCh38|protein_coding|ENST00000236137.10|core|solute carrier family 19 member 2 [Source:HGNC Symbol;Acc:HGNC:10938]|SLC19A2|TTTGATTAA...|169485944|ENSG00000117479|ensembl_havana_gene_homo_sapiens|Gene|1|ensembl_havana|homo_sapiens|169463909|-1|15|

## Identifiers
Identifiers outputs to standard output a set of identifiers from a column present in a flat file (e.g. CSV, TSV).

<ins>Usage:</ins> **identifiers** [OPTIONS] **--index** <index> **--file** <file>

<ins>Options:</ins>

  **-i**, **--index** <index>          A column index to take the set of values

  **-d**, **--delimiter** <delimiter>  The delimiter character that separates each field value (e.g. ',', ';', '\t')
 
  **-f**, **--file** <file>            The flat file (e.g. CSV, TSV) file path to parse for identifiers
 
  **-n**, **--no-headers**             A flag that indicates no header row is present
  
  **-h**, **--help**                   Print help information

  **-V**, **--version**                Print version information
### Example
Given the following file `/home/user/data/csv/gene_expressions.csv` with the following entries,

| Gene name | Gene | Tissue region | Transcripts per million |
| --- | --- | --- | --- |
|CLHC1|ENSG00000162994|cerebral cortex|0.9|
|CLHC1|ENSG00000162994|basal ganglia|1.4|
|CLHC1|ENSG00000162994|hippocampal formation|1.5|
|SLC19A2|ENSG00000117479|hippocampal formation|2.0|
|SLC19A2|ENSG00000117479|cerebral cortex|3.5|
|SETD9|ENSG00000155542|midbrain|0.7|

The following command,

```
identifiers \
--file "/home/user/data/csv/gene_expressions.csv" \
--index 1
```

will output EnsEMBL identifiers to standard output:

```
ENSG00000162994
ENSG00000117479
ENSG0000015554
```