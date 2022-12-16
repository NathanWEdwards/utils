//! Provides a set of Ensembl gene identifiers present in a column of a flat file.

/// Returns a list of Ensembl identifiers from a flat file (e.g. CSV).
fn main() {
    let argument_matches = clap::App::new("identifiers")
        .version("0.1.0")
        .author("Nathan Edwards <Nathan.W.Edwards@Outlook.com>")
        .about("Return a set of Ensembl identifiers from a column present in a flat file (e.g. CSV, TSV).")
        .arg(clap::Arg::with_name("column")
                .short("c")
                .long("column")
                .takes_value(true)
                .help("The name of the column, or row index, that possesses Ensembl identifiers."))
        .arg(clap::Arg::with_name("delimiter")
                .short("d")
                .long("delimiter")
                .takes_value(true)
                .help("The delimiter character or string token that seperates each field value (e.g. \",\", \";\", '\t')."))
        .arg(clap::Arg::with_name("file")
                .short("f")
                .long("file")
                .takes_value(true)
                .help("The flat file (e.g. CSV, TSV) file path to parse for Ensemble identifiers."))
        .arg(clap::Arg::with_name("headers")
                .short("h")
                .long("headers")
                .takes_value(true)
                .help("States if the supplied file has a header row or not. This file expects \"yes\", \"true\", or a non-zero integer if the intended file to parse has a header. If no header is present, this file expects \"no\", \"false\", or 0 (zero)."))        
        .get_matches()
}


fn read(filename: String, delimeter: Option<String>, has_headers: Option<bool>) -> Result<(), Box<dyn Error>> {
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(delimiter.unwrap_or(',').as_bytes())
        .has_headers(has_headers.unwrap_or(true))
        .from_path(filename)
}