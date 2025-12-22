use clap::{Arg, Command};
use rr_parser_lib::{FinConverter, InputParserFormat, OutputParserFormat};
use std::fs::{self, File};
use std::io::{self, BufReader, Read, Write};
use std::path::{Path, PathBuf};


// #[derive(Parser)]
pub struct Cli {
    pub input: String,
    pub output: String,
    pub in_format: InputParserFormat,
    pub out_format: OutputParserFormat,
}

fn parse_cli() -> Result<Cli, Box<dyn std::error::Error>> {
    let matches = Command::new("format-converter")
        .version("0.1.0")
        .about("Convert between CSV and XML")
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .help("Input file ('-' for stdin)")
                .default_value("-")
                .value_parser(clap::value_parser!(String)),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .help("Output file ('-' for stdout)")
                .default_value("-")
                .value_parser(clap::value_parser!(String)),
        )
        .arg(
            Arg::new("in-format")
                .long("in-format")
                .help("Input format: csv or xml")
                .required(true)
                .value_parser(parse_input_format_clap),
        )
        .arg(
            Arg::new("out-format")
                .long("out-format")
                .help("Output format: csv or xml")
                .required(true)
                .value_parser(parse_output_format_clap),
        )
        .get_matches();

    Ok(Cli {
        input: matches.get_one::<String>("input").unwrap().clone(),
        output: matches.get_one::<String>("output").unwrap().clone(),
        in_format: matches.get_one::<InputParserFormat>("in-format").unwrap().clone(),
        out_format: matches.get_one::<OutputParserFormat>("out-format").unwrap().clone(),
    })
}

fn parse_input_format_clap(s: &str) -> Result<InputParserFormat, String> {
    s.parse()
}


fn parse_output_format_clap(s: &str) -> Result<OutputParserFormat, rr_parser_lib::ParseError> {
    s.parse()
}

fn read_input(source: &str) -> Result<String, Box<dyn std::error::Error>> {
    if source == "-" {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        Ok(buffer)
    } else {
        fs::read_to_string(source).map_err(Into::into)
    }
}

// fn get_timestamped_path(original_path: &Path, format: &OutputParserFormat) -> PathBuf {
//     let now = time::OffsetDateTime::now_utc();
//     let timestamp = now
//         .format(&time::format_description::well_known::Iso8601::DEFAULT)
//         .unwrap_or_else(|_| now.unix_timestamp().to_string())
//         .replace(':', "-")
//         .replace('+', "_")
//         .replace('Z', "");

//     let stem = original_path
//         .file_stem()
//         .and_then(|s| s.to_str())
//         .unwrap_or("output");
//     let ext = match format {
//         OutputParserFormat::Csv => "csv",
//         OutputParserFormat::Yaml => "yaml",
//         OutputParserFormat::Camt053 => "camt053",
//         OutputParserFormat::Mt940 => "mt940",
//     };

//     original_path.with_file_name(format!("{}-{}.{}", stem, timestamp, ext))
// }

// fn write_output(
//     dest: &str,
//     content: &str,
//     out_format: &OutputParserFormat,
// ) -> Result<(), Box<dyn std::error::Error>> {
//     if dest == "-" {
//         let mut stdout = io::stdout();
//         stdout.write_all(content.as_bytes())?;
//         stdout.flush()?;
//         return Ok(());
//     }

//     let path = get_timestamped_path(Path::new(dest), out_format);
//     if let Some(parent) = path.parent() {
//         if !parent.exists() {
//             fs::create_dir_all(parent)?;
//         }
//     }
//     fs::write(&path, content)?;
//     eprintln!("Written to: {}", path.display());
//     Ok(())
// }


// // ===== MAIN CONVERTER =====
// pub fn parse_input(input: &str, format: &ParserFormat) -> Result<Data, Box<dyn std::error::Error>> {
//     match format {
//         ParserFormat::Csv => parse_csv(input),
//         ParserFormat::Xml => parse_xml(input),
//         ParserFormat::Camt053 => parse_camt053(input),
//         ParserFormat::Mt940 => parse_mt940(input),
//     }
// }



fn process_input_format(command: &str) -> Result<InputParserFormat, Box<dyn std::error::Error>> {
    match command {
        // "csv" => Ok(InputParserFormat::Csv),
        "csvextrafin" => Ok(InputParserFormat::CsvExtraFin),
        "mt940" => Ok(InputParserFormat::Mt940),
        // "xml" => Ok(InputParserFormat::Xml),
        "camt053" => Ok(InputParserFormat::Camt053),
        _ => Err(format!("Unknown command. Supported argument for in-format: {:#?}\n"," &InputParserFormat::all_variants()").into()) , // The catch-all pattern
    }
}


fn process_output_format(command: &str) -> Result<OutputParserFormat, Box<dyn std::error::Error>> {
    match command {
        // "csv" => Ok(OutputParserFormat::Csv),
        "csvextrafin" => Ok(OutputParserFormat::CsvExtraFin),
        "mt940" => Ok(OutputParserFormat::Mt940),
        "yaml" => Ok(OutputParserFormat::Yaml),
        "camt053" => Ok(OutputParserFormat::Camt053),
        _ => Err(format!("Unknown command. Supported argument for output-format: {:#?}\n"," &OutputParserFormat::all_variants()").into()) , // The catch-all pattern
    }
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = parse_cli()?;
    let input_content = read_input(&cli.input)?;
    // let in_format = parse_input(&input_content, &cli.in_format)?;
    // let output_content = serialize_output(&data, &cli.out_format)?;
    // write_output(&cli.output, &output_content, &cli.out_format)?;

    // parse_input_and_serialize_via_trait(&input_content, &cli.in_format, &cli.out_format,  &cli.output )?;


    let process_input_type: InputParserFormat = cli.in_format;
    let process_output_type = cli.out_format;

//     mut input_buff_reader: TypeOfBuffInput,
//     mut output_buff_writer: TypeOfBuffOutput,
//     process_input_type: InputParserFormat,
//     process_output_type: OutputParserFormat,
// ) -> Result<()> {
    // Create our transformer
    let mut converter = FinConverter::new(process_input_type, process_output_type);
    
    


    let mut reader_from_sdtdio: BufReader<std::io::Stdin> = BufReader::new(io::stdin());

    let dash_string = "-";
    dbg!(&cli.input);
    dbg!(&cli.input);

        // 1️⃣ Read CSV from stdin using Read trait (via copy)
    match &cli.input == dash_string {
        true => {
            dbg!("try to read from sdtio");
            std::io::copy(&mut reader_from_sdtdio, &mut converter)?
            // std::io::copy(&mut input_buff_reader, &mut converter)?
        },
        false => {
            dbg!("try to read from file");
            let input_file = fs::File::open(Path::new(&cli.input)).unwrap();       
            let mut input_buff_reader = BufReader::new(input_file);
            std::io::copy(&mut input_buff_reader, &mut converter)?
        }
    };

    // std::io::copy(&mut input_buff_reader, &mut converter)?;

    // 2️⃣ Flush to trigger parsing (optional — Read will trigger it too)
    converter.flush()?;
    let mut output_writer_stdout = io::BufWriter::new(io::stdout());


    let output_file = Path::new(&cli.output);
    let parent_dir = output_file.parent().unwrap();

    std::fs::create_dir_all(parent_dir).unwrap();
    dbg!(&cli.output);

    let output_is_std_out = &cli.output == dash_string;

    match &cli.output == dash_string {
        true => {
                dbg!(output_is_std_out);
                std::io::copy(&mut converter, &mut output_writer_stdout)
            ?},
        _ => {let outputfile = File::create(output_file).unwrap();
            let mut output_writer_file = io::BufWriter::new(outputfile);
            std::io::copy(&mut converter, &mut output_writer_file)?  
        }
    };
    Ok(())
}
