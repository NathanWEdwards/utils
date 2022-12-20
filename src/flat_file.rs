//! MIT license.

/// Read a column from a flat file (e.g. CSV, TSV) and return a vector of strings.
/// 
/// # Example
/// 
/// ```
/// use std::io::Write;
/// 
/// let expected: Vec<&str> = vec!["Mar. 2018", "Dec. 2015", "Nov. 2011", "Jul. 2009", "Jan. 2020"];
/// let csv_contents = "SPECIES\tUCSC VERSION\tRELEASE DATE\tRELEASE NAME\tSTATUS\n
/// Chicken\tgalGal6\tMar. 2018\tGRCg6 Gallus-gallus-6.0\tAvailable\n
/// Chicken\tgalGal5\tDec. 2015\tICGC Gallus-gallus-5.0\tAvailable\n
/// Chicken\tgalGal4\tNov. 2011\tICGC Gallus-gallus-4.0\tAvailable\n
/// Elephant\tloxAfr3\tJul. 2009\tBroad Institute LoxAfr3\tAvailable\n
/// SARS-CoV-2\twuhCor1\tJan. 2020\tSARS-CoV-2 ASM985889v3\tAvailable";
/// 
/// let mut csv_file : tempfile::NamedTempFile = tempfile::NamedTempFile::new().unwrap();
/// csv_file.write_all(csv_contents.as_bytes());
/// let csv_entries = utils::flat_file::read_column(csv_file.path(), b'\t', true, 2).unwrap();
/// csv_file.close();
/// assert_eq!(csv_entries, expected);
/// ```
pub fn read_column(filename: &std::path::Path, delimiter: u8, has_headers: bool, index: usize) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    // Values to return.
    let mut ret = Vec::<String>::new();

    let reader = csv::ReaderBuilder::new()
        .delimiter(delimiter)
        .has_headers(has_headers)
        .from_path(filename);

    // Assume the index is valid.
    for row in reader?.records() {
        let record = row?;
        ret.push(record[index].to_string());
    }

    return Ok(ret);
}