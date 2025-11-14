use std::fs;
use serde::Serialize;
use anyhow::{Context, Result};

mod alt_mt_940_parser;

// use swift_mt_message::{SwiftParser, messages::MT940} ;
// use swift_mt_message::mt940;
// use swift_mt_message::types::{Currency, CreditDebitMark};
// // use swift_mt_message::types::{Currency, Amount, CreditDebitMark};
// use swift_mt_message::mt940::Message as Mt940Message;
// use swift_mt_message::mt940::StatementMessage;
// use swift_mt_message::mt940::StatementMessage;

// use swift_mt_message::mt940::StatementMessage;
// use swift_mt_message::types::{CreditDebitMark, Currency};

// ====== Common Data Model ======
#[derive(Serialize, Debug, Clone)]
pub struct StatementData {
    pub account: String,
    pub currency: String,
    pub statement_id: String,
    pub creation_time: Option<String>, // Only in CAMT
    pub opening_balance: Option<Balance>,
    pub closing_balance: Option<Balance>,
    pub transactions: Vec<Transaction>,
}

#[derive(Serialize, Debug, Clone)]
pub struct Balance {
    pub amount: f64,
    pub currency: String,
    pub credit_debit: String, // "CRDT" / "DBIT" or "C"/"D"
    pub date: String,         // YYYY-MM-DD
}

#[derive(Serialize, Debug, Clone)]
pub struct Transaction {
    pub value_date: String,
    pub amount: f64,
    pub currency: String,
    pub credit_debit: String,
    pub narrative: Vec<String>,
}

// ====== Parsers ======
#[derive(Debug)]
pub enum InputFormat {
    Camt053,
    Mt940,
}



pub struct Parser;

impl Parser {
    pub fn detect_format(content: &str) -> InputFormat {
        if content.trim_start().starts_with('<') {
            InputFormat::Camt053
        } else {
            InputFormat::Mt940
        }
    }

    pub fn parse_camt053(content: &str) -> Result<StatementData> {
        use roxmltree::{Document, Node};
        const NS: &str = "urn:iso:std:iso:20022:tech:xsd:camt.053.001.02";

        let doc = Document::parse(content)?;
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

        // Balances
        let mut balances = Vec::new();
        for bal in stmt.children().filter(|n| n.has_tag_name((NS, "Bal"))) {
            let code = find_nested_text(bal, &[(NS, "Tp"), (NS, "CdOrPrtry"), (NS, "Cd")]);
            let amt_node = bal.children().find(|n| n.has_tag_name((NS, "Amt")))
                .context("Balance missing Amt")?;
            let amount: f64 = amt_node.text().unwrap_or("0").parse().unwrap_or(0.0);
            let amt_ccy = amt_node.attribute("Ccy").unwrap_or(&currency).to_string();
            let credit_debit = get_text(bal, (NS, "CdtDbtInd"));
            let date = find_nested_text(bal, &[(NS, "Dt"), (NS, "Dt")]);

            balances.push((code, Balance {
                amount,
                currency: amt_ccy,
                credit_debit,
                date,
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
            let credit_debit = get_text(ntry, (NS, "CdtDbtInd"));
            let value_date = find_nested_text(ntry, &[(NS, "ValDt"), (NS, "Dt")]);

            let mut narratives = Vec::new();
            if let Some(rmt) = ntry.children().find(|n| n.has_tag_name((NS, "RmtInf"))) {
                for ustrd in rmt.children().filter(|n| n.has_tag_name((NS, "Ustrd"))) {
                    if let Some(text) = ustrd.text() {
                        narratives.push(text.trim().to_string());
                    }
                }
            }

            transactions.push(Transaction {
                value_date,
                amount,
                currency: currency.clone(),
                credit_debit,
                narrative: narratives,
            });
        }

        Ok(StatementData {
            account,
            currency,
            statement_id: msg_id,
            creation_time: Some(cre_dt_tm),
            opening_balance,
            closing_balance,
            transactions,
        })
    }

    pub fn parse_mt940(content: &str) -> Result<StatementData> {
        


        alt_mt_940_parser::parse_mt940_alt(content);

        // let message: swift_mt_message::SwiftMessage<_> = SwiftParser::parse(content).unwrap();

        // use swift_mt_message::mt940::Message as Mt940Message;
        
        // mt940::MT940::parse_from_block4(block4)
        // let message = mt940::parse(content)
        // .with_context(|| "Failed to parse MT940")?;
        // let message = swift_mt_message::mt940::Message::from_str(content)
        // .with_context(|| "Failed to parse MT940")?;
        // let message = StatementMessage::from_str(content)?;
        // let message = mt940::parse(content)
        // .with_context(|| "Failed to parse MT940")?;

        // mt940::Message::from_str(content).with_context(|| "Failed to parse MT940")?;
    
        // let message = swift_mt_message::mt940::parse(content)
        //     .with_context(|| "Failed to parse MT940")?;
    
        // let stmt = message.statement
        //     .first()
        //     .context("MT940 contains no statement")?;
    
        // let account = stmt.account.clone();
        let account = "account".to_string();

        let currency = "currency".to_string();
        // let currency = stmt.opening_balance
        //     .as_ref()
        //     .map(|b| b.currency)
        //     .or_else(|| stmt.closing_balance.as_ref().map(|b| b.currency))
        //     .unwrap_or(Currency::from_str("XXX").unwrap_or(Currency::from_u32(999).unwrap()));
        // let currency = stmt.opening_balance
        //     .as_ref()
        //     .map(|b| b.currency)
        //     .or_else(|| stmt.closing_balance.as_ref().map(|b| b.currency))
        //     .unwrap_or("test".to_string());


        let opening_balance = Some(            
            Balance{
            amount: 99f64,
            currency: "currency".to_string(),
            // credit_debit: if b.indicator.is_credit() { "CRDT".to_string() } else { "DBIT".to_string() },
            credit_debit:  "DBIT".to_string(),
            date: "date".to_string(),
        });
        // let opening_balance = stmt.opening_balance.as_ref().map(|b| Balance {
        //     amount: b.amount.value,
        //     currency: b.currency.to_string(),
        //     credit_debit: if b.indicator.is_credit() { "CRDT".to_string() } else { "DBIT".to_string() },
        //     date: b.date.to_string(), // e.g., "230420"
        // });

        let closing_balance = Some(
            Balance{
                amount: 44f64,
                currency: "currency".to_string(),
                // credit_debit: if b.indicator.is_credit() { "CRDT".to_string() } else { "DBIT".to_string() },
                credit_debit:  "CRDT".to_string(),
                date: "date".to_string(),
            }
            
        );
        // let closing_balance = stmt.closing_balance.as_ref().map(|b| Balance {
        //     amount: b.amount.value,
        //     currency: b.currency.to_string(),
        //     credit_debit: if b.indicator.is_credit() { "CRDT".to_string() } else { "DBIT".to_string() },
        //     date: b.date.to_string(),
        // });
    
        let mut transactions = Vec::new();
        // for line in &stmt.statement_lines {
        //     let credit_debit = if line.amount.credit_debit_mark == CreditDebitMark::Credit {
        //         "CRDT"
        //     } else {
        //         "DBIT"
        //     };

        //     // let credit_debit = "CRDT";
    
    
        //     // Narrative is a single string; split into lines if needed
        //     let narrative = if line.narrative.is_empty() {
        //         vec![]
        //     } else {
        //         line.narrative.lines().map(|s| s.to_string()).collect()
        //     };
    
        //     transactions.push(Transaction {
        //         value_date: line.value_date.to_string(),
        //         amount: line.amount.value,
        //         currency: line.amount.currency.to_string(),
        //         credit_debit: credit_debit.to_string(),
        //         narrative,
        //     });
        // }
    
        Ok(StatementData {
            account,
            currency: currency.to_string(),
            statement_id: "statement_id".to_string(),
            creation_time: None,
            opening_balance,
            closing_balance,
            transactions,
        })
    }
}

// ====== Helpers ======
fn get_text(node: roxmltree::Node, tag: (&str, &str)) -> String {
    node.children()
        .find(|n| n.has_tag_name(tag))
        .and_then(|n| n.text())
        .unwrap_or("")
        .trim()
        .to_string()
}

fn find_nested_text(parent: roxmltree::Node, path: &[(&str, &str)]) -> String {
    let mut current = parent;
    for &tag in path {
        current = match current.children().find(|n| n.has_tag_name(tag)) {
            Some(n) => n,
            None => return String::new(),
        };
    }
    current.text().unwrap_or("").trim().to_string()
}


fn test_parsr_xml() -> Result<()> {
    let input_path = "tests/test_files/camt053_dk_example.xml";
    let xml_content = fs::read_to_string(input_path)?;
    let format = Parser::detect_format(&xml_content);
    let data = match format {
        InputFormat::Camt053 => Parser::parse_camt053(&xml_content)?,
        InputFormat::Mt940 => Parser::parse_mt940(&xml_content)?,
    };
   
    let yaml_output = serde_yaml::to_string(&data)?;
    let another_path = std::path::Path::new(input_path);
    let output_name = "test_parse".to_string();
    let output_path = format!("output/{}_parsed.yaml", output_name);
    fs::write(&output_path, yaml_output)
        .with_context(|| format!("Failed to write YAML: {}", output_path))?;
    // println!("✅ Parsed {} as {:?} → {}", input_path, format, output_path);
    // fs::write("output/output_from_camt53.yaml", yaml_output)?;
    Ok(())
}

fn example_use_uniparser(input_path: &std::path::Path, input_format: InputFormat) -> Result<()> {
    // let input_path = "tests/test_files/MT940_github_1.mt940";

    // let input_path = "tests/test_files/MT_940_aiophotoz.mt940";

    

    
    let input_content = fs::read_to_string(&input_path)?;
    // let format = Parser::detect_format(&xml_content);
    // let format = InputFormat::Mt940;
    
    let data = match input_format {
        InputFormat::Camt053 => Parser::parse_camt053(&input_content)?,
        InputFormat::Mt940 => Parser::parse_mt940(&input_content)?,
    };
   
    let yaml_output = serde_yaml::to_string(&data)?;
    let gen_output_name = input_path.file_name().unwrap().to_str().unwrap() ;
    
    // let output_name = "test_parse2".to_string();
    let output_path = format!("output/{}_parsed2.yaml", gen_output_name);
    dbg!(&output_path);
    fs::write(&output_path, yaml_output)
        .with_context(|| format!("Failed to write YAML: {}", output_path))?;
    // println!("✅ Parsed {} as {:?} → {}", input_path, format, output_path);
    // fs::write("output/output_from_camt53.yaml", yaml_output)?;
    Ok(())
}

// ====== Main ======
fn main() -> Result<()> {
        // let input_path = "tests/test_files/MT_940_aiophotoz.mt940";
   
    // typt_1_2_3_4
    let sample_mt940_type_1_2_3_4 = "tests/test_files/MT940_github_1.mt940";
    let _ = example_use_uniparser(std::path::Path::new(&sample_mt940_type_1_2_3_4),
      InputFormat::Mt940);


    let sample_camt053 = "tests/test_files/camt053_dk_example.xml";
    let _ = example_use_uniparser(std::path::Path::new(&sample_camt053), 
    InputFormat::Camt053);
    // use mt940::parse_mt940;
    // let input = "\
    // :20:3996-11-11111111\r\n\
    // :25:DABADKKK/111111-11111111\r\n\
    // :28C:00001/001\r\n\
    // :60F:C090924EUR54484,04\r\n\
    // :61:0909250925DR583,92NMSC1110030403010139//1234\r\n\
    // :86:11100304030101391234\r\n\
    // Beneficiary name\r\n\
    // Something else\r\n\
    // :61:0910010930DR62,60NCHGcustomer id//bank id\r\n\
    // :86:Fees according to advice\r\n\
    // :62F:C090930EUR53126,94\r\n\
    // :64:C090930EUR53189,31\r\n\
    // \r\n";
    // let input_path = "tests/test_files/MT940_github_1.mt940";
    // let input_path = "tests/test_files/MT_940_oracle.mt940";
    // use serde::Serialize;
    // use regex::Regex;
    // use std::collections::HashMap;
    // use mt_940_parser::parse_mt940;
    // let input_path = "tests/test_files/mt_940-gs.mt940";
    // let input = fs::read_to_string(input_path)?;
    // let messages = parse_mt940(&input);
    // let yaml = serde_yaml::to_string(&messages).expect("Failed to serialize to YAML");
    // println!("{}", yaml);
    
    
    
    // let mt940_content = fs::read_to_string(input_path)?;

    // let input_parsed = parse_mt940(&mt940_content).unwrap();
    // dbg!(&input_parsed);
    // assert_eq!(input_parsed[0].transaction_ref_no, "3996-11-11111111");    
    Ok(())
}