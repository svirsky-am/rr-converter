use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, PartialEq)]
pub enum Format {
    Csv,
    Xml,
}

impl std::str::FromStr for Format {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "csv" => Ok(Format::Csv),
            "xml" => Ok(Format::Xml),
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
pub fn parse_input(input: &str, format: &Format) -> Result<Data, Box<dyn std::error::Error>> {
    match format {
        Format::Csv => parse_csv(input),
        Format::Xml => parse_xml(input),
    }
}

pub fn serialize_output(
    data: &Data,
    format: &Format,
) -> Result<String, Box<dyn std::error::Error>> {
    Ok(match format {
        Format::Csv => serialize_csv(data),
        Format::Xml => serialize_xml(data),
    })
}

pub fn read_file(path: &Path) -> Result<String, std::io::Error> {
    fs::read_to_string(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::ErrorKind;
    use std::path::Path;

    #[test]
    fn test_read_nonexistent_file() {
        let fake_path = Path::new("/definitely/does/not/exist.txt");
        let result = read_file(fake_path);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), ErrorKind::NotFound);
    }
}
