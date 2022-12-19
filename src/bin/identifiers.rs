//! MIT license.
//! Provides a set of the values present in a column of a flat file.

/// Returns a list of the values from a flat file (e.g. CSV).
fn main() {
    let argument_matches = cli().get_matches();

    let file = match argument_matches.get_one::<String>("file") {
        Some(file) => file,
        _ => unreachable!("required(true) prevents `None`.")
    };
    
    if std::path::Path::new(file).exists() != true {
        eprintln!("file error: {}", std::io::Error::from(std::io::ErrorKind::NotFound));
        std::process::exit(1);
    }

    let has_headers = !argument_matches.get_flag("no_headers");

    let index = match argument_matches.get_one::<String>("index") {
        Some(index) => match index.parse::<usize>() {
                Ok(value) => value,
                Err(_) => {
                        eprintln!("index error: {}", std::io::Error::from(std::io::ErrorKind::InvalidInput));
                        std::process::exit(1);
                }
        },
        _ => unreachable!("required(true) prevent `None`.")
    };

    let delimiter = match argument_matches.get_one::<String>("delimiter") {
        Some(delimiter) => delimiter.as_bytes()[0],
        None => b','
    };
    
    match utils::flat_file::read(file, delimiter, has_headers, index) {
        Ok(list) => {
            let set: std::collections::HashSet<String> = std::collections::HashSet::from_iter(list);
            for element in set {
                println!("{}", element);
            }
        },
        Err(error) => {
            eprintln!("file read error: {}", error);
            std::process::exit(1);
        }
    }
}

fn cli() -> clap::Command {
        clap::Command::new("identifiers")
        .version("0.1.0")
        .author("Nathan Edwards <Nathan.W.Edwards@Outlook.com>")
        .about("Outputs to standard output a set of identifiers from a column present in a flat file (e.g. CSV, TSV)")
        .arg(clap::Arg::new("index")
                .required(true)
                .short('i')
                .long("index")
                .help("A column index to take the set of values"))
        .arg(clap::Arg::new("delimiter")
                .short('d')
                .long("delimiter")
                .help("The delimiter character that separates each field value (e.g. ',', ';', '\\t')"))
        .arg(clap::Arg::new("file")
                .required(true)
                .short('f')
                .long("file")
                .help("The flat file (e.g. CSV, TSV) file path to parse for identifiers"))
        .arg(clap::Arg::new("no_headers")
                .short('n')
                .long("no-headers")
                .action(clap::ArgAction::SetTrue)
                .help("A flag that indicates no header row is present"))
}