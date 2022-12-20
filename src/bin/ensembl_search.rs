//!  MIT license.
//!
//!  Given a list of EnsEMBL identifiers, return a CSV file of EnsEMBL entries.

#[actix_web::main]
async fn main() {
    //  Get arguments from the command line.
    let argument_matches = cli().get_matches();

    //  Assign certificate arguments passed in from the command line to a variable.
    //  Certificate authority certificates are required, there should always be at least one certificate string present.
    let client = match argument_matches.get_many::<String>("certificates") {
        Some(certificate_files) => {

            let paths: Vec<&std::path::Path> = certificate_files.map(|cert| std::path::Path::new(&cert[..])).collect();
            utils::web::new_client(paths)
        },
        _ => unreachable!("required(true) prevents `None`."),
    };

    //  Assign the input file argument passed in from the command line to a variable.
    //  The input file argument is required, there should always be an input filename present as an argument because of the clap crate's required(true) implementation.
    let file: &std::path::Path = match argument_matches.get_one::<String>("file") {
        Some(file) => std::path::Path::new(&file[..]),
        _ => unreachable!("required(true) prevents `None`."),
    };

    //  Exit with an error code (1) if the input file to retrieve EnsEMBL strings does not exist.
    if file.exists() != true {
        eprintln!(
            "{{\"file\": \"{}\", \"error\": \"{}\"}}",
            file.display(),
            std::io::Error::from(std::io::ErrorKind::NotFound)
        );
        std::process::exit(1);
    }

    //  Assign the output filename argument passed in from the command line to a variable.
    //  The output filename is required, there should always be an output filename present as an argument because of the clap crate's required(true) implementation.
    let output_file = match argument_matches.get_one::<String>("output") {
        Some(output_file) => output_file,
        _ => unreachable!("required(true) prevents `None`."),
    };

    //  Exit with an error code (1) if the output filename already exists.
    if std::path::Path::new(output_file).exists() == true {
        eprintln!("{{\"file\": \"{}\", \"error\": \"File already exists! Please provide a filename that does not exist.\"}}", output_file);
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
                eprintln!(
                    "{{\"error\": \"{}\"}}",
                    std::io::Error::from(std::io::ErrorKind::InvalidInput)
                );
                std::process::exit(1);
            }
        },
        _ => unreachable!("required(true) prevent `None`."),
    };

    //  Assign the delimiter argument to a variable.
    //  If no delimiter argument is supplied through the command line, assume the file delimiter is a comma.
    let delimiter = match argument_matches.get_one::<String>("delimiter") {
        Some(delimiter) => delimiter.as_bytes()[0],
        None => b',',
    };

    //  Create a csv crate CSV Writer and open the output filename for editing.
    let mut csv_writer = match csv::Writer::from_path(&output_file[..]) {
        Ok(writer) => writer,
        Err(error) => {
            eprintln!(
                "{{\"file\": \"{}\", \"error\": \"{}\"}}",
                output_file, error
            );
            std::process::exit(1);
        }
    };

    //  Read the file and extract identifiers from the column defined by the 'index' argument.
    //  If an error occurs then exit with an exit code (1) and output the error to standard error.
    let identifiers: std::collections::HashSet<String> =
        match utils::flat_file::read_column(file, delimiter, has_headers, index) {
            Ok(list) => std::collections::HashSet::from_iter(list),
            Err(error) => {
                eprintln!("{{\"error\": \"{}\"}}", error);
                std::process::exit(1);
            }
        };

    //  Track the number of requests made and the time the last request was made.
    let mut number_of_requests_made: u32 = 0;
    let mut timestamp: Option<std::time::SystemTime> = None;

    //  For each EnsEMBL identifer,
    for identifier in identifiers {
        //  Search the genome browsers.
        match utils::genome_browser::ensembl_search(
            &client,
            &identifier,
            true,
            timestamp,
            Some(number_of_requests_made),
        )
        .await
        {
            Ok(genome_browser_response) => {
                number_of_requests_made += 1;
                timestamp = Some(genome_browser_response.timestamp);

                //  Write the result as a CSV row.
                let serialized_result = csv_writer.serialize(genome_browser_response.gene);

                if serialized_result.is_err() {
                    eprintln!(
                        "{{\"id\": \"{}\", \"error\": \"{:#?}\"}}",
                        identifier,
                        serialized_result.err()
                    );
                }
            }
            Err(error) => {
                eprintln!("{{\"id\": \"{}\", \"error\": \"{}\"}}", identifier, error);
            }
        }
    }

    //  Flush the CSV writer before exiting.
    let csv_writer_flush_result = csv_writer.flush();
    if csv_writer_flush_result.is_err() {
        eprintln!("{{\"error\": \"{:#?}\"}}", csv_writer_flush_result.err());
    }
}

///  Command line arguments for the EnsEMBL sequence finder program.
///
///  Options:
///  -c, --certificate <certificates> (required)  A DER-encoded X.509 file
///  -i, --index <index>              (required)  A column index to take the set of values.
///  -d, --delimiter <delimiter>                  The delimiter character that separates each field value (e.g. ',', ';', '\t')
///  -f, --file <file>                (required)  The flat file (e.g. CSV, TSV) file path to parse for identifiers
///  -n, --no-headers                             A flag that indicates no header row is present
///  -O, --output <output>            (required)  The output file name and path to write a CSV file
///  -h, --help                                   Print help information
///  -V, --version                                Print version information
fn cli() -> clap::Command {
    clap::Command::new("ensembl_sequence")
    .version("0.1.0")
    .author("Nathan Edwards <Nathan.W.Edwards@Outlook.com>")
    .about("Given a flat file with EnsEMBL identifiers at the specified index and valid certificates for *.ensembl.org and genome.ucsc.edu, generate a CSV file of EnsEMBL entries")
    .arg(clap::Arg::new("certificates")
        .short('c')
        .long("certificate")
        .required(true)
        .action(clap::ArgAction::Append)
        .help("A DER-encoded X.509 file"))
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
    .arg(clap::Arg::new("output")
        .short('O')
        .long("output")
        .required(true)
        .help("The output file name and path to write a CSV file"))
}
