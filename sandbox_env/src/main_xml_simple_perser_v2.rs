use std::collections::HashMap;
use std::fs;
use roxmltree::{Document, Node};
use serde::Serialize;

const NS: &str = "urn:iso:std:iso:20022:tech:xsd:camt.053.001.02";

#[derive(Serialize, Debug)]
struct Account {
    iban: String,
    currency: String,
    name: String,
    owner_name: String,
    bic: String,
}

#[derive(Serialize, Debug)]
struct Balance {
    code: String,           // e.g., OPBD, CLBD
    amount: f64,
    currency: String,
    credit_debit: String,   // CRDT or DBIT
    date: String,
}

#[derive(Serialize, Debug)]
struct Transaction {
    entry_ref: String,
    amount: f64,
    currency: String,
    credit_debit: String,
    booking_date: String,
    value_date: String,
    transaction_id: Option<String>,
    end_to_end_id: Option<String>,
    remittance: Vec<String>,
    debtor_name: Option<String>,
    creditor_name: Option<String>,
    debtor_iban: Option<String>,
    creditor_iban: Option<String>,
}

#[derive(Serialize, Debug)]
struct CamtData {
    message_id: String,
    creation_time: String,
    account: Account,
    balances: Vec<Balance>,
    transactions: Vec<Transaction>,
}

fn get_text_or_blank(node: Node, tag: &str) -> String {
    node.children()
        .find(|n| n.has_tag_name((NS, tag)))
        .and_then(|n| n.text())
        .unwrap_or("")
        .trim()
        .to_string()
}

fn get_attribute(node: Node, attr: &str) -> Option<String> {
    node.attribute(attr).map(|s| s.to_string())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let xml_content = fs::read_to_string("tests/test_files/camt053_dk_example.xml")?;
    let doc = Document::parse(&xml_content)?;
    let root = doc.root_element();

    let bk_to_cstmr_stmt = root
        .children()
        .find(|n| n.has_tag_name((NS, "BkToCstmrStmt")))
        .ok_or("Missing BkToCstmrStmt")?;

    let grp_hdr = bk_to_cstmr_stmt
        .children()
        .find(|n| n.has_tag_name((NS, "GrpHdr")))
        .ok_or("Missing GrpHdr")?;

    let msg_id = get_text_or_blank(grp_hdr, "MsgId");
    let cre_dt_tm = get_text_or_blank(grp_hdr, "CreDtTm");

    let stmt = bk_to_cstmr_stmt
        .children()
        .find(|n| n.has_tag_name((NS, "Stmt")))
        .ok_or("Missing Stmt")?;

    // === Account ===
    let acct = stmt
        .children()
        .find(|n| n.has_tag_name((NS, "Acct")))
        .ok_or("Missing Acct")?;

    let iban = get_text_or_blank(
        acct,
        "Id/IBAN",
    ); // roxmltree doesn't support XPath, so we traverse manually

    // Helper to find nested element by path (simple)
    let find_nested_text = |parent: Node, path: &[&str]| -> String {
        let mut current = parent;
        for &tag in path {
            current = match current.children().find(|n| n.has_tag_name((NS, tag))) {
                Some(n) => n,
                None => return String::new(),
            };
        }
        current.text().unwrap_or("").trim().to_string()
    };

    let iban = find_nested_text(acct, &["Id", "IBAN"]);
    let currency = get_text_or_blank(acct, "Ccy");
    let account_name = get_text_or_blank(acct, "Nm");
    let owner_name = find_nested_text(acct, &["Ownr", "Nm"]);
    let bic = find_nested_text(acct, &["Svcr", "FinInstnId", "BIC"]);

    let account = Account {
        iban,
        currency,
        name: account_name,
        owner_name,
        bic,
    };

    // === Balances ===
    let mut balances = Vec::new();
    for bal in stmt.children().filter(|n| n.has_tag_name((NS, "Bal"))) {
        let code = find_nested_text(bal, &["Tp", "CdOrPrtry", "Cd"]);
        let amt_node = bal.children().find(|n| n.has_tag_name((NS, "Amt"))).unwrap();
        let amount: f64 = amt_node.text().unwrap_or("0").parse().unwrap_or(0.0);
        let amt_ccy = amt_node.attribute("Ccy").unwrap_or("").to_string();
        let credit_debit = get_text_or_blank(bal, "CdtDbtInd");
        let date = find_nested_text(bal, &["Dt", "Dt"]);

        balances.push(Balance {
            code,
            amount,
            currency: amt_ccy,
            credit_debit,
            date,
        });
    }

    // === Transactions ===
    let mut transactions = Vec::new();
    for ntry in stmt.children().filter(|n| n.has_tag_name((NS, "Ntry"))) {
        let entry_ref = get_text_or_blank(ntry, "NtryRef");
        let amt_node = ntry.children().find(|n| n.has_tag_name((NS, "Amt"))).unwrap();
        let amount: f64 = amt_node.text().unwrap_or("0").parse().unwrap_or(0.0);
        let currency = amt_node.attribute("Ccy").unwrap_or("").to_string();
        let credit_debit = get_text_or_blank(ntry, "CdtDbtInd");
        let booking_date = find_nested_text(ntry, &["BookgDt", "Dt"]);
        let value_date = find_nested_text(ntry, &["ValDt", "Dt"]);

        // Drill into NtryDtls -> TxDtls (may be multiple)
        let ntry_dtls = ntry.children().find(|n| n.has_tag_name((NS, "NtryDtls")));
        let tx_dtls_list: Vec<Node> = if let Some(dtls) = ntry_dtls {
            dtls.children()
                .filter(|n| n.has_tag_name((NS, "TxDtls")))
                .collect()
        } else {
            vec![]
        };

        // If no TxDtls, create one dummy from parent Ntry
        let tx_dtls_list = if tx_dtls_list.is_empty() {
            vec![ntry]
        } else {
            tx_dtls_list
        };

        for tx_dtl in tx_dtls_list {
            let refs = tx_dtl.children().find(|n| n.has_tag_name((NS, "Refs")));
            let end_to_end_id = if let Some(r) = refs {
                get_text_or_blank(r, "EndToEndId")
            } else {
                String::new()
            };
            let end_to_end_id = if end_to_end_id.is_empty() { None } else { Some(end_to_end_id) };

            let transaction_id = if let Some(r) = refs {
                get_text_or_blank(r, "TxId")
            } else {
                String::new()
            };
            let transaction_id = if transaction_id.is_empty() { None } else { Some(transaction_id) };

            // Remittance info
            let mut remittance = Vec::new();
            if let Some(rmt) = tx_dtl.children().find(|n| n.has_tag_name((NS, "RmtInf"))) {
                for ustrd in rmt.children().filter(|n| n.has_tag_name((NS, "Ustrd"))) {
                    if let Some(text) = ustrd.text() {
                        remittance.push(text.trim().to_string());
                    }
                }
            }

            // Parties
            let rltd_pties = tx_dtl.children().find(|n| n.has_tag_name((NS, "RltdPties")));
            let debtor_name = rltd_pties.as_ref().and_then(|p| {
                p.children().find(|n| n.has_tag_name((NS, "Dbtr"))).map(|d| {
                    get_text_or_blank(d, "Nm")
                })
            }).filter(|s| !s.is_empty());

            let creditor_name = rltd_pties.as_ref().and_then(|p| {
                p.children().find(|n| n.has_tag_name((NS, "Cdtr"))).map(|c| {
                    get_text_or_blank(c, "Nm")
                })
            }).filter(|s| !s.is_empty());

            let debtor_iban = rltd_pties.as_ref().and_then(|p| {
                p.children().find(|n| n.has_tag_name((NS, "DbtrAcct"))).and_then(|a| {
                    a.children().find(|n| n.has_tag_name((NS, "Id"))).map(|id| {
                        find_nested_text(id, &["IBAN"])
                    })
                })
            }).filter(|s| !s.is_empty());

            let creditor_iban = rltd_pties.as_ref().and_then(|p| {
                p.children().find(|n| n.has_tag_name((NS, "CdtrAcct"))).and_then(|a| {
                    a.children().find(|n| n.has_tag_name((NS, "Id"))).map(|id| {
                        find_nested_text(id, &["IBAN"])
                    })
                })
            }).filter(|s| !s.is_empty());

            transactions.push(Transaction {
                entry_ref: entry_ref.clone(),
                amount,
                currency: currency.clone(),
                credit_debit: credit_debit.clone(),
                booking_date: booking_date.clone(),
                value_date: value_date.clone(),
                transaction_id,
                end_to_end_id,
                remittance: remittance.clone(),
                debtor_name,
                creditor_name,
                debtor_iban,
                creditor_iban,
            });
        }
    }

    let data = CamtData {
        message_id: msg_id,
        creation_time: cre_dt_tm,
        account,
        balances,
        transactions,
    };

    let yaml_output = serde_yaml::to_string(&data)?;
    fs::write("output/output_from_camt53.yaml", yaml_output)?;

    println!("✅ Successfully parsed CAMT.053 and saved to output.yaml");
    Ok(())
}