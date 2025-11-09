use quick_xml::Reader;
use serde::Deserialize;
use std::collections::HashMap;

// We must ignore the default namespace during deserialization
// quick-xml doesn't handle namespaces natively in serde mode,
// so we just use local names and hope the structure is unambiguous.

#[derive(Debug, Deserialize)]
struct Document {
    #[serde(rename = "BkToCstmrStmt")]
    bk_to_cstmr_stmt: BkToCstmrStmt,
}

#[derive(Debug, Deserialize)]
struct BkToCstmrStmt {
    stmt: Stmt,
}

#[derive(Debug, Deserialize)]
struct Stmt {
    #[serde(rename = "Acct")]
    acct: Account,
    #[serde(rename = "Bal", default)]
    balances: Vec<Balance>,
    #[serde(rename = "Ntry", default)]
    entries: Vec<Entry>,
}

#[derive(Debug, Deserialize)]
struct Account {
    id: AccountId,
    ccy: String,
    #[serde(rename = "Nm", default)]
    name: Option<String>,
}

#[derive(Debug, Deserialize)]
struct AccountId {
    iban: Option<String>,
    // You could also add Othr if needed
}

#[derive(Debug, Deserialize)]
struct Balance {
    #[serde(rename = "Tp")]
    tp: BalanceType,
    #[serde(rename = "Amt")]
    amt: Amount,
    #[serde(rename = "CdtDbtInd")]
    credit_debit_indicator: String,
    #[serde(rename = "Dt")]
    date: BalanceDate,
}

#[derive(Debug, Deserialize)]
struct BalanceType {
    #[serde(rename = "CdOrPrtry")]
    code_or_proprietary: CodeOrProprietary,
}

#[derive(Debug, Deserialize)]
struct CodeOrProprietary {
    cd: String,
}

#[derive(Debug, Deserialize)]
struct Amount {
    #[serde(rename = "$value")]
    value: String,
    #[serde(rename = "@Ccy")]
    currency: String,
}

#[derive(Debug, Deserialize)]
struct BalanceDate {
    #[serde(rename = "Dt")]
    date: String,
}

#[derive(Debug, Deserialize)]
struct Entry {
    #[serde(rename = "NtryRef")]
    entry_ref: String,
    #[serde(rename = "Amt")]
    amount: Amount,
    #[serde(rename = "CdtDbtInd")]
    credit_debit: String,
    #[serde(rename = "BookgDt")]
    booking_date: DateWrapper,
    #[serde(rename = "ValDt", default)]
    value_date: Option<DateWrapper>,
    #[serde(rename = "AcctSvcrRef", default)]
    acct_svcr_ref: Option<String>,
    #[serde(rename = "NtryDtls", default)]
    details: Option<EntryDetails>,
}

#[derive(Debug, Deserialize, Default)]
struct EntryDetails {
    #[serde(rename = "TxDtls", default)]
    tx_details: Vec<TransactionDetail>,
}

#[derive(Debug, Deserialize, Default)]
struct TransactionDetail {
    #[serde(rename = "Refs", default)]
    refs: Option<References>,
    #[serde(rename = "AmtDtls", default)]
    amt_details: Option<AmountDetails>,
    #[serde(rename = "RltdPties", default)]
    related_parties: Option<RelatedParties>,
    #[serde(rename = "RmtInf", default)]
    remittance_info: Option<RemittanceInfo>,
}

#[derive(Debug, Deserialize, Default)]
struct References {
    #[serde(rename = "EndToEndId", default)]
    end_to_end_id: Option<String>,
    #[serde(rename = "TxId", default)]
    tx_id: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
struct AmountDetails {
    #[serde(rename = "TxAmt", default)]
    tx_amount: Option<Amount>,
}

#[derive(Debug, Deserialize, Default)]
struct RelatedParties {
    #[serde(rename = "Dbtr", default)]
    debtor: Option<Party>,
    #[serde(rename = "Cdtr", default)]
    creditor: Option<Party>,
}

#[derive(Debug, Deserialize, Default)]
struct Party {
    nm: Option<String>,
}

#[derive(Debug, Deserialize)]
struct DateWrapper {
    #[serde(rename = "Dt")]
    date: String,
}

#[derive(Debug, Deserialize, Default)]
struct RemittanceInfo {
    #[serde(rename = "Ustrd", default)]
    unstructured: Vec<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let xml = include_str!("../../tests/test_files/camt053_dk_example.xml"); // or read from file/stdin

    // Strip the default namespace for easier parsing (optional but helpful)
    // quick-xml + serde doesn't handle xmlns well; we rely on local names.
    let cleaned_xml = xml.replace("xmlns=\"urn:iso:std:iso:20022:tech:xsd:camt.053.001.02\"", "");
    use quick_xml::de::from_str;
    let doc: Document = from_str(cleaned_xml);

    let acct = &doc.bk_to_cstmr_stmt.stmt.acct;
    println!("Account IBAN: {:?}", acct.id.iban);
    println!("Currency: {}", acct.ccy);
    println!("Account Name: {:?}", acct.name);

    println!("\nBalances:");
    for bal in &doc.bk_to_cstmr_stmt.stmt.balances {
        println!(
            "  Type: {}, Amount: {} {}, Indicator: {}, Date: {}",
            bal.tp.code_or_proprietary.cd,
            bal.amt.value,
            bal.amt.currency,
            bal.credit_debit_indicator,
            bal.date.date
        );
    }

    println!("\n{} Transactions:", doc.bk_to_cstmr_stmt.stmt.entries.len());
    for entry in &doc.bk_to_cstmr_stmt.stmt.entries {
        println!(
            "  Ref: {}, Amount: {} {}, Type: {}, Booked: {}",
            entry.entry_ref,
            entry.amount.value,
            entry.amount.currency,
            entry.credit_debit,
            entry.booking_date.date
        );

        if let Some(details) = &entry.details {
            for tx in &details.tx_details {
                if let Some(refs) = &tx.refs {
                    println!("    TxId: {:?}", refs.tx_id);
                    println!("    EndToEndId: {:?}", refs.end_to_end_id);
                }
                if let Some(amt) = &tx.amt_details.as_ref().and_then(|a| a.tx_amount.as_ref()) {
                    println!("    Tx Amount: {} {}", amt.value, amt.currency);
                }
                if let Some(parties) = &tx.related_parties {
                    if let Some(d) = &parties.debtor {
                        println!("    Debtor: {:?}", d.nm);
                    }
                    if let Some(c) = &parties.creditor {
                        println!("    Creditor: {:?}", c.nm);
                    }
                }
                if let Some(rmt) = &tx.remittance_info {
                    for ustr in &rmt.unstructured {
                        println!("    Remittance: {}", ustr);
                    }
                }
            }
        }
    }

    Ok(())
}