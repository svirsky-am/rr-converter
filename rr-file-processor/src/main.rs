use clap::{Arg, Command};
use rr_parser_lib::{
    FinConverter, InputParserFormat, OutputParserFormat,
};
use std::fs::{self, File};
use std::io::{self, BufReader, Write};
use std::path::Path;

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
        in_format: matches
            .get_one::<InputParserFormat>("in-format")
            .unwrap()
            .clone(),
        out_format: matches
            .get_one::<OutputParserFormat>("out-format")
            .unwrap()
            .clone(),
    })
}

fn parse_input_format_clap(s: &str) -> Result<InputParserFormat, String> {
    s.parse()
}

fn parse_output_format_clap(s: &str) -> Result<OutputParserFormat, rr_parser_lib::ParseError> {
    s.parse()
}



fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = parse_cli()?;
    // let input_content = read_input(&cli.input)?;
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
        }
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
            std::io::copy(&mut converter, &mut output_writer_stdout)?
        }
        _ => {
            let outputfile = File::create(output_file).unwrap();
            let mut output_writer_file = io::BufWriter::new(outputfile);
            std::io::copy(&mut converter, &mut output_writer_file)?
        }
    };
    Ok(())
}
