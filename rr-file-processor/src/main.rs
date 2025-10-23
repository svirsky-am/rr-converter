use clap::{Arg, Command};
use rr_parser_lib::{Format, parse_input, serialize_output};
use std::fs;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

pub struct Cli {
    pub input: String,
    pub output: String,
    pub in_format: Format,
    pub out_format: Format,
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
                .value_parser(parse_format_clap),
        )
        .arg(
            Arg::new("out-format")
                .long("out-format")
                .help("Output format: csv or xml")
                .required(true)
                .value_parser(parse_format_clap),
        )
        .get_matches();

    Ok(Cli {
        input: matches.get_one::<String>("input").unwrap().clone(),
        output: matches.get_one::<String>("output").unwrap().clone(),
        in_format: matches.get_one::<Format>("in-format").unwrap().clone(),
        out_format: matches.get_one::<Format>("out-format").unwrap().clone(),
    })
}

fn parse_format_clap(s: &str) -> Result<Format, String> {
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

fn get_timestamped_path(original_path: &Path, format: &Format) -> PathBuf {
    let now = time::OffsetDateTime::now_utc();
    let timestamp = now
        .format(&time::format_description::well_known::Iso8601::DEFAULT)
        .unwrap_or_else(|_| now.unix_timestamp().to_string())
        .replace(':', "-")
        .replace('+', "_")
        .replace('Z', "");

    let stem = original_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("output");
    let ext = match format {
        Format::Csv => "csv",
        Format::Xml => "xml",
    };

    original_path.with_file_name(format!("{}-{}.{}", stem, timestamp, ext))
}

fn write_output(
    dest: &str,
    content: &str,
    out_format: &Format,
) -> Result<(), Box<dyn std::error::Error>> {
    if dest == "-" {
        let mut stdout = io::stdout();
        stdout.write_all(content.as_bytes())?;
        stdout.flush()?;
        return Ok(());
    }

    let path = get_timestamped_path(Path::new(dest), out_format);
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)?;
        }
    }
    fs::write(&path, content)?;
    eprintln!("Written to: {}", path.display());
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = parse_cli()?;
    let input_content = read_input(&cli.input)?;
    let data = parse_input(&input_content, &cli.in_format)?;
    let output_content = serialize_output(&data, &cli.out_format)?;
    write_output(&cli.output, &output_content, &cli.out_format)?;
    Ok(())
}
