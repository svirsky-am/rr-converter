use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
// use std::io;
use std::io::{self, Read, Write};

use serde::Serialize;

#[derive(Debug, Clone, PartialEq)]
pub enum ParserFormat {
    Csv,
    Xml,
}

impl std::str::FromStr for ParserFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "csv" => Ok(ParserFormat::Csv),
            "xml" => Ok(ParserFormat::Xml),
            _ => Err(format!("Unsupported format: {}. Supported: csv, xml", s)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Data {
    // Simple table: list of rows, each row is a map of field -> value
    pub headers: Vec<String>,
    pub rows: Vec<HashMap<String, String>>,
}

impl Data {
    pub fn new(headers: Vec<String>, rows: Vec<HashMap<String, String>>) -> Self {
        Self { headers, rows }
    }
}

// ===== CSV PARSING (simple, no quotes/escaping) =====
pub fn parse_csv(input: &str) -> Result<Data, Box<dyn std::error::Error>> {
    let mut lines = input.lines();
    let header_line = lines.next().ok_or("CSV is empty")?;
    let headers: Vec<String> = header_line
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();

    let mut rows = Vec::new();
    for line in lines {
        if line.trim().is_empty() {
            continue;
        }
        let values: Vec<&str> = line.split(',').collect();
        if values.len() != headers.len() {
            return Err(format!(
                "Row has {} fields, expected {}",
                values.len(),
                headers.len()
            )
            .into());
        }
        let mut row = HashMap::new();
        for (i, &val) in values.iter().enumerate() {
            row.insert(headers[i].clone(), val.trim().to_string());
        }
        rows.push(row);
    }
    Ok(Data::new(headers, rows))
}

// ===== XML PARSING (very basic, assumes flat structure) =====
pub fn parse_xml(input: &str) -> Result<Data, Box<dyn std::error::Error>> {
    use std::io::BufRead;

    let reader = std::io::BufReader::new(input.as_bytes());
    let mut headers = Vec::new();
    let mut rows = Vec::new();
    let mut current_row = HashMap::new();
    let mut in_record = false;

    for line in reader.lines() {
        let line = line?;
        let trimmed = line.trim();

        if trimmed.starts_with("<record") && trimmed.ends_with('>') {
            in_record = true;
            current_row.clear();
        } else if trimmed == "</record>" {
            in_record = false;
            if !current_row.is_empty() {
                // Infer headers from first row
                if headers.is_empty() {
                    headers = current_row.keys().cloned().collect();
                }
                rows.push(current_row.clone());
            }
        } else if in_record
            && trimmed.starts_with('<')
            && trimmed.ends_with('>')
            && !trimmed.starts_with("</")
        {
            // Extract tag and content: <name>Alice</name>
            let tag_start = trimmed.find('<').unwrap() + 1;
            let tag_end = trimmed[tag_start..].find('>').unwrap_or(0) + tag_start;
            let tag = &trimmed[tag_start..tag_end];

            let content_start = tag_end + 1;
            let content_end = trimmed[content_start..]
                .find('<')
                .unwrap_or(trimmed.len() - content_start)
                + content_start;
            let content = &trimmed[content_start..content_end];

            current_row.insert(tag.to_string(), content.to_string());
        }
    }

    if headers.is_empty() && !rows.is_empty() {
        headers = rows[0].keys().cloned().collect();
    }

    Ok(Data::new(headers, rows))
}

// ===== CSV SERIALIZATION =====
pub fn serialize_csv(data: &Data) -> String {
    if data.headers.is_empty() {
        return String::new();
    }

    let mut output = String::new();
    output.push_str(&data.headers.join(","));
    output.push('\n');

    for row in &data.rows {
        let values: Vec<String> = data
            .headers
            .iter()
            .map(|h| row.get(h).cloned().unwrap_or_default())
            .collect();
        output.push_str(&values.join(","));
        output.push('\n');
    }
    output
}

// ===== XML SERIALIZATION =====
pub fn serialize_xml(data: &Data) -> String {
    let mut output = String::from("<records>\n");
    for row in &data.rows {
        output.push_str("  <record>\n");
        for header in &data.headers {
            let value = row.get(header).cloned().unwrap_or_default();
            output.push_str(&format!(
                "    <{}>{}</{}>\n",
                header,
                escape_xml(&value),
                header
            ));
        }
        output.push_str("  </record>\n");
    }
    output.push_str("</records>\n");
    output
}

fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "<")
        .replace('>', ">")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

// ===== MAIN CONVERTER =====
pub fn parse_input(input: &str, format: &ParserFormat) -> Result<Data, Box<dyn std::error::Error>> {
    match format {
        ParserFormat::Csv => parse_csv(input),
        ParserFormat::Xml => parse_xml(input),
    }
}

pub fn serialize_output(
    data: &Data,
    format: &ParserFormat,
) -> Result<String, Box<dyn std::error::Error>> {
    Ok(match format {
        ParserFormat::Csv => serialize_csv(data),
        ParserFormat::Xml => serialize_xml(data),
    })
}

pub fn read_file(path: &Path) -> Result<String, std::io::Error> {
    fs::read_to_string(path)
}



// Define the trait
trait Parseble {
    // &str data;
    // fn name(&self) -> &str;
    // fn data(&self) -> &str;
    // fn area(&self) -> f64;
    fn data(&self) -> String {
        // println!("Drawing a shape with area: {:.2}", self.area());
        "Drawing a shape with area:".to_string()
    }
    fn draw(&self);
}

// Define three structs
struct SerilyzerMT940 {
    radius: f64,
}

struct InputParserCSV {
    width: f64,
    height: f64,
}

struct InputParserCAMT053 {
    base: f64,
    height: f64,
}

// Implement the trait for each struct
impl Parseble for SerilyzerMT940 {
    fn draw(&self) {
        println!("Drawing a SerilyzerMT940 with radius {}", self.radius);
    }
       
    // fn data(&self) -> &str {
    //     todo!()
    // }
}


use std::fmt;

impl fmt::Display for SerilyzerMT940 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write the formatted string to the provided buffer `f`
        write!(f, "User {{ name: \"{}\" }}", self.data())
    }
}

impl Parseble for InputParserCSV {
    fn draw(&self) {
        println!("Drawing a InputParserCAMT053: {} x {}", self.width, self.height);
    }
}

impl Parseble for InputParserCAMT053 {
    fn draw(&self) {
        println!("Drawing a InputParserCAMT053 with base {} and height {}", self.base, self.height);
    }
}

// Optional: a function that accepts any type implementing Parseble
fn render_shape<T: Parseble>(shape: T) {
    shape.draw();
}

// use std::time::{SystemTime, UNIX_EPOCH};

fn get_timestamped_path(original_path: &Path, format: &ParserFormat) -> PathBuf {
    let now = time::OffsetDateTime::now_utc();
    // println!("{}", now.format("%Y-%m-%d %H:%M:%S").unwrap());
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
        ParserFormat::Csv => "csv",
        ParserFormat::Xml => "xml",
    };

    original_path.with_file_name(format!("{}-{}.{}", stem, timestamp, ext))
}

fn write_output (
    dest: &str,
    content: &str,
    out_format: &ParserFormat,
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

// ===== MAIN CONVERTER =====
pub fn parse_input_and_serialize(input: &str, input_format: &ParserFormat, output_format: &ParserFormat, output: &str) -> Result<(), Box<dyn std::error::Error>> {
    let data   = match input_format {
        ParserFormat::Csv => parse_csv(input),
        ParserFormat::Xml => parse_xml(input),
    }?;
    // Result<Data, Box<dyn std::error::Error>>

    // let data: &Data;
//     data: &Data,
//     format: &ParserFormat,
// ) -> Result<String, Box<dyn std::error::Error>> {
    let output_content = match output_format {
        ParserFormat::Csv => serialize_csv(&data),
        ParserFormat::Xml => serialize_xml(&data),
    };

    // let output_content = serialized_result;
    let _ = write_output(&output, &output_content, output_format);
    Ok(())
}





mod tests;
