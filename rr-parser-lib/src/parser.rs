use std::cell::RefCell;

// use std::io;
use std::io::{self, Read, Write};

use std::fmt;

use std::rc::Rc;

// In-memory parsed CSV
#[derive(Debug, Default)]
struct UniParser {
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
}

#[derive(Hash, PartialEq, Eq, Debug)]
struct Transaction {
    country: &'static str,
    id: u32,
}

#[derive(Hash, PartialEq, Eq, Debug)]
struct AccountInto {
    description: String,
    id: u32,
    transactions: Vec<Transaction>,
    currency: String,
}

impl Default for AccountInto {
    fn default() -> Self {
        AccountInto {
            id: 5,
            description: String::new(),
            transactions: Vec::new(),
            currency: String::new(),
        }
    }
}

impl AccountInto {
    pub fn new(id: u32, description: String) -> Self {
        Self {
            id,
            description,
            ..Default::default()
        }
    }

    fn render_content_as_csv(&mut self) -> Vec<u8> {
        let mut result_content = String::from("---\n");

        result_content.push_str(&format!("account_info: {:#?}\n", self));
        // let mut result_content: Vec<u8> = format!("result_content: {}\n", self)
        // ;
        result_content.as_bytes().to_vec()
    }

    fn render_content_as_csv_extra_fin(&mut self) -> Vec<u8> {
        let mut result_content = String::from("---\n");

        result_content.push_str(&format!("account_info: {:#?}\n", self));
        // let mut result_content: Vec<u8> = format!("result_content: {}\n", self)
        // ;
        result_content.as_bytes().to_vec()
    }

    fn render_content_as_yaml(&mut self) -> Vec<u8> {
        let mut result_content = String::from("---\n");

        result_content.push_str(&format!("account_info: {:#?}\n", self));
        // let mut result_content: Vec<u8> = format!("result_content: {}\n", self)
        // ;
        result_content.as_bytes().to_vec()
    }

    fn render_content_as_camt053(&mut self) -> Vec<u8> {
        let mut result_content = String::from("---\n");

        result_content.push_str(&format!("account_info: {:#?}\n", self));
        // let mut result_content: Vec<u8> = format!("result_content: {}\n", self)
        // ;
        result_content.as_bytes().to_vec()
    }

    fn render_content_as_mt940(&mut self) -> Vec<u8> {
        let mut result_content = String::from("---\n");

        result_content.push_str(&format!("account_info: {:#?}\n", self));
        // let mut result_content: Vec<u8> = format!("result_content: {}\n", self)
        // ;
        result_content.as_bytes().to_vec()
    }
}
#[derive(Debug)]
struct XmlNode {
    tag_name: String,
    value: String,
    parent: Option<Rc<RefCell<XmlNode>>>,
}

fn find_xml_xml_tag_with_value_in_line(trimed_line: &str) -> Option<Rc<RefCell<XmlNode>>> {
    let tag_start = trimed_line.find('<').unwrap() + 1;
    let tag_end = trimed_line[tag_start..].find('>').unwrap_or(0) + tag_start;
    let tag = &trimed_line[tag_start..tag_end];
    let content_start = tag_end + 1;
    let content_end = trimed_line[content_start..]
        .find('<')
        .unwrap_or(trimed_line.len() - content_start)
        + content_start;
    let content: &str = &trimed_line[content_start..content_end];
    let result_node: Rc<RefCell<XmlNode>> = Rc::new(RefCell::new(XmlNode {
        tag_name: tag.to_owned(),
        value: content.to_owned(),
        parent: None,
    }));
    if !content.is_empty() {
        Some(result_node)
    } else {
        None
    }
}

fn find_open_tag(trimed_line: &str) -> Option<Rc<RefCell<XmlNode>>> {
    let tag_start = trimed_line.find('<').unwrap() + 1;
    let tag_end = trimed_line[tag_start..].find('>').unwrap_or(0) + tag_start;
    let tag = &trimed_line[tag_start..tag_end];
    let content_start = tag_end + 1;
    let content_end = trimed_line[content_start..]
        .find('<')
        .unwrap_or(trimed_line.len() - content_start)
        + content_start;
    let content: &str = &trimed_line[content_start..content_end];
    let result_node: Rc<RefCell<XmlNode>> = Rc::new(RefCell::new(XmlNode {
        tag_name: tag.to_owned(),
        value: content.to_owned(),
        parent: None,
    }));
    if !content.is_empty() {
        Some(result_node)
    } else {
        None
    }
}

fn find_xml_tag_with_value_in_line(
    trimed_line: &str,
    parent_node: &Rc<RefCell<XmlNode>>,
) -> Option<Rc<RefCell<XmlNode>>> {
    let tag_start = trimed_line.find('<').unwrap() + 1;
    let tag_end = trimed_line[tag_start..].find('>').unwrap_or(0) + tag_start;
    let tag = &trimed_line[tag_start..tag_end];
    let content_start = tag_end + 1;
    let content_end = trimed_line[content_start..]
        .find('<')
        .unwrap_or(trimed_line.len() - content_start)
        + content_start;
    let content: &str = &trimed_line[content_start..content_end];
    let result_node: Rc<RefCell<XmlNode>> = Rc::new(RefCell::new(XmlNode {
        tag_name: tag.to_owned(),
        value: content.to_owned(),
        parent: Some(Rc::clone(parent_node)),
    }));
    if !content.is_empty() {
        Some(result_node)
    } else {
        None
    }
}

impl UniParser {
    fn parse_csv_from_str(&mut self, input: &str) -> AccountInto {
        let mut lines = input.lines();
        if let Some(header) = lines.next() {
            self.headers = header.split(',').map(|s| s.trim().to_string()).collect();
        }

        let mut data_transactions: Vec<Transaction> = Vec::new();
        let account_data = AccountInto::new(5, "csv from str".to_owned());

        for line in lines {
            let line = line.trim();
            if !line.is_empty() {
                let _split_string = &line.split(',').map(|s| s.trim().to_string());
                self.rows
                    .push(line.split(',').map(|s| s.trim().to_string()).collect());

                // data_transactions.append(other);

                data_transactions.push(Transaction {
                    country: "ru",
                    id: 1,
                });
            }
        }

        // dbg!(&data_transactions);
        account_data
    }

    fn parse_csv_extra_fin_from_str(&mut self, input: &str) -> AccountInto {
        let mut lines = input.lines();
        if let Some(header) = lines.next() {
            self.headers = header.split(',').map(|s| s.trim().to_string()).collect();
        }

        let mut data_transactions: Vec<Transaction> = Vec::new();
        let account_data = AccountInto::new(7, "csv from str".to_owned());

        for line in lines {
            let line = line.trim();
            if !line.is_empty() {
                let _split_string = &line.split(',').map(|s| s.trim().to_string());
                self.rows
                    .push(line.split(',').map(|s| s.trim().to_string()).collect());

                // data_transactions.append(other);

                data_transactions.push(Transaction {
                    country: "ru",
                    id: 1,
                });
            }
        }

        // dbg!(&data_transactions);
        account_data
    }

    fn parse_camt053_from_str(&mut self, input: &str) -> AccountInto {
        let lines = input.lines();
        let data_transactions: Vec<Transaction> = Vec::new();
        let account_data = AccountInto::new(3, "camt053 from str".to_owned());

        let mut reader = quick_xml::reader::Reader::from_str(input);
        reader.config_mut().trim_text(true);


        let mut count = 0;
        let mut txt: Vec<String> = Vec::new();
        let mut buf = Vec::new();

        // The `Reader` does not implement `Iterator` because it outputs borrowed data (`Cow`s)
        loop {
            // NOTE: this is the generic case when we don't know about the input BufRead.
            // when the input is a &str or a &[u8], we don't actually need to use another
            // buffer, we could directly call `reader.read_event()`
            match reader.read_event_into(&mut buf) {
                Err(e) => panic!("Error at position {}: {:?}", reader.error_position(), e),
                // exits the loop when reaching end of file
                Ok(Event::Eof) => break,

                Ok(Event::Start(e)) => {
                    match e.name().as_ref() {
                        b"tag1" => println!("attributes values: {:?}",
                                            e.attributes().map(|a| a.unwrap().value)
                                            .collect::<Vec<_>>()),
                        b"tag2" => count += 1,
                        _ => (),
                    }
                }
                Ok(Event::Text(e)) => txt.push(e.decode().unwrap().into_owned()),

                // There are several other `Event`s we do not consider here
                _ => (),
            }
            // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
            buf.clear();
        }

        // dbg!(&data_transactions);
        account_data
    }

    fn parse_mt940_from_str(&mut self, input: &str) -> AccountInto {
        let mut lines = input.lines();

        if let Some(header) = lines.next() {
            self.headers = header.split(',').map(|s| s.trim().to_string()).collect();
        }

        // if let Some(header) = lines.next() {
        //     self.headers = header.split(',').map(|s| s.trim().to_string()).collect();
        // }

        // let mut data_transactions: Vec<Transaction> = Vec::new();
        // let account_data = AccountInto::new(6, "csv from str".to_owned());

        // for line in lines {
        //     let line = line.trim();
        //     if !line.is_empty() {
        //         let _split_string = &line.split(',').map(|s| s.trim().to_string());
        //         self.rows
        //             .push(line.split(',').map(|s| s.trim().to_string()).collect());

        //         // data_transactions.append(other);

        //         data_transactions.push(Transaction {
        //             country: "ru",
        //             id: 1,
        //         });
        //     }
        // }

        // dbg!(&data_transactions);
        AccountInto::new(6, "csv from str".to_owned())
    }

    fn to_yaml_bytes(&self) -> Vec<u8> {
        if self.headers.is_empty() {
            return Vec::new();
        }

        let mut yaml = String::from("---\n");
        for row in &self.rows {
            yaml.push_str("-\n");
            for (i, value) in row.iter().enumerate() {
                if i < self.headers.len() {
                    let key = &self.headers[i];
                    let display = if value.chars().all(|c| c.is_ascii_digit())
                        || value.contains([' ', ':', '{', '}', '[', ']', ',', '"'])
                    {
                        format!("\"{}\"", value)
                    } else {
                        value.clone()
                    };
                    yaml.push_str(&format!("  {}: {}\n", key, display));
                }
            }
        }
        yaml.into_bytes()
    }

    fn account_to_yaml_bytes(&self, account_data: AccountInto) -> Vec<u8> {
        // if self.headers.is_empty() {
        //     return Vec::new();
        // }

        let mut yaml = String::from("---\n");

        yaml.push_str(&format!("account_info: {:#?}\n", account_data));

        yaml.into_bytes()
    }
}

#[derive(Debug, Clone)]
pub enum InputParserFormat {
    Csv,
    CsvExtraFin,
    Xml,
    Camt053,
    Mt940,
}

impl fmt::Display for InputParserFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InputParserFormat::Csv => write!(f, "csv"),
            InputParserFormat::CsvExtraFin => write!(f, "CsvExtraFin♦"),
            InputParserFormat::Xml => write!(f, "Xml"),
            InputParserFormat::Mt940 => write!(f, "Mt940♣"),
            InputParserFormat::Camt053 => write!(f, "Camt053♥"),
        }
    }
}

impl std::str::FromStr for InputParserFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "csv" => Ok(InputParserFormat::Csv),
            "csvextrafin" => Ok(InputParserFormat::CsvExtraFin),
            "xml" => Ok(InputParserFormat::Xml),
            "camt053" => Ok(InputParserFormat::Camt053),
            "mt940" => Ok(InputParserFormat::Mt940),
            _ => Err(format!(
                "Unsupported format: {}. Supported: csv, xml, camt053, mt940",
                s
            )),
        }
    }
}

impl InputParserFormat {
    pub fn all_variants() -> &'static [InputParserFormat] {
        &[
            InputParserFormat::Csv,
            InputParserFormat::CsvExtraFin,
            InputParserFormat::Xml,
            InputParserFormat::Mt940,
        ]
    }
}

#[derive(Debug, Clone, strum_macros::EnumString)]
pub enum OutputParserFormat {
    #[strum(serialize = "csv")]
    Csv,
    #[strum(serialize = "csvextrafin", serialize = "csv_extra_fin")]
    CsvExtraFin,
    #[strum(serialize = "yaml")]
    Yaml,
    // Xml,
    #[strum(serialize = "camt053")]
    Camt053,
    #[strum(serialize = "mt940")]
    Mt940,
}
impl fmt::Display for OutputParserFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OutputParserFormat::Csv => write!(f, "csv"),
            OutputParserFormat::CsvExtraFin => write!(f, "CsvExtraFin"),
            OutputParserFormat::Yaml => write!(f, "Yaml"),
            OutputParserFormat::Mt940 => write!(f, "Mt940"),
            OutputParserFormat::Camt053 => write!(f, "Camt053"),
        }
    }
}

#[derive(Debug)]
pub struct ParseOutputParserFormatError(String);

impl std::fmt::Display for ParseOutputParserFormatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl std::error::Error for ParseOutputParserFormatError {}

// impl std::str::FromStr for OutputParserFormat {
//     type Err = String;
//     // type Err = String;

//     fn from_str(s: &str) -> Result<OutputParserFormat, String> {
//         let binding = s.to_lowercase();
//         let match_string = binding.as_str();
//         match match_string {
//             "csv" => Ok(OutputParserFormat::Csv),
//             "csvextrafin" => Ok(OutputParserFormat::CsvExtraFin),
//             "yaml" => Ok(OutputParserFormat::Yaml),
//             "camt053" => Ok(OutputParserFormat::Camt053),
//             "mt940" => Ok(OutputParserFormat::Mt940),
//             _ => Err(format!("Unsupported format: {}. Supported: csv, xml, camt053, mt940", match_string)),
//         }
//     }
// }

impl OutputParserFormat {
    pub fn all_variants() -> &'static [OutputParserFormat] {
        &[
            OutputParserFormat::Csv,
            OutputParserFormat::CsvExtraFin,
            OutputParserFormat::Yaml,
            OutputParserFormat::Mt940,
        ]
    }
}

// 🔑 The core struct: implements both Read and Write
pub struct FinConverter {
    // Input state (for Write)
    process_input_type: InputParserFormat,
    process_output_type: OutputParserFormat,
    input_buffer: String,
    flushed: bool,

    // Output state (for Read)
    output_bytes: Vec<u8>,
    read_pos: usize,
}

impl FinConverter {
    pub fn new(
        process_input_type: InputParserFormat,
        process_output_type: OutputParserFormat,
    ) -> Self {
        Self {
            process_input_type,
            process_output_type,
            input_buffer: String::new(),
            flushed: false,
            output_bytes: Vec::new(),
            read_pos: 0,
        }
    }

    // Internal method: parse CSV and generate YAML bytes
    fn process_data(&mut self) {
        if self.flushed {
            return;
        }

        let mut parser = UniParser::default();
        let mut parsed_account_data = match self.process_input_type {
            InputParserFormat::Csv => parser.parse_csv_from_str(&self.input_buffer),
            InputParserFormat::CsvExtraFin => {
                parser.parse_csv_extra_fin_from_str(&self.input_buffer)
            }
            InputParserFormat::Xml => parser.parse_csv_from_str(&self.input_buffer),
            InputParserFormat::Camt053 => parser.parse_camt053_from_str(&self.input_buffer),
            InputParserFormat::Mt940 => parser.parse_mt940_from_str(&self.input_buffer),
        };

        // self.output_bytes = parser.account_to_yaml_bytes(account_data);
        let rendered_result = match self.process_output_type {
            OutputParserFormat::Csv => parsed_account_data.render_content_as_csv(),
            OutputParserFormat::CsvExtraFin => {
                parsed_account_data.render_content_as_csv_extra_fin()
            }
            OutputParserFormat::Yaml => parsed_account_data.render_content_as_yaml(),
            OutputParserFormat::Camt053 => parsed_account_data.render_content_as_camt053(),
            OutputParserFormat::Mt940 => parsed_account_data.render_content_as_mt940(),
        };

        self.output_bytes = rendered_result;
        let mut output_format_str = format!("output_format: {}\n", self.process_output_type)
            .as_bytes()
            .to_vec();
        let mut input_format_str = format!("input_format: {}\n", self.process_input_type)
            .as_bytes()
            .to_vec();
        self.output_bytes.append(&mut input_format_str);
        self.output_bytes.append(&mut output_format_str);

        self.flushed = true;
    }
}

use chardetng::EncodingDetector;
use quick_xml::events::Event;

fn detect_and_decode(buf: &[u8]) -> String {
    let mut detector = EncodingDetector::new();
    detector.feed(buf, true); // true = last buffer
    let encoding = detector.guess(None, true);
    let (cow, ..) = encoding.decode(buf);
    cow.into_owned()
}
// detector.

// 📥 Implement Write: accept CSV data
impl Write for FinConverter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let _detected_coding = detect_and_decode(buf);
        let s = if let Ok(utf8) = std::str::from_utf8(buf) {
            utf8.to_string()
        } else {
            detect_and_decode(buf)
        };
        self.input_buffer.push_str(&s);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        self.process_data(); // Parse and prepare YAML
        self.flushed = false;
        Ok(())
    }
}

// 📤 Implement Read: emit YAML data
// Read apply to buffer of converter
impl Read for FinConverter {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if self.read_pos >= self.output_bytes.len() {
            return Ok(0); // EOF
        }

        let remaining = self.output_bytes.len() - self.read_pos;
        let to_copy = std::cmp::min(buf.len(), remaining);
        buf[..to_copy].copy_from_slice(&self.output_bytes[self.read_pos..self.read_pos + to_copy]);
        self.read_pos += to_copy;
        Ok(to_copy)
    }
}

// ===== Example usage with stdio and BufReader/BufWriter =====
pub fn parse_input_and_serialize_via_trait<TypeOfBuffInput: Read, TypeOfBuffOutput: Write>(
    mut input_buff_reader: TypeOfBuffInput,
    mut output_buff_writer: TypeOfBuffOutput,
    process_input_type: InputParserFormat,
    process_output_type: OutputParserFormat,
) -> io::Result<()> {
    // Create our transformer
    let mut converter = FinConverter::new(process_input_type, process_output_type);

    // 1️⃣ Read CSV from stdin using Read trait (via copy)

    std::io::copy(&mut input_buff_reader, &mut converter)?;

    // 2️⃣ Flush to trigger parsing (optional — Read will trigger it too)
    converter.flush()?;

    // 3️⃣ Write YAML to stdout using Read trait (via copy)
    std::io::copy(&mut converter, &mut output_buff_writer)?;

    Ok(())
}

mod tests;
