use std::fmt;

use serde::Serialize;

use chrono::{NaiveDate, NaiveDateTime};

#[derive(Debug, Clone, PartialEq, Serialize)]
pub(crate) enum OpKind {
    // пополнить/потратить счёт
    // Deposit(u64),
    // Withdraw(u64),
    // закрыть аккаунт - все средства выведены
    // CloseAccount,
} // вот и всё, никаких посторонних операций и данных! 

#[derive(Debug, Clone, PartialEq, Serialize)]
pub(crate) enum BalanceAdjustType {
    Debit,
    Credit,
    WithoutInfo,
}

impl Default for BalanceAdjustType {
    fn default() -> Self {
        Self::WithoutInfo
    }
}

impl fmt::Display for BalanceAdjustType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            BalanceAdjustType::Credit => "Credit",
            BalanceAdjustType::Debit => "Debit",
            BalanceAdjustType::WithoutInfo => "WithoutInfo",
        };
        write!(f, "{}", name)
    }
}

#[derive(PartialEq, Serialize, Debug, Clone)]
pub struct Balance {
    pub amount: f64,
    pub currency: String,
    pub credit_debit: BalanceAdjustType, // "CRDT" / "DBIT" or "C"/"D"
    pub date: NaiveDate,                 // YYYY-MM-DD
    // pub country: String,
    pub last_ops: Vec<OpKind>,
}

// #[derive(Debug)]
// pub struct Balance {
//     pub result: u64,
//     last_ops: Vec<OpKind>,
// }

#[derive(PartialEq, Serialize, Debug, Clone, Default)]
pub struct Transaction {
    pub date_time: NaiveDateTime,
    pub debit_account: String,
    pub credit_account: String,
    pub amount: f64, // sum
    pub currency: String,
    pub credit_debit: BalanceAdjustType,
    pub transaction_type: Option<String>,
    // pub narrative: Vec<String>,
    pub service_bank: String,
    pub purpose: String,
    // pub country: &'static str,
    pub id: u128,
}

pub fn gen_time_prefix_to_filename() -> String {
    let now_local = chrono::Local::now();
    let custom_format: chrono::format::DelayedFormat<chrono::format::StrftimeItems<'_>> =
        now_local.format("%m.%d.%Y_%I-%M-%S_%.3f");
    custom_format.to_string()
}

pub fn parse_russian_date(input: &str) -> Result<NaiveDate, Box<dyn std::error::Error>> {
    let months = [
        ("января", 1),
        ("февраля", 2),
        ("марта", 3),
        ("апреля", 4),
        ("мая", 5),
        ("июня", 6),
        ("июля", 7),
        ("августа", 8),
        ("сентября", 9),
        ("октября", 10),
        ("ноября", 11),
        ("декабря", 12),
    ];

    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.len() != 3 {
        return Err("Expected format: 'DD MMMM YYYY'".into());
    }

    let day = parts[0].parse::<u32>()?;
    let year = parts[2].parse::<i32>()?;

    let month = months
        .iter()
        .find(|&&(name, _)| name == parts[1])
        .ok_or("Unknown Russian month")?
        .1;

    let date = NaiveDate::from_ymd_opt(year, month, day).ok_or("Invalid calendar date")?;

    // Ok(date.format("%Y-%m-%d").to_string())
    Ok(date)
}
