// use roxmltree::Document;
use serde::Serialize;
// use std::io;
use std::io::{self, Read, Write};

use std::fmt;
use std::path::PathBuf;
mod common;
mod sup_camp053;
mod sup_extra_fin_csv;
mod sup_mt940;
mod render;
use common::{Balance, Transaction};



// In-memory parsed CSV
#[derive(Debug, Default)]
struct UniParser {
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
    log_dir: PathBuf,
}

// #[derive(Hash, PartialEq, Eq, Debug)]
// struct Transaction {
//     country: &'static str,
//     id: u32,
// }
// use roxmltree::{Document, Node}

// #[derive(Serialize, Debug, Clone)]
#[derive(PartialEq, Debug, Clone, Serialize)]
struct Wallet {
    description: String,
    id: u128,
    // transactions: Vec<Transaction>,
    currency: String,
    pub account: String,
    pub statement_id: String,
    pub creation_time: Option<String>, // Only in CAMT
    pub opening_balance: Option<Balance>,
    pub closing_balance: Option<Balance>,
    pub transactions: Vec<Transaction>,
}

impl Default for Wallet {
    fn default() -> Self {
        Wallet {
            id: 0,
            description: String::new(),
            transactions: Vec::new(),
            currency: String::new(),
            account: String::new(),
            statement_id: String::new(),
            creation_time: Some(String::new()),
            opening_balance: Some(Balance { amount: 0.0,
                currency: "default_currency".to_owned(),
                credit_debit: common::BalanceAdjustType::WithoutInfo,
                date: "default_credit_debit".to_owned(),
                country: "default_credit_debit".to_owned(),
                last_ops: Vec::new()
             }),
            closing_balance: Some(Balance { amount: 0.0,
                currency: "default_currency".to_owned(),
                credit_debit: common::BalanceAdjustType::WithoutInfo,
                date: "default_credit_debit".to_owned(),
                country: "default_credit_debit".to_owned(),  
                last_ops: Vec::new() 
            }),

        }
    }
}

impl Wallet {
    pub fn new(id: u128, description: String) -> Self {
        Self {
            id,
            description,
            ..Default::default()
        }
    }



    fn make_debug_as_yaml(&self) -> Wallet  {
        // let mut result_content = String::from("---\n");

        // result_content.push_str(&format!("account_info: {:#?}\n", self));
        // // let mut result_content: Vec<u8> = format!("result_content: {}\n", self)
        // // ;
        // result_content.as_bytes().to_vec()

        let opening_balance = Balance { amount: 0.0,
            currency: "currency".to_owned(),
            credit_debit: common::BalanceAdjustType::Credit,
            date: "credit_debit".to_owned(),
            country: "credit_debit".to_owned(),
            last_ops: Vec::new()};
        let closing_balance = Balance { amount: 0.0,
            currency: "currency".to_owned(),
            credit_debit: common::BalanceAdjustType::Credit,
            date: "credit_debit".to_owned(),
            country: "credit_debit".to_owned(),
            last_ops: Vec::new()};
        // let transactions = Vec::new();
        Wallet::default()

    }


 
}

impl UniParser {
    fn parse_csv_from_str(&mut self, input: &str) -> anyhow::Result<Vec<Wallet>> {
        dbg!(&input);
        let mut lines = input.lines();
        if let Some(header) = lines.next() {
            self.headers = header.split(',').map(|s| s.trim().to_string()).collect();
        }

        let mut data_transactions: Vec<Transaction> = Vec::new();
        let account_data = Wallet::new(5, "csv from str".to_owned());



        // let mut output: Vec<Wallet> = Vec::new();
        // output.push(account_data);
        // 
        let output = vec![account_data] ;
        Ok(output)
    }

    fn parse_csv_extra_fin_from_str(&mut self, input: &str) -> anyhow::Result<Vec<Wallet>> {


        let parts: Vec<&str> = input.split(",,,,,,,,,,,,,,,,,,,,,,\n").collect();


        for cap in parts.iter() {
            let body = &cap;
            // dbg!(&body);
        }

        let sratemnts_header = parts[1];
        // dbg!(sratemnts_header);
        let bracked_csv = parts[2];
        // dbg!(bracked_csv);

        let output_dbg_sratemnts_header = self.log_dir.join(format!("{}_extra_csv_sratemnts_header.txt", gen_time_prefix_to_filename()));
        std::fs::create_dir_all(&self.log_dir).unwrap();
        let mut file_dbg_sratemnts_header = std::fs::File::create(output_dbg_sratemnts_header).unwrap();
        file_dbg_sratemnts_header.write_all(sratemnts_header.as_bytes());

        let output_dbg_bracked_csv = self.log_dir.join(format!("{}_extra_csv_bracked.csv", gen_time_prefix_to_filename()));
        let mut file_dbg_bracked_csv = std::fs::File::create(output_dbg_bracked_csv).unwrap();
        file_dbg_bracked_csv.write_all(bracked_csv.as_bytes());


        let normalyzed_csv_str = sup_extra_fin_csv::normalyze_csv_str(bracked_csv.to_owned());
        // dbg!(&normalyzed_csv_str);
        let output_normalyzed_csv = self.log_dir.join(format!("{}_normalyzed.csv", gen_time_prefix_to_filename()));
        let mut file_output_normalyzed_csv = std::fs::File::create(output_normalyzed_csv).unwrap();
        file_output_normalyzed_csv.write_all(normalyzed_csv_str.as_bytes());

        let mut transactions  = sup_extra_fin_csv::parsr_csv_str(normalyzed_csv_str.to_owned())?;


        // let re_msg_all = Regex::new(r"[\[\(\{]1\:(.*)\}\{2\:(.*)\}\{3:[^}]*\}\{4:\n?([:\w\n\d\/, ]*)").unwrap();
        // let re_msg_all = regex::Regex::new(r",,,,,,,,,,,,,,,,,,,,,,\n([.\n]*)\d").unwrap();
        // for cap in re_msg_all.captures_iter(input) {
        //     let body = &cap[0];
        //     dbg!(&body);
        //     // dbg!( &body);
        //     let mut fields: Vec<(&str, String)> = Vec::new();
        // }



        let mut lines = input.lines();
        if let Some(header) = lines.next() {
            self.headers = header.split(',').map(|s| s.trim().to_string()).collect();
        }

        let mut data_transactions: Vec<Transaction> = Vec::new();
        let mut account_data = Wallet::new(7, "csv from str".to_owned());
        account_data.transactions = transactions;

        // for line in lines {
        //     let line = line.trim();
        //     if !line.is_empty() {
        //         let _split_string = &line.split(',').map(|s| s.trim().to_string());
        //         self.rows
        //             .push(line.split(',').map(|s| s.trim().to_string()).collect());

        //         // data_transactions.append(other);

        //         // data_transactions.push(Transaction {
        //         //     // country: "ru",
        //         //     amount: 0.0f64,
                    
        //         //     id: 1,
        //         //     date: "value_date".to_owned(),
        //         //     // amount: todo!(),
        //         //     currency: "todo!()".to_owned(),
        //         //     credit_debit: "todo!()".to_owned(),
        //         //     narrative: Vec::new(),
        //         // });
        //     }
        // }

        let output = vec![account_data] ;
        Ok(output)
    }

    fn parse_camt053_from_str(&mut self, input: &str) -> anyhow::Result<Vec<Wallet>> {
        const NS: &str = "urn:iso:std:iso:20022:tech:xsd:camt.053.001.02";
        use roxmltree::Document;
        use sup_camp053::{get_text, find_nested_text};

        use anyhow::{Context};
        // let content = input;
        let doc = Document::parse(input).unwrap();
        let root = doc.root_element();

        let bk_to_cstmr_stmt = root.children()
            .find(|n| n.has_tag_name((NS, "BkToCstmrStmt")))
            .context("Missing BkToCstmrStmt")?;

        let grp_hdr = bk_to_cstmr_stmt.children()
            .find(|n| n.has_tag_name((NS, "GrpHdr")))
            .context("Missing GrpHdr")?;
        let msg_id = get_text(grp_hdr, (NS, "MsgId"));
        let cre_dt_tm = get_text(grp_hdr, (NS, "CreDtTm"));

        let stmt = bk_to_cstmr_stmt.children()
            .find(|n| n.has_tag_name((NS, "Stmt")))
            .context("Missing Stmt")?;

        let acct = stmt.children()
            .find(|n| n.has_tag_name((NS, "Acct")))
            .context("Missing Acct")?;
        let iban = find_nested_text(acct, &[(NS, "Id"), (NS, "IBAN")]);
        let currency = get_text(acct, (NS, "Ccy"));
        let account = if iban.is_empty() { "UNKNOWN".to_string() } else { iban };

        let id: u128 = 0;
        
        // Balances
        let mut balances = Vec::new();
        // let last_ops_vec: Vec<common::OpKind> = Vec::new();
        // let last_ops = last_ops_vec.to_owned();
        for bal in stmt.children().filter(|n| n.has_tag_name((NS, "Bal"))) {
            let code = find_nested_text(bal, &[(NS, "Tp"), (NS, "CdOrPrtry"), (NS, "Cd")]);
            let amt_node = bal.children().find(|n| n.has_tag_name((NS, "Amt")))
                .context("Balance missing Amt")?;
            let amount: f64 = amt_node.text().unwrap_or("0").parse().unwrap_or(0.0);
            let amt_ccy = amt_node.attribute("Ccy").unwrap_or(&currency).to_string();
            let credit_debit = match get_text(bal, (NS, "CdtDbtInd")).as_str() {
                "CRDT" => common::BalanceAdjustType::Credit,
                "DBIT" => common::BalanceAdjustType::Debit,
                _ => common::BalanceAdjustType::Debit, // or consider logging/warning for unexpected values
            };

            let date = find_nested_text(bal, &[(NS, "Dt"), (NS, "Dt")]);
            let country: String= "en".to_string();
            balances.push((code, Balance {
                amount,
                currency: amt_ccy,
                credit_debit,
                date,
                country,
                last_ops: Vec::new()
            }));
        }

        let opening_balance = balances.iter().find(|(code, _)| code == "OPBD").map(|(_, b)| b.clone());
        let closing_balance = balances.iter().find(|(code, _)| code == "CLBD").map(|(_, b)| b.clone());

        // Transactions
        let mut transactions = Vec::new();
        for ntry in stmt.children().filter(|n| n.has_tag_name((NS, "Ntry"))) {
            let amt_node = ntry.children().find(|n| n.has_tag_name((NS, "Amt")))
                .context("Entry missing Amt")?;
            let amount: f64 = amt_node.text().unwrap_or("0").parse().unwrap_or(0.0);
            let currency = amt_node.attribute("Ccy").unwrap_or("").to_string();
            let currency = if currency.is_empty() { currency.clone() } else { currency };
            let credit_debit = match get_text(ntry, (NS, "CdtDbtInd")).as_str(){
                "DBIT" => common::BalanceAdjustType::Debit,
                "CRDT" => common::BalanceAdjustType::Credit,
                _ => common::BalanceAdjustType::WithoutInfo 
            };
            let date = find_nested_text(ntry, &[(NS, "ValDt"), (NS, "Dt")]);


            let debit_account: String = "TODO debit_account".to_string();
            let credit_account: String = "TODO credit_account".to_string();
            let target_bank: String = "TODO target bank".to_string();
            let purpose = "TODO target bank".to_string();
            // AddtlTxInf


            let mut narratives = Vec::new();
            if let Some(rmt) = ntry.children().find(|n| n.has_tag_name((NS, "RmtInf"))) {
                for ustrd in rmt.children().filter(|n| n.has_tag_name((NS, "Ustrd"))) {
                    if let Some(text) = ustrd.text() {
                        narratives.push(text.trim().to_string());
                    }
                }
            }
            
            transactions.push(Transaction {
                id,
                // &&country,

                credit_account,
                debit_account,
                date,
                amount,
                currency: currency.clone(),
                credit_debit,
                // narrative: narratives,
                target_bank,
                purpose,
                transaction_type: None
            });
        }
        let id: u128 = 0;
        let description: String = "0".to_owned();
        let output = vec![Wallet {
            id,
            description,
            account,
            currency,
            statement_id: msg_id,
            creation_time: Some(cre_dt_tm),
            opening_balance,
            closing_balance,
            transactions,
        }] ;
        Ok(output)
        // Ok()
        // dbg!(&data_transactions);
        // account_data
    }

    fn parse_mt940_from_str(&mut self, input: &str) -> anyhow::Result<Vec<Wallet>> {
        // let mut lines = input.lines();

        // if let Some(header) = lines.next() {
        //     self.headers = header.split(',').map(|s| s.trim().to_string()).collect();
        // }

        // let output = vec![Wallet::new(6, "csv from str".to_owned())];
        let output = sup_mt940::parse_mt940_alt(input);
        Ok(output?)
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

    fn account_to_yaml_bytes(&self, account_data: Wallet) -> Vec<u8> {
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
    // Csv,
    CsvExtraFin,
    // Xml,
    Camt053,
    Mt940,
}

impl fmt::Display for InputParserFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            // InputParserFormat::Csv => write!(f, "csv"),
            InputParserFormat::CsvExtraFin => write!(f, "csv_extra_fin"),
            // InputParserFormat::Xml => write!(f, "Xml"),
            InputParserFormat::Mt940 => write!(f, "mt_940"),
            InputParserFormat::Camt053 => write!(f, "camt_053"),
        }
    }
}

impl std::str::FromStr for InputParserFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            // "csv" => Ok(InputParserFormat::Csv),
            "csv_extra_fin" => Ok(InputParserFormat::CsvExtraFin),
            // "xml" => Ok(InputParserFormat::Xml),
            "camt_053" => Ok(InputParserFormat::Camt053),
            "mt_940" => Ok(InputParserFormat::Mt940),
            _ => Err(format!(
                "Unsupported format: {}. Supported: csv_extra_fin, camt_053, mt_940",
                s
            )),
        }
    }
}

impl InputParserFormat {
    pub fn all_variants() -> &'static [InputParserFormat] {
        &[
            // InputParserFormat::Csv,
            InputParserFormat::CsvExtraFin,
            // InputParserFormat::Xml,
            InputParserFormat::Mt940,
            InputParserFormat::Camt053
        ]
    }
}

#[derive(Debug, Clone, strum_macros::EnumString)]
pub enum OutputParserFormat {
    // #[strum(serialize = "csv")]
    // Csv,
    #[strum(serialize = "csv_extra_fin", serialize = "CsvExtraFin")]
    CsvExtraFin,
    #[strum(serialize = "yaml")]
    Yaml,
    // Xml,
    #[strum(serialize = "camt_053")]
    Camt053,
    #[strum(serialize = "mt_940")]
    Mt940,
}
impl fmt::Display for OutputParserFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            // OutputParserFormat::Csv => write!(f, "csv"),
            OutputParserFormat::CsvExtraFin => write!(f, "csv_extra_fin"),
            OutputParserFormat::Yaml => write!(f, "yaml"),
            OutputParserFormat::Mt940 => write!(f, "mt_940"),
            OutputParserFormat::Camt053 => write!(f, "camt_053"),
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
            // OutputParserFormat::Csv,
            OutputParserFormat::CsvExtraFin,
            OutputParserFormat::Yaml,
            OutputParserFormat::Mt940,
            OutputParserFormat::Camt053,
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
    log_dir: std::path::PathBuf
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
            log_dir: std::path::PathBuf::from("output"),
        }
    }

    // Internal method: parse CSV and generate YAML bytes
    fn process_data(&mut self) {
        if self.flushed {
            return;
        }

        let mut parser: UniParser = UniParser::default();
        parser.log_dir = self.log_dir.clone();
        let result_statement_data  = match self.process_input_type {
            // InputParserFormat::Csv => parser.parse_csv_from_str(&self.input_buffer),
            InputParserFormat::CsvExtraFin => {
                parser.parse_csv_extra_fin_from_str(&self.input_buffer)
            }
            // InputParserFormat::Xml => parser.parse_csv_from_str(&self.input_buffer),
            InputParserFormat::Camt053 => parser.parse_camt053_from_str(&self.input_buffer),
            InputParserFormat::Mt940 => parser.parse_mt940_from_str(&self.input_buffer),
        };

        // let parsed_account_data = Box::new(result_statement_data.unwrap());
        let parsed_account_data = result_statement_data.unwrap();
        // let todo_clone_parsed_account_data = parsed_account_data.clone();
        // let log_dir = std::path::Path::new("output");
        // for mut cash_statement_data in todo_clone_parsed_account_data {
        let mut iter_count = 0;
        let mut report_string: String = serde_yaml::to_string(&parsed_account_data).unwrap();
        // for cash_statement_data in &parsed_account_data {
        //     // let debug_data: Wallet = cash_statement_data.make_debug_as_yaml();
        //     // report_string += &serde_yaml::to_string(&cash_statement_data).unwrap().to_owned();
        //     dbg(cash_statement_data);
        //     // iter_count += 1;
        // }


        let gen_output_name = format!("from_{}_to_{}_{}.yaml", self.process_input_type, self.process_output_type, gen_time_prefix_to_filename());
        let output_path = self.log_dir.join(gen_output_name);
        
        dbg!(&output_path);
        std::fs::create_dir_all(&self.log_dir).unwrap();
        let mut file = std::fs::File::create(output_path).unwrap();
        file.write_all(report_string.as_bytes());


        // std::fs::write(&output_path, yaml_string)
        //     .with_context(|| format!("Failed to write YAML: {}", output_path))?;


        // self.output_bytes = parser.account_to_yaml_bytes(account_data);
        let rendered_result = match self.process_output_type {
            // OutputParserFormat::Csv => render::render_content_as_csv(parsed_account_data),
            OutputParserFormat::CsvExtraFin => {
                render::render_content_as_csv_extra_fin(parsed_account_data)
            }
            OutputParserFormat::Yaml => render::render_content_as_yaml(parsed_account_data),
            OutputParserFormat::Camt053 => render::render_content_as_camt053(parsed_account_data),
            OutputParserFormat::Mt940 => render::render_content_as_mt940(parsed_account_data),
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

use crate::parser::common::gen_time_prefix_to_filename;
// use quick_xml::events::Event;

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
