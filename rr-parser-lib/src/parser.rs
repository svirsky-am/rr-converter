use chrono::{DateTime, NaiveDate, NaiveDateTime};
// use roxmltree::Document;
use serde::Serialize;
// use std::io;
use std::io::{self, Read, Write};

use std::fmt;
use std::path::PathBuf;
mod common;
mod render;
mod sup_camp053;
mod sup_extra_fin_csv;
mod sup_mt940;
use common::{Balance, Transaction, parse_russian_date};

// In-memory parsed CSV
#[derive(Debug, Default)]
struct UniParser {
    // headers: Vec<String>,
    // rows: Vec<Vec<String>>,
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
    pub bank_maintainer: String,
    pub currency: String,
    pub account: String,
    pub statement_id: String,
    pub statement_period_start: NaiveDateTime,
    pub statement_period_end: NaiveDateTime,
    pub creation_time: Option<NaiveDateTime>, // Only in CAMT
    pub opening_balance: Option<Balance>,
    pub closing_balance: Option<Balance>,
    pub transactions: Vec<Transaction>,
}

impl Default for Wallet {
    fn default() -> Self {
        Wallet {
            id: 0,
            bank_maintainer: "default_bank_maintainer".to_owned(),
            description: String::new(),
            transactions: Vec::new(),
            currency: String::new(),
            account: String::new(),
            statement_id: String::new(),
            statement_period_start: DateTime::from_timestamp(0, 0).unwrap().naive_utc(),
            statement_period_end: DateTime::from_timestamp(4_102_444_800, 0).unwrap().naive_utc(),
            creation_time: Some( DateTime::from_timestamp(4_102_444_800, 0).unwrap().naive_utc(),),
            opening_balance: Some(Balance {
                amount: 0.0,
                currency: "default_currency".to_owned(),
                credit_debit: common::BalanceAdjustType::WithoutInfo,
                date: NaiveDate::from_ymd_opt(1951, 1, 1).unwrap(),
                last_ops: Vec::new(),
            }),
            closing_balance: Some(Balance {
                amount: 0.0,
                currency: "default_currency".to_owned(),
                credit_debit: common::BalanceAdjustType::WithoutInfo,
                date: NaiveDate::from_ymd_opt(1952, 2, 2).unwrap(),
                last_ops: Vec::new(),
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
}

impl UniParser {
    fn parse_csv_extra_fin_from_str(&mut self, input: &str) -> anyhow::Result<Vec<Wallet>> {
        let mut account_data = Wallet::new(7, "csv from str".to_owned());

        let parts: Vec<&str> = input.split(",,,,,,,,,,,,,,,,,,,,,,\n").collect();

        let sratemnts_header = parts[1];

        let currency_by_header = std::rc::Rc::new("TODO рубли".to_string());
        // let match_header_parser = regex::Regex::new(r"\,(.*)\,\,\,\,(\s+).*\n")?;
        let match_header_parser = regex::Regex::new(
            r"(?x)
            (?P<date>\b\d{2}\.\d{2}\.\d{4}\b)\,\,\,\,(?P<business>.+)\x20
            \b(?P<code>\d{2}\.\d{3}\.\d{2}-\d{4}\b)\x2c{17}\n
            \x2c(?P<bank_maintainer>[[^\x2c]\n\W\x22]{1, 40})\x2c{21}\n
            \x2cДата\x20формирования\x20выписки\x20(?P<data_creation>[\d\x20\x27\x2e\x3a]+в[\d\x20\x27\x2e\x3a]+).*\n
            \x2cВЫПИСКА\x20ОПЕРАЦИЙ\x20ПО\x20ЛИЦЕВОМУ\x20СЧЕТУ\x2c{11}(?P<client_id>\d{1,40})\x2c+\n
            \x2c+(?P<client_name>[[^\x2c]\W\x22\x20]{1, 60})\x2c{10}\n
            \x2c\x2cза\x20период\x20с\x20(?P<statement_period_start>\d{1,2}.{1,15}\d{4})\x20г.\x2c{12}\x20по\x20,(?P<statement_period_end>\d{1,2}\x20.{1,15}\d{4})\x20г.\x2c+\n
            \x2c\x2c(?P<currency>[^\x2c]{1,40})\x2c{10}

            "
        ).unwrap();
        if let Some(caps) = match_header_parser.captures(input) {
            let creation_time_str = String::from(&caps["data_creation"]);
            let result_creation_sate_time = NaiveDateTime::parse_from_str(&creation_time_str, "%d.%m.%Y в %H:%M:%S").unwrap();
            // dbg!(result_creation_sate_time);
            account_data.statement_id = String::from(&caps["code"]);
            account_data.bank_maintainer = String::from(&caps["bank_maintainer"]).trim().to_string();
            account_data.id = String::from(&caps["client_id"]).parse::<u128>()?;
            account_data.currency = String::from(&caps["currency"]);
            account_data.account = String::from(&caps["client_name"]);
            account_data.creation_time = Some(result_creation_sate_time);

            account_data.statement_period_start = parse_russian_date(&String::from(&caps["statement_period_start"])).unwrap().and_hms_opt(0, 0, 0).unwrap();
            account_data.statement_period_end = parse_russian_date(&String::from(&caps["statement_period_end"])).unwrap().and_hms_opt(0, 0, 0).unwrap();
            
            
            // dbg!(&caps);

        } else {
            println!("No full match");
        }
        let bracked_csv = parts[2];

        let output_dbg_sratemnts_header = self.log_dir.join(format!(
            "{}_extra_csv_sratemnts_header.txt",
            gen_time_prefix_to_filename()
        ));
        std::fs::create_dir_all(&self.log_dir).unwrap();
        let mut file_dbg_sratemnts_header =
            std::fs::File::create(output_dbg_sratemnts_header).unwrap();
        let _ = file_dbg_sratemnts_header.write_all(sratemnts_header.as_bytes());

        let output_dbg_bracked_csv = self.log_dir.join(format!(
            "{}_extra_csv_bracked.csv",
            gen_time_prefix_to_filename()
        ));
        let mut file_dbg_bracked_csv = std::fs::File::create(output_dbg_bracked_csv).unwrap();
        let _ = file_dbg_bracked_csv.write_all(bracked_csv.as_bytes());

        let normalyzed_csv_str = sup_extra_fin_csv::normalyze_csv_str(bracked_csv.to_owned());
        // dbg!(&normalyzed_csv_str);
        let output_normalyzed_csv = self
            .log_dir
            .join(format!("{}_normalyzed.csv", gen_time_prefix_to_filename()));
        let mut file_output_normalyzed_csv = std::fs::File::create(output_normalyzed_csv).unwrap();
        let _ = file_output_normalyzed_csv.write_all(normalyzed_csv_str.as_bytes());

        let transactions = sup_extra_fin_csv::parsr_csv_str(normalyzed_csv_str.to_owned())?;

        account_data.transactions = transactions;
        let sratemnts_balance_ending = parts[4];
        // dbg!(sratemnts_balance_ending);
        // (:\d{2}[A-Z]?:)
        // \,\,\,\,\,\,
        let match_input_balance = regex::Regex::new(
            r".*(\,Входящий остаток\,\,\,\,\,\,.{0,10}\,\,\,\,)(.{0,10})\,\,\,\,\,\,\(П\)\,\,(.*) г.\,\,\,\n.*\n(\,Исходящий остаток)\,\,\,\,\,\,.{0,10}\,\,\,\,(.{0,10})\,\,\,\,\,\,\(П\)\,\,(.*) г.\,\,\,\n.*",
        )?;
        for cap in match_input_balance.captures_iter(sratemnts_balance_ending) {
            let input_balance = &cap[2];
            let date_of_input_balance = &cap[3];

            let parsed_input_date = parse_russian_date(date_of_input_balance);

            // account_data.opening_balance = Some(input_balance.parse::<f64>()?);
            let output_balance = &cap[5];
            let date_of_output_balance = &cap[6];
            let parsed_output_date = parse_russian_date(date_of_output_balance);
            // dbg!(&parsed_output_date);

            // dbg!(output_balance);
            // let copy_currency = currency.clone();
            account_data.opening_balance = Some(Balance {
                amount: input_balance.parse::<f64>()?,
                // currency: "default_currency".to_owned(),
                credit_debit: common::BalanceAdjustType::WithoutInfo,
                date: parsed_input_date.unwrap(),
                last_ops: Vec::new(),
                currency: currency_by_header.to_string(),
            });
            account_data.closing_balance = Some(Balance {
                amount: output_balance.parse::<f64>()?,
                // currency: "default_currency".to_owned(),
                credit_debit: common::BalanceAdjustType::WithoutInfo,
                date: parsed_output_date.unwrap(),
                last_ops: Vec::new(),
                currency: currency_by_header.to_string(),
            });
        }

        let output = vec![account_data];
        Ok(output)
    }

    fn parse_camt053_from_str(&mut self, input: &str) -> anyhow::Result<Vec<Wallet>> {
        const NS: &str = "urn:iso:std:iso:20022:tech:xsd:camt.053.001.02";
        use roxmltree::Document;
        use sup_camp053::{find_nested_text, get_text};

        use anyhow::Context;
        // let content = input;
        let doc = Document::parse(input).unwrap();
        let root = doc.root_element();

        let bk_to_cstmr_stmt = root
            .children()
            .find(|n| n.has_tag_name((NS, "BkToCstmrStmt")))
            .context("Missing BkToCstmrStmt")?;

        let grp_hdr = bk_to_cstmr_stmt
            .children()
            .find(|n| n.has_tag_name((NS, "GrpHdr")))
            .context("Missing GrpHdr")?;
        let msg_id = get_text(grp_hdr, (NS, "MsgId"));
        let cre_dt_tm = get_text(grp_hdr, (NS, "CreDtTm"));
        let creation_time =Some( NaiveDateTime::parse_from_str(&cre_dt_tm, "%Y-%m-%dT%H:%M:%S").unwrap());

        let stmt = bk_to_cstmr_stmt
            .children()
            .find(|n| n.has_tag_name((NS, "Stmt")))
            .context("Missing Stmt")?;

        let acct = stmt
            .children()
            .find(|n| n.has_tag_name((NS, "Acct")))
            .context("Missing Acct")?;

        let bank_maintainer = find_nested_text(acct, &[(NS, "Nm")]);

        let iban = find_nested_text(acct, &[(NS, "Id"), (NS, "IBAN")]);
        let currency = get_text(acct, (NS, "Ccy"));
        let account = if iban.is_empty() {
            "UNKNOWN".to_string()
        } else {
            iban
        };

        let fr_to_dt = stmt
            .children()
            .find(|n| n.has_tag_name((NS, "FrToDt")))
            .context("Missing FrToDt")?;
        let statement_period_start_str = find_nested_text(fr_to_dt, &[(NS, "FrDtTm")]);
        let statement_period_start = NaiveDateTime::parse_from_str(&statement_period_start_str, "%Y-%m-%dT%H:%M:%S").unwrap();
        let statement_period_end_str = find_nested_text(fr_to_dt, &[(NS, "ToDtTm")]);
        let statement_period_end = NaiveDateTime::parse_from_str(&statement_period_end_str, "%Y-%m-%dT%H:%M:%S").unwrap();
        let id: u128 = 0;

        // Balances
        let mut balances = Vec::new();
        // let last_ops_vec: Vec<common::OpKind> = Vec::new();
        // let last_ops = last_ops_vec.to_owned();
        for bal in stmt.children().filter(|n| n.has_tag_name((NS, "Bal"))) {
            let code = find_nested_text(bal, &[(NS, "Tp"), (NS, "CdOrPrtry"), (NS, "Cd")]);
            let amt_node = bal
                .children()
                .find(|n| n.has_tag_name((NS, "Amt")))
                .context("Balance missing Amt")?;
            let amount: f64 = amt_node.text().unwrap_or("0").parse().unwrap_or(0.0);
            let amt_ccy = amt_node.attribute("Ccy").unwrap_or(&currency).to_string();
            let credit_debit = match get_text(bal, (NS, "CdtDbtInd")).as_str() {
                "CRDT" => common::BalanceAdjustType::Credit,
                "DBIT" => common::BalanceAdjustType::Debit,
                _ => common::BalanceAdjustType::Debit, // or consider logging/warning for unexpected values
            };

            let date_str = find_nested_text(bal, &[(NS, "Dt"), (NS, "Dt")]);
            let date =
                NaiveDate::parse_from_str(&date_str, "%Y-%m-%d").expect("Failed to parse date");
            balances.push((
                code,
                Balance {
                    amount,
                    currency: amt_ccy,
                    credit_debit,
                    date,
                    // country,
                    last_ops: Vec::new(),
                },
            ));
        }

        let opening_balance = balances
            .iter()
            .find(|(code, _)| code == "OPBD")
            .map(|(_, b)| b.clone());
        let closing_balance = balances
            .iter()
            .find(|(code, _)| code == "CLBD")
            .map(|(_, b)| b.clone());

        // Transactions
        let mut transactions = Vec::new();
        for ntry in stmt.children().filter(|n| n.has_tag_name((NS, "Ntry"))) {
            let amt_node = ntry
                .children()
                .find(|n| n.has_tag_name((NS, "Amt")))
                .context("Entry missing Amt")?;
            let amount: f64 = amt_node.text().unwrap_or("0").parse().unwrap_or(0.0);
            let currency = amt_node.attribute("Ccy").unwrap_or("").to_string();
            let currency = if currency.is_empty() {
                currency.clone()
            } else {
                currency
            };
            let credit_debit = match get_text(ntry, (NS, "CdtDbtInd")).as_str() {
                "DBIT" => common::BalanceAdjustType::Debit,
                "CRDT" => common::BalanceAdjustType::Credit,
                _ => common::BalanceAdjustType::WithoutInfo,
            };
            // RltdDts
            // AccptncDtTm

            // (input, "%Y-%m-%dT%H:%M:%S")

            // let debit_account: String = "TODO debit_account".to_string();
            // let credit_account: String = "TODO credit_account".to_string(); //<BkTxCd Prtry
            let bk_tx_cd_prtry_tag = ntry.descendants().find(|n| n.tag_name().name() == "Prtry").unwrap();
            let cd_target_tr_tag_text = bk_tx_cd_prtry_tag.children()
                .find(|n| n.has_tag_name((NS, "Cd")))
                .context("Balance missing Prtry")?.text().unwrap();
            // dbg!(cd_target_tr_tag);
            let (debit_account, credit_account) = match credit_debit{
                common::BalanceAdjustType::Debit => (cd_target_tr_tag_text.to_string(), account.clone()),
                common::BalanceAdjustType::Credit => (account.clone(), cd_target_tr_tag_text.to_string()),
                common::BalanceAdjustType::WithoutInfo => (account.clone(), cd_target_tr_tag_text.to_string()),
            };
            // bk_tx_cd_tag.children()
            // let target_bank: String = "TODO target bank".to_string();
            // let purpose = "TODO target bank".to_string();
            // AddtlTxInf

            // let mut narratives = Vec::new();
            // if let Some(rmt) = ntry.children().find(|n| n.has_tag_name((NS, "RmtInf"))) {
            //     for ustrd in rmt.children().filter(|n| n.has_tag_name((NS, "Ustrd"))) {
            //         if let Some(text) = ustrd.text() {
            //             narratives.push(text.trim().to_string());
            //         }
            //     }
            // }
            // dbg!(&narratives);
            // let purpose = "TODO target bank".to_string();
            // let purpose = sup_camp053::get_text_of_deep_child_node(ntry, "RltdPties");
            // dbg!(&purpose);
            let purpose = "TODO parse RltdPties".to_string();
            let sub_fmly_cd = sup_camp053::get_text_of_deep_child_node(ntry, "SubFmlyCd").unwrap();
            let accptnc_dt_tm_str =
                sup_camp053::get_text_of_deep_child_node(ntry, "AccptncDtTm").unwrap();
            let service_bank = sup_camp053::get_text_of_deep_child_node(ntry, "AcctSvcrRef")
                .unwrap()
                .to_string();

            let date_time =
                NaiveDateTime::parse_from_str(accptnc_dt_tm_str, "%Y-%m-%dT%H:%M:%S").unwrap();
            // dbg!(date_time);

            transactions.push(Transaction {
                id,
                // &&country,
                credit_account,
                debit_account,
                date_time,
                amount,
                currency,
                credit_debit,
                // narrative: narratives,
                service_bank,
                purpose,
                transaction_type: Some(sub_fmly_cd.to_owned()),
            });
        }

        let id: u128 = 0;
        let description: String = "0".to_owned();
        let output = vec![Wallet {
            id,
            bank_maintainer,
            description,
            account,
            currency,
            statement_id: msg_id,
            statement_period_start,
            statement_period_end,
            creation_time,
            opening_balance,
            closing_balance,
            transactions,
        }];
        Ok(output)
        // Ok()
        // dbg!(&data_transactions);
        // account_data
    }

    fn parse_mt940_from_str(&mut self, input: &str) -> anyhow::Result<Vec<Wallet>> {
        
        sup_mt940::parse_mt940_alt(input)
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
            InputParserFormat::Camt053,
        ]
    }
}

#[derive(Debug, Clone, strum_macros::EnumString)]
pub enum OutputParserFormat {
    #[strum(serialize = "csv_extra_fin", serialize = "CsvExtraFin")]
    CsvExtraFin,
    #[strum(serialize = "yaml")]
    Yaml,
    #[strum(serialize = "camt_053")]
    Camt053,
    #[strum(serialize = "mt_940")]
    Mt940,
}
impl fmt::Display for OutputParserFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OutputParserFormat::CsvExtraFin => write!(f, "csv_extra_fin"),
            OutputParserFormat::Yaml => write!(f, "yaml"),
            OutputParserFormat::Mt940 => write!(f, "mt_940"),
            OutputParserFormat::Camt053 => write!(f, "camt_053"),
        }
    }
}

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

// #[derive(Debug)]
// pub enum FinConverterError {
//     Io(std::io::Error),
//     ParseError(String),
//     SerializeError(serde_yaml::Error),
//     AlreadyFlushed,
// }

// impl std::fmt::Display for FinConverterError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             FinConverterError::Io(e) => write!(f, "IO error: {}", e),
//             FinConverterError::ParseError(e) => write!(f, "Parse error: {}", e),
//             FinConverterError::SerializeError(e) => write!(f, "Serialization error: {}", e),
//             FinConverterError::AlreadyFlushed => write!(f, "Data already flushed"),
//         }
//     }
// }
// impl std::error::Error for FinConverterError {}

pub struct FinConverter {
    // Input state (for Write)
    process_input_type: InputParserFormat,
    process_output_type: OutputParserFormat,
    input_buffer: String,
    flushed: bool,

    // Output state (for Read)
    output_bytes: Vec<u8>,
    read_pos: usize,
    log_dir: std::path::PathBuf,
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
    // type Err = String;
    // Internal method: parse CSV and generate YAML bytes
    fn process_data(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.flushed {
            return Ok(());
        }

        let mut parser: UniParser = UniParser::default();
        parser.log_dir = self.log_dir.clone();
        let result_statement_data = match self.process_input_type {
            // InputParserFormat::Csv => parser.parse_csv_from_str(&self.input_buffer),
            InputParserFormat::CsvExtraFin => {
                parser.parse_csv_extra_fin_from_str(&self.input_buffer)
            }
            // InputParserFormat::Xml => parser.parse_csv_from_str(&self.input_buffer),
            InputParserFormat::Camt053 => parser.parse_camt053_from_str(&self.input_buffer),
            InputParserFormat::Mt940 => parser.parse_mt940_from_str(&self.input_buffer),
        };

        let parsed_account_data = result_statement_data.unwrap();
        let report_string: String = serde_yaml::to_string(&parsed_account_data).unwrap();

        let gen_output_name = format!(
            "from_{}_to_{}_{}.yaml",
            self.process_input_type,
            self.process_output_type,
            gen_time_prefix_to_filename()
        );
        let output_path = self.log_dir.join(gen_output_name);

        std::fs::create_dir_all(&self.log_dir).unwrap();
        let mut file = std::fs::File::create(output_path).unwrap();
        let _ = file.write_all(report_string.as_bytes());

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

        self.output_bytes = rendered_result?;
        let mut output_format_str = format!("output_format: {}\n", self.process_output_type)
            .as_bytes()
            .to_vec();
        let mut input_format_str = format!("input_format: {}\n", self.process_input_type)
            .as_bytes()
            .to_vec();
        self.output_bytes.append(&mut input_format_str);
        self.output_bytes.append(&mut output_format_str);

        self.flushed = true;
        Ok(())
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
        let _result = self.process_data(); // Parse and prepare YAML
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
