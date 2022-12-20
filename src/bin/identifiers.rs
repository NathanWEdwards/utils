//!  MIT license.
//!  Provides a set of the values present in a column of a flat file.

///  Print a list of the unique column values from a flat file (e.g. CSV).
fn main() {
    //  Get arguments from the command line.
    let argument_matches = cli().get_matches();

    //  Assign the filename passed as an argument from the command line to a variable.
    //  The input filename argument is required, there should always be an input filename present as an argument because of the clap crate's required(true) implementation.
    let file = match argument_matches.get_one::<String>("file") {
        Some(file) => file,
        _ => unreachable!("required(true) prevents `None`.")
    };
    
    //  The input file does not exist print to standard error and exit with an error code (1).
    if std::path::Path::new(file).exists() != true {
        eprintln!("file error: {}", std::io::Error::from(std::io::ErrorKind::NotFound));
        std::process::exit(1);
    }

    //  Assign the flag value of no_headers to a variable.
    //  If no_headers is not set, no_headers will default to false, and this, by default, implies the input file has headers.
    let has_headers = !argument_matches.get_flag("no_headers");

    //  Assign the index argument value to a variable.
    //  The index value will always be defined because of the clap crate's required(true) implementation. The index argument is required.
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

    //  Assign the delimiter argument to a variable.
    //  If no delimiter argument is supplied through the command line, assume the file delimiter is a comma.
    let delimiter = match argument_matches.get_one::<String>("delimiter") {
        Some(delimiter) => delimiter.as_bytes()[0],
        None => b','
    };
    
    //  Read the file and extract identifiers from the column defined by the 'index' argument.
    //  If an error occurs then exit with an exit code (1) and output the error to standard error.
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

///  Command line arguments for the EnsEMBL sequence finder program.
///
///  Options:
///  -i, --index <index>         (required) A column index to take the set of values.      
///  -d, --delimiter <delimiter>            The delimiter character that separates each field value (e.g. ',', ';', '\t')
///  -f, --file <file>           (required) The flat file (e.g. CSV, TSV) file path to parse for identifiers
///  -n, --no-headers                       A flag that indicates no header row is present  
///  -h, --help                             Print help information
///  -V, --version                          Print version information
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