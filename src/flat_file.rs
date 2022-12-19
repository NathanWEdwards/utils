//! MIT license.

pub fn read(filename: &String, delimiter: u8, has_headers: bool, index: usize) -> Result<Vec<String>, Box<dyn std::error::Error>> {
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