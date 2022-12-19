//! MIT license.
//!
//! Given a list of EnsEMBL identifiers, return a CSV file of EnsEMBL entries.

#[actix_web::main]
async fn main() {
    let argument_matches = cli().get_matches();

    let client = match argument_matches.get_many::<String>("certificates") {
        Some(certificate_files) => utils::web::new_client(certificate_files),
        _ => unreachable!("required(true) prevents `None`."),
    };

    let file = match argument_matches.get_one::<String>("file") {
        Some(file) => file,
        _ => unreachable!("required(true) prevents `None`."),
    };

    if std::path::Path::new(file).exists() != true {
        eprintln!(
            "{{\"file\": {}, \"error\": \"{}\"}}",
            file,
            std::io::Error::from(std::io::ErrorKind::NotFound)
        );
        std::process::exit(1);
    }

    let output_file = match argument_matches.get_one::<String>("output") {
        Some(output_file) => output_file,
        _ => unreachable!("required(true) prevents `None`."),
    };

    if std::path::Path::new(output_file).exists() == true {
        eprintln!("{{\"file\": {}, \"error\": \"File already exists! Please provide a filename that does not exist.\"}}", output_file);
        std::process::exit(1);
    }

    let has_headers = !argument_matches.get_flag("no_headers");

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

    let delimiter = match argument_matches.get_one::<String>("delimiter") {
        Some(delimiter) => delimiter.as_bytes()[0],
        None => b',',
    };

    let identifiers: std::collections::HashSet<String> =
        match utils::flat_file::read(file, delimiter, has_headers, index) {
            Ok(list) => std::collections::HashSet::from_iter(list),
            Err(error) => {
                eprintln!("{{\"error\": \"{}\"}}", error);
                std::process::exit(1);
            }
        };

    let mut csv_writer = match csv::Writer::from_path(&output_file[..]) {
        Ok(writer) => writer,
        Err(error) => {
            eprintln!("{{\"file\": \"{}\", \"error\":\"{}\"}}", output_file, error);
            std::process::exit(1);
        }
    };

    let mut last_request_made = std::time::SystemTime::now();

    for identifier in identifiers {
        match utils::gene::ensembl_search(&client, &identifier, true).await {
            Ok(gene) => {
                let serialized_result = csv_writer.serialize(gene);

                if serialized_result.is_err() {
                    eprintln!(
                        "{{\"id\":\"{}\", \"error\":\"{:#?}\"}}",
                        identifier,
                        serialized_result.err()
                    );
                }

                // Genome Browser conditions of use limits requests to one request every 15 seconds.
                let time_after_request = std::time::SystemTime::now();
                match time_after_request.duration_since(last_request_made) {
                    Ok(elapsed_time) => {
                        if elapsed_time < std::time::Duration::from_secs(15) {
                            let milliseconds_to_wait = std::time::Duration::from_secs(15).as_millis() as u64 - elapsed_time.as_millis() as u64;
                            std::thread::sleep(std::time::Duration::from_millis(milliseconds_to_wait));
                        }
                    }
                    Err(error) => {
                        eprintln!("{{\"error\": {}}}", error);
                        std::thread::sleep(std::time::Duration::from_secs(15));
                    }
                }
                last_request_made = std::time::SystemTime::now();
            }
            Err(error) => {
                eprintln!("{{\"id\":\"{}\", \"error\":\"{}\"}}", identifier, error);
            }
        }
    }

    let csv_writer_flush_result = csv_writer.flush();
    if csv_writer_flush_result.is_err() {
        eprintln!("{{\"error\":\"{:#?}\"}}", csv_writer_flush_result.err());
    }
}

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
