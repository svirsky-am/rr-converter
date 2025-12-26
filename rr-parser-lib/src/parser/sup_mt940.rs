use chrono::{NaiveDate, NaiveDateTime};
use regex::Regex;
use std::collections::HashMap;

use crate::parser::{
    Wallet,
    common::{Balance, BalanceAdjustType, Transaction},
};

///:61
/// Value Date: 2009-09-25
/// Entry Date: 2009-09-25
/// Debit/Credit: D → Debit
/// Amount: 583,92
/// Transaction Type: N → Normal
/// Bank Code: MSC → Debit card / POS terminal
/// Reference: 1110030403010139//1234
pub fn parse_mt940_alt(input: &str) -> anyhow::Result<Vec<Wallet>> {
    let mut statement_data_vec: Vec<Wallet> = Vec::new();
    let re_msg_all =
        // Regex::new(r"[\[\(\{]1\:...(.{1,100})\{2\:.940(.{1,100})[N]\}.{0,40}\{4:\n?([:\x20\w\n\d\/, \s\S]{1,250})\}").unwrap();
        Regex::new(r"[\[\(\{]1\:...(.{1,100})\{2\:.940(\S{1,100})[N]\}.{0,40}\{4\:([^\}]{1,750})-\}\{5.*").unwrap();
    let re_lines = Regex::new(r"(:\d{2}[A-Z]?:)").unwrap();
    for cap in re_msg_all.captures_iter(input) {
        // dbg!(&cap);
        let bank_maintainer = &cap[2];
        // dbg!(&body);
        let body: &str = &cap[3];
        // dbg!(&body);
        let mut fields: Vec<(&str, String)> = Vec::new();

        let mut last_tag = "";
        let mut current_value = String::new();

        for line in body.lines() {
            if let Some(m) = re_lines.find(line.trim()) {
                if !last_tag.is_empty() {
                    fields.push((last_tag, current_value.trim_end().to_string()));
                }
                last_tag = m.as_str();
                current_value = line[m.end()..].to_string();
            } else {
                current_value.push('\n');
                current_value.push_str(line);
            }
        }
        if !last_tag.is_empty() {
            fields.push((last_tag, current_value.trim_end().to_string()));
        }

        let field_map: HashMap<_, _> = fields.into_iter().collect();
        // Field 20: Transaction reference number
        // Field 25: Account identification
        // Field 28C: Statement number / sequence
        // Field 60F: Opening balance
        // Field 61: Statement lines (individual transactions)
        // Field 86: Optional narrative for each transaction
        // Field 62F: Closing balance
        let mut i = 0;
        while i < field_map.len() {
            i += 1;
        }

        // let account_identification = field_map.get(":25:").cloned().unwrap_or_default().parse::<u128>()?;
        let account_name_identification = field_map.get(":25:").cloned().unwrap_or_default();

        let parsed_opening_balance = field_map.get(":60F:").map(|v| parse_60f(v));
        let parsed_closing_balance = field_map.get(":62F:").map(|v| parse_60f(v));
        // dbg!(&parsed_closing_balance);
        let currency = &parsed_opening_balance.clone().unwrap().currency; // TODO without unwrap and clone

        let re_match_61_86_tr = regex::Regex::new(
            r"(?x)
        \:61\:(?P<data_time>\d{6,10})
        (?P<debit_credit>[cdCD])R?
                (?P<amount>\d+[\,\.]\d\d)
                (?P<transaction_type_code>\w)(?P<bank_transaction_code>.{3})
                        (?P<transaction_id>[\w\/]+)
        [\n\w\s]*\:86\:(?P<description_filed>[.\w\s]*)
        ",
        )
        .unwrap();

        let transactions: Vec<Transaction> = re_match_61_86_tr
            .captures_iter(&body)
            .map(|caps| {
                print!("testestset");
                let date_time = NaiveDateTime::parse_from_str(
                    &caps.name("data_time").unwrap().as_str(),
                    "%y%m%d%H%M",
                )
                .unwrap();
                let credit_debit = match caps.name("debit_credit").unwrap().as_str() {
                    "C" => BalanceAdjustType::Credit,
                    "D" => BalanceAdjustType::Debit,
                    "c" => BalanceAdjustType::Credit,
                    "d" => BalanceAdjustType::Debit,
                    _ => BalanceAdjustType::WithoutInfo,
                };
                let amount = caps
                    .name("amount")
                    .unwrap()
                    .as_str()
                    .replace(',', ".")
                    .parse::<f64>()
                    .unwrap();
                let _transaction_type_code = caps.name("transaction_type_code").unwrap().as_str();
                // dbg!(transaction_type_code);
                let bank_transaction_code = caps.name("bank_transaction_code").unwrap().as_str();
                // dbg!(bank_transaction_code);
                let description_filed = caps.name("description_filed").unwrap().as_str();
                let tr_direction = description_filed.split("\x20").nth(0).unwrap().to_string();
                // dbg!(&description_filed);
                // dbg!(&tr_direction);
                let transaction_id = caps
                    .name("transaction_id")
                    .unwrap()
                    .as_str()
                    .split("//")
                    .next()
                    .unwrap();
                // dbg!(transaction_id);

                let (credit_account, debit_account) = match &credit_debit {
                    BalanceAdjustType::Debit => (tr_direction, account_name_identification.clone()),
                    BalanceAdjustType::Credit => {
                        (account_name_identification.clone(), tr_direction)
                    }
                    BalanceAdjustType::WithoutInfo => todo!(),
                };
                Transaction {
                    id: transaction_id.to_owned(),
                    currency: currency.to_string(),
                    date_time,
                    credit_debit,
                    amount,
                    transaction_type: Some(bank_transaction_code.to_owned()),
                    credit_account,
                    debit_account,
                    service_bank: String::new(),
                    purpose: description_filed.trim().to_owned(), // filled later
                }
            })
            .collect();
        let _account_name = field_map.get("20:").cloned().unwrap_or_default(); // Transaction reference number

        let mut todo_cash_data = Wallet::default();
        // todo_cash_data.bank_maintainer = account_servicer;
        todo_cash_data.bank_maintainer = bank_maintainer.to_string();
        // dbg!(bank_maintainer);
        todo_cash_data.account = account_name_identification;
        // // todo_cash_data.id = account_identification;
        todo_cash_data.statement_id = field_map.get(":28C:").cloned().unwrap_or_default();
        // dbg!(&parsed_opening_balance);
        // if let Some(_opening_balance)  = &parsed_opening_balance {
        todo_cash_data.opening_balance = parsed_opening_balance;
        // }

        // if let Some(_closing_balance)  = &parsed_closing_balance {
        todo_cash_data.closing_balance = parsed_closing_balance;
        // }

        todo_cash_data.transactions = transactions;
        // todo_cash_data.opening_balance = Some(field_map.get(":60F:").map(|v| parse_60f(v)));
        // dbg!(&todo_cash_data);
        statement_data_vec.push(todo_cash_data);
        // dbg!(&messages);
    }

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
    let date_str = rest[..6].to_string();
    let date = parse_yymmdd(&date_str).unwrap();
    let currency = rest[6..9].to_string();
    let amount_str = rest[9..].to_string().replace("\n-", "");
    // dbg!(&amount_str);
    let amount = amount_str.replace(',', ".").parse::<f64>().unwrap_or(0.0);
    // Balance { dc_mark, date, currency, amount }
    // dbg!(amount);

    Balance {
        amount,
        currency,
        date,
        // country: "pakistan".to_owned(),
        credit_debit,
        last_ops: Vec::new(),
    }
}

fn parse_yymmdd(s: &str) -> Result<NaiveDate, chrono::ParseError> {
    NaiveDate::parse_from_str(s, "%y%m%d")
}
