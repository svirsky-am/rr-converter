use std::fmt;

use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub(crate) enum OpKind {
    // пополнить/потратить счёт
    Deposit(u64),
    Withdraw(u64),
    // закрыть аккаунт - все средства выведены
    CloseAccount,
} // вот и всё, никаких посторонних операций и данных! 

#[derive(Debug, Clone, PartialEq, Serialize)]
pub(crate) enum BalanceAdjustType {
    Debit,
    Credit,
    WithoutInfo
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
    pub date: String,         // YYYY-MM-DD
    pub country: String,
    pub last_ops: Vec<OpKind>,
}

// #[derive(Debug)]
// pub struct Balance {
//     pub result: u64,
//     last_ops: Vec<OpKind>,
// }



#[derive(PartialEq, Serialize, Debug, Clone, Default)]
pub struct Transaction {
    pub date: String,
    pub debit_account: String,
    pub credit_account: String,
    pub amount: f64, // sum
    pub currency: String,
    pub credit_debit: BalanceAdjustType,
    pub transaction_type: Option<String>,
    // pub narrative: Vec<String>,
    pub target_bank: String,
    pub purpose: String, 
    // pub country: &'static str, 
    pub id: u128
}

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

pub fn gen_time_prefix_to_filename() -> String
{
    let now_local = chrono::Local::now();
    let custom_format: chrono::format::DelayedFormat<chrono::format::StrftimeItems<'_>> = now_local.format("%m.%d.%Y_%I-%M-%S_%.3f");
    custom_format.to_string()
}