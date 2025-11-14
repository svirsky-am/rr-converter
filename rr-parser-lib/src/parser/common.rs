use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct Balance {
    pub amount: f64,
    pub currency: String,
    pub credit_debit: String, // "CRDT" / "DBIT" or "C"/"D"
    pub date: String,         // YYYY-MM-DD
    pub country: String,
}

#[derive(Hash, PartialEq, Eq, Serialize, Debug, Clone)]
pub struct Transaction {
    pub value_date: String,
    // pub amount: f64,
    pub currency: String,
    pub credit_debit: String,
    pub narrative: Vec<String>,
    pub country: &'static str, 
    pub id: u32
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
