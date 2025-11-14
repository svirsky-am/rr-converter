use std::fs;
use serde::Serialize;
use anyhow::{Context, Result};

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
        let message = mt940::parse(content)
            .with_context(|| "Failed to parse MT940")?;
    
        let stmt = message.statement
            .first()
            .context("MT940 contains no statement")?;
    
        let account = stmt.account.clone();
        let currency = stmt.opening_balance
            .as_ref()
            .map(|b| b.currency)
            .or_else(|| stmt.closing_balance.as_ref().map(|b| b.currency))
            .unwrap_or(Currency::from_str("XXX").unwrap_or(Currency::from_u32(999).unwrap()));
    
        let opening_balance = stmt.opening_balance.as_ref().map(|b| Balance {
            amount: b.amount.value,
            currency: b.currency.to_string(),
            credit_debit: if b.indicator.is_credit() { "CRDT".to_string() } else { "DBIT".to_string() },
            date: b.date.to_string(), // e.g., "230420"
        });
    
        let closing_balance = stmt.closing_balance.as_ref().map(|b| Balance {
            amount: b.amount.value,
            currency: b.currency.to_string(),
            credit_debit: if b.indicator.is_credit() { "CRDT".to_string() } else { "DBIT".to_string() },
            date: b.date.to_string(),
        });
    
        let mut transactions = Vec::new();
        for line in &stmt.statement_lines {
            let credit_debit = if line.amount.credit_debit_mark == CreditDebitMark::Credit {
                "CRDT"
            } else {
                "DBIT"
            };
    
            // Narrative is a single string; split into lines if needed
            let narrative = if line.narrative.is_empty() {
                vec![]
            } else {
                line.narrative.lines().map(|s| s.to_string()).collect()
            };
    
            transactions.push(Transaction {
                value_date: line.value_date.to_string(),
                amount: line.amount.value,
                currency: line.amount.currency.to_string(),
                credit_debit: credit_debit.to_string(),
                narrative,
            });
        }
    
        Ok(StatementData {
            account,
            currency: currency.to_string(),
            statement_id: message.transaction_reference,
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

// ====== Main ======
fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <input-file>", args[0]);
        std::process::exit(1);
    }
    let input_path = &args[1];
    let content = fs::read_to_string(input_path)
        .with_context(|| format!("Failed to read file: {}", input_path))?;

    let format = Parser::detect_format(&content);
    let data = match format {
        InputFormat::Camt053 => Parser::parse_camt053(&content)?,
        InputFormat::Mt940 => Parser::parse_mt940(&content)?,
    };

    let yaml = serde_yaml::to_string(&data)?;
    let output_path = format!("{}.yaml", input_path);
    fs::write(&output_path, yaml)
        .with_context(|| format!("Failed to write YAML: {}", output_path))?;

    println!("✅ Parsed {} as {:?} → {}", input_path, format, output_path);
    Ok(())
}