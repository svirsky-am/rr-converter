use serde::Serialize;
use regex::Regex;
use std::collections::HashMap;

use crate::parser::{Wallet, common::{Balance, BalanceAdjustType, Transaction}};

#[derive(Serialize, Debug)]
pub struct Mt940Message {
    transaction_reference: String, // :20:
    account_identification: String, // :25:
    statement_number: String,       // :28C:
    opening_balance: Option<Balance>,
    closing_balance: Option<Balance>,
    transactions: Vec<Transaction>,
}

// #[derive(Serialize, Debug)]
// struct Balance {
//     dc_mark: char,      // C or D
//     date: String,       // YYMMDD -> we keep as string
//     currency: String,
//     amount: String,     // e.g., "444,29"
// }

// #[derive(Serialize, Debug)]
// struct Transaction {
//     value_date: String,   // YYMMDD
//     entry_date: String,   // YYMMDD (optional in some cases but present here)
//     debit_credit: char,   // C or D
//     amount: String,
//     transaction_type: String,
//     reference: String,
//     description: String,
// }

pub fn parse_mt940_alt(input: &str) -> anyhow::Result<Vec<Wallet>>  {
    let mut statement_data_vec: Vec<Wallet>  = Vec::new();
    // let mut transactions = Vec::new();
    // let re_msg = Regex::new(r"\{1:[^}]*\}\{2:[^}]*\}\{3:[^}]*\}\{4:\n?([^}]*)\}-\}\{5:[^}]*\}").unwrap();
    print!("TEST_TEST2");
    // let re_msg_all = Regex::new(r"[\[\(]1\:(.*)\}2\:(.*)\}4\:(\n?\n?[^}]*)").unwrap();
    // let re_msg_all = Regex::new(r"\{1:[^}]*\}\{2:[^}]*\}\{3:[^}]*\}\{4:\n?([^}]*)\}-\}\{5:[^}]*\}").unwrap();


    // let re_msg_all = Regex::new(r"\{1:[^}]*\}\{2:[^}]*\}").unwrap();
    let re_msg_all = Regex::new(r"[\[\(\{]1\:(.*)\}\{2\:(.*)\}\{3:[^}]*\}\{4:\n?([:\w\n\d\/, ]*)").unwrap();



    // let re_msg_all = Regex::new(r"[\[\{\(]1\:(.*)").unwrap();
    // let re_msg_4 = Regex::new(r".*\}4\:(\n?\n?[^}]*)").unwrap();
    let re_lines = Regex::new(r"(:\d{2}[A-Z]?:)").unwrap();

    // let mut messages = Vec::new();
    
    for cap in re_msg_all.captures_iter(input) {
        let body = &cap[3];
        // dbg!( &body);
        let mut fields: Vec<(&str, String)> = Vec::new();

        // Split into tagged fields, respecting multi-line values
        let mut last_tag = "";
        let mut current_value = String::new();

        for line in body.lines() {
            if let Some(m) = re_lines.find(line) {
                // Save previous field
                if !last_tag.is_empty() {
                    fields.push((last_tag, current_value.trim_end().to_string()));
                }
                last_tag = m.as_str();
                current_value = line[m.end()..].to_string();
            } else {
                // Continuation line
                current_value.push('\n');
                current_value.push_str(line);
            }
        }
        if !last_tag.is_empty() {
            fields.push((last_tag, current_value.trim_end().to_string()));
        }

        // Build a map for easy access
        let field_map: HashMap<_, _> = fields.into_iter().collect();

        let mut transactions = Vec::new();
        let mut i = 0;
        while i < field_map.len() {
            // Not efficient, but safe: scan in order
            i += 1;
        }

        // Collect all :61: and corresponding :86:
        let mut field_iter = field_map.iter().peekable();
        let account_identification = field_map.get(":25:").cloned().unwrap_or_default();
        while let Some((tag, value)) = field_iter.next() {
            if *tag == ":61:" {
                let tx = parse_61(value);
                let mut desc = String::new();

                

                // Peek next for :86:
                if let Some((next_tag, next_value)) = field_iter.peek() {
                    if **next_tag == ":86:" {
                        desc = (*next_value).clone();
                        field_iter.next(); // consume :86:
                    }
                }

                transactions.push(Transaction {
                    purpose: desc,
                    ..tx
                });
            }
        }
        dbg!(&transactions);
        let account_servicer = field_map.get("20:").cloned().unwrap_or_default(); // sending bank 
        dbg!(&account_servicer);
        // messages.push(Mt940Message {
        //     transaction_reference,
        //     account_identification,
        //     statement_number: field_map.get(":28C:").cloned().unwrap_or_default(),
        //     opening_balance: field_map.get(":60F:").map(|v| parse_60f(v)),
        //     closing_balance: field_map.get(":62F:").map(|v| parse_60f(v)),
        //     transactions,
        // });
        let mut todo_cash_data = Wallet::default();
        todo_cash_data.account = account_identification;
        todo_cash_data.statement_id = field_map.get(":28C:").cloned().unwrap_or_default();
        todo_cash_data.opening_balance = field_map.get(":60F:").map(|v| parse_60f(v));
        todo_cash_data.closing_balance = field_map.get(":62F:").map(|v| parse_60f(v));
        // todo_cash_data.transactions = transactions;
        // todo_cash_data.opening_balance = Some(field_map.get(":60F:").map(|v| parse_60f(v)));
        statement_data_vec.push(todo_cash_data);
        // dbg!(&messages);

    }
    statement_data_vec.push(Wallet::default());
    // messages
    Ok(statement_data_vec)
}

fn parse_60f(s: &str) -> Balance {
    // Format: CYYMMDDCCYAMOUNT
    // Example: C200101EUR444,29
    let dc_mark = s.chars().next().unwrap_or('C');
    let credit_debit = match dc_mark {
        'D' => BalanceAdjustType::Debit,
        'C' => BalanceAdjustType::Credit,
        _ => BalanceAdjustType::Credit,
    };
    let rest = &s[1..];
    let date = rest[..6].to_string();
    let currency = rest[6..9].to_string();
    let amount = rest[9..].to_string().parse::<f64>().unwrap_or(0.0);
    // Balance { dc_mark, date, currency, amount }
    Balance { amount,
        currency,
        date,
        country: "pakistan".to_owned(),
        credit_debit,
        last_ops: Vec::new()
    }
}

fn parse_61(s: &str) -> Transaction {
    // Format: YYMMDD[MMDD][C/D]AMOUNT[TYPE][REF]
    // But actual: 0909250925DR583,92NMSC1110030403010139//1234
    // Or: 2001050105C1000,00NIOBNL56ASNB9999999999
    let s = s.trim();
    let mut chars = s.chars().peekable();

    // First 6: value date YYMMDD
    let value_date: String = chars.by_ref().take(6).collect();
    // Next 4 (optional): entry date MMDD → but in MT940 it's often same length; in your sample it's always 4 more
    // We assume 4-digit entry date if present (total 10 chars before C/D)
    let date = match  s.len() >= 10 {
        true => format!("{}_{}", value_date, s[6..10].to_string()),
        false => value_date,
            };

    // Now find C/D
    let mut pos = 10;
    let mut credit_debit = BalanceAdjustType::Credit;
    if s.len() > 10 {
        if let Some(c) = s.chars().nth(10) {
            if c == 'C' || c == 'D' {
                credit_debit = BalanceAdjustType::Credit;
                pos = 11;
            } else if let Some(c2) = s.chars().nth(11) {
                if c2 == 'C' || c2 == 'D' {
                    credit_debit = BalanceAdjustType::Debit;
                    pos = 12;
                }
            }
        }
    }

    // Amount: from pos until first non-amount char (digits, comma, dot)
    let amount_end = s[pos..]
        .char_indices()
        .find(|(_, c)| !c.is_ascii_digit() && *c != ',' && *c != '.')
        .map(|(i, _)| pos + i)
        .unwrap_or(s.len());
    let amount = s[pos..amount_end].to_string().parse::<f64>().unwrap_or(0.0f64);

    // let debit_amount = if !debit_str.is_empty() {
    //     Some(debit_str.parse::<f64>()?)
    // } else {
    //     None
    // };

    // Rest is transaction type + reference
    let rest = &s[amount_end..];
    let (transaction_type, reference) = if rest.len() >= 4 {
        (Some(rest[..4].to_string()), rest[4..].to_string())
    } else {
        (Some(rest.to_string()), String::new())
    };
    dbg!(&reference);

    Transaction {
        id: 0,
        currency: "EUR".to_owned(),
        date,
        credit_debit,
        amount,
        transaction_type,
        credit_account: String::new(),
        debit_account: String::new(),
        target_bank: reference, 
        purpose: String::new(), // filled later


    }
}
