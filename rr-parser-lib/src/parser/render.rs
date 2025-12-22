use chrono::{Datelike, NaiveDate};

use quick_xml::events::{BytesDecl, BytesEnd, BytesStart, BytesText, Event};
use quick_xml::Writer;
use std::cell::RefCell;
use std::io::Cursor;
use std::rc::{Rc};

use crate::parser::{Wallet, common::BalanceAdjustType};

pub fn render_content_as_yaml(input_vec: Vec<Wallet>) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // let mut result_content = String::from("---\n");
    let iner_result_content = serde_yaml::to_string(&input_vec).unwrap();
    Ok(iner_result_content.as_bytes().to_vec())
}


type SharedDepth = Rc<RefCell<usize>>;

pub struct RrXmlTag {
    node_name: String,
    depth_at_open: usize,
    writer: *mut Writer<Cursor<Vec<u8>>>,
    depth_ref: SharedDepth,
}

impl RrXmlTag {
 pub fn open(
        node_name: String,
        writer: &mut Writer<Cursor<Vec<u8>>>,
        depth_ref: SharedDepth,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let current_depth = *depth_ref.borrow();
        write_indent(writer, current_depth)?;
        writer.write_event(Event::Start(BytesStart::new(&node_name)))?;
        writer.write_event(Event::Text(BytesText::from_escaped("\n")))?;
        *depth_ref.borrow_mut() += 1;

        Ok(RrXmlTag {
            node_name,
            depth_at_open: current_depth,
            writer: writer as *mut _,
            depth_ref,
        })
    }

    pub fn close(mut self) -> Result<(), Box<dyn std::error::Error>> {
        let writer = unsafe { &mut *self.writer };
        let depth_ref = self.depth_ref;

        *depth_ref.borrow_mut() = self.depth_at_open;

        write_indent(writer, self.depth_at_open)?;
        writer.write_event(Event::End(BytesEnd::new(&self.node_name)))?;
        writer.write_event(Event::Text(BytesText::from_escaped("\n")))?;
        // std::mem::forget(self);
        Ok(())
    }
}

fn write_indent(writer: &mut Writer<Cursor<Vec<u8>>>, level: usize) -> Result<(), Box<dyn std::error::Error>> {
    let indent = "  ".repeat(level);
    writer.write_event(Event::Text(BytesText::from_escaped(&indent)))?;
    Ok(())
}

fn add_child_event_with_text(
    writer: &mut Writer<Cursor<Vec<u8>>>,
    depth: usize,
    node_name: &str,
    node_text: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    write_indent(writer, depth)?;
    writer.write_event(Event::Start(BytesStart::new(node_name)))?;
    writer.write_event(Event::Text(BytesText::from_escaped(node_text)))?;
    writer.write_event(Event::End(BytesEnd::new(node_name)))?;
    writer.write_event(Event::Text(BytesText::from_escaped("\n")))?;
    Ok(())
}





pub fn render_content_as_camt053(input_vec: Vec<Wallet>) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut writer = Writer::new(Cursor::new(Vec::new()));
    let depth_ref = Rc::new(RefCell::new(0));

    // XML declaration
    let decl = BytesDecl::new("1.0", Some("UTF-8"), None);
    writer.write_event(Event::Decl(decl))?;
    writer.write_event(Event::Text(BytesText::from_escaped("\n")))?;

    // <Document>
    write_indent(&mut writer, *depth_ref.borrow())?;
    *depth_ref.borrow_mut() += 1;

    let mut document = BytesStart::new("Document");
    document.push_attribute(("xmlns", "urn:iso:std:iso:20022:tech:xsd:camt.053.001.02"));
    document.push_attribute(("xmlns:xsi", "http://www.w3.org/2001/XMLSchema-instance"));
    document.push_attribute((
        "xsi:schemaLocation",
        "urn:iso:std:iso:20022:tech:xsd:camt.053.001.02 camt.053.001.02.xsd"
    ));
    writer.write_event(Event::Start(document))?;
    writer.write_event(Event::Text(BytesText::from_escaped("\n")))?;

    for cash_statement_data in &input_vec {
        let statement_id = &cash_statement_data.statement_id;
        let creation_time = cash_statement_data
            .creation_time
            .unwrap()
            .format("%Y-%m-%dT%H:%M:%S")
            .to_string();

        let bk_to_cstmr_stmt_tag = RrXmlTag::open("BkToCstmrStmt".to_string(), &mut writer, depth_ref.clone())?;

        {
            let grp_hdr_tag = RrXmlTag::open("GrpHdr".to_string(), &mut writer, depth_ref.clone())?;
            add_child_event_with_text(&mut writer, *depth_ref.borrow(), "MsgId", statement_id)?;
            add_child_event_with_text(&mut writer, *depth_ref.borrow(), "CreDtTm", &creation_time)?;
            grp_hdr_tag.close()?;
        }

        {
            let stmt_tag = RrXmlTag::open("Stmt".to_string(), &mut writer, depth_ref.clone())?;
            add_child_event_with_text(&mut writer, *depth_ref.borrow(), "Id", statement_id)?;
            add_child_event_with_text(&mut writer, *depth_ref.borrow(), "CreDtTm", &creation_time)?;

            let fr_to_dt_tag = RrXmlTag::open("FrToDt".to_string(), &mut writer, depth_ref.clone())?;
            add_child_event_with_text(
                &mut writer,
                *depth_ref.borrow(),
                "FrDtTm",
                &cash_statement_data.statement_period_start.format("%Y-%m-%dT%H:%M:%S").to_string(),
            )?;
            add_child_event_with_text(
                &mut writer,
                *depth_ref.borrow(),
                "ToDtTm",
                &cash_statement_data.statement_period_end.format("%Y-%m-%dT%H:%M:%S").to_string(),
            )?;
            fr_to_dt_tag.close()?;

            let acct_tag = RrXmlTag::open("Acct".to_string(), &mut writer, depth_ref.clone())?;
            acct_tag.close()?;
            stmt_tag.close()?;
        }

        bk_to_cstmr_stmt_tag.close()?;
    }

    // Close Document
    *depth_ref.borrow_mut() = 0;
    write_indent(&mut writer, 0)?;
    writer.write_event(Event::End(BytesEnd::new("Document")))?;
    writer.write_event(Event::Text(BytesText::from_escaped("\n")))?;

    let xml_bytes = writer.into_inner().into_inner();
    Ok(xml_bytes)


//     for cash_statement_data in &input_vec {
//         let date_of_statemant = cash_statement_data
//             .creation_time
//             .clone()
//             .unwrap();
//         let account = &cash_statement_data.account;
//         let currency = &cash_statement_data.currency;
//         let statement_id = &cash_statement_data.statement_id;
//         iner_result_content.push_str(&format!(",,,,,,,,,,,,,,,,,,,,,,
// ,{date_of_statemant},,,,СберБизнес. {statement_id},,,,,,,,,,,,,,,,,
// ,\"todo АВТОР ВЫПИСКИ\",,,,,,,,,,,,,,,,,,,,,
// ,Дата формирования выписки 14.10.2025 в 21:13:22,,,,,,,,,,,,,,,,,,,,,
// ,ВЫПИСКА ОПЕРАЦИЙ ПО ЛИЦЕВОМУ СЧЕТУ,,,,,,,,,,,{account},,,,,,,,,,
// ,,,,,,,,,,,,{account},,,,,,,,,,
// ,,за период с 01 января 2024 г.,,,,,,,,,,,, по ,31 декабря 2024 г.,,,,,,,
// ,,{currency},,,,,,,,,,Дата предыдущей операции по счету 11 декабря 2023 г. ,,,,,,,,,,
// ,,,,,,,,,,,,,,,,,,,,,,
// ,Дата проводки,,,Счет,,,,,Сумма по дебету,,,,Сумма по кредиту,№ документа,,ВО,Банк (БИК и наименование),,,Назначение платежа,,
// ,,,,Дебет,,,,Кредит,,,,,,,,,,,,,,\n"
// ));
//         iner_result_content.push_str("\n>>> START TRANSACTIONS <<<\n");
//         for tr in &cash_statement_data.transactions {
//             match tr.credit_debit {
//                 BalanceAdjustType::Debit => {
//                     iner_result_content.push_str(&format!(
//                         "\n,{},,,\"{}\",,,,\"{}\",{},,,,,1,,01,\"{}\",,,{},",
//                         tr.date_time,
//                         tr.debit_account,
//                         tr.credit_account,
//                         tr.amount,
//                         tr.target_bank,
//                         tr.purpose // avoid newlines in CSV
//                     ));
//                 }

//                 BalanceAdjustType::Credit => {
//                     iner_result_content.push_str(&format!(
//                         "\n,{},,,\"{}\",,,,\"{}\",{},,,,,1,,01,\"{}\",,,{},",
//                         tr.date_time,
//                         tr.debit_account,
//                         tr.credit_account,
//                         tr.amount,
//                         tr.target_bank,
//                         tr.purpose // avoid newlines in CSV
//                     ));
//                 }
//                 BalanceAdjustType::WithoutInfo => iner_result_content.push_str("none"),
//             };
//         }
//         iner_result_content.push_str(">>> END TRANSACTIONS <<<\n");
//     }

//     // let mut result_content: Vec<u8> = format!("result_content: {}\n", self)
//     // ;
    
}

pub fn render_content_as_mt940(input_vec: Vec<Wallet>) -> Result<Vec<u8>, Box<dyn std::error::Error>>{
    let mut iner_result_content = String::new();
    for cash_statement_data in &input_vec {
        let date_of_statemant = cash_statement_data
            .creation_time
            .clone()
            .unwrap();
        let account_id = &cash_statement_data.id;
        let account_name = &cash_statement_data.account;
        let currency = &cash_statement_data.currency;
        let statement_id = &cash_statement_data.statement_id;
        iner_result_content.push_str(&format!(",,,,,,,,,,,,,,,,,,,,,,
,{date_of_statemant},,,,СберБизнес. {statement_id},,,,,,,,,,,,,,,,,
,\"todo АВТОР ВЫПИСКИ\",,,,,,,,,,,,,,,,,,,,,
,Дата формирования выписки {date_of_statemant},,,,,,,,,,,,,,,,,,,,,
,ВЫПИСКА ОПЕРАЦИЙ ПО ЛИЦЕВОМУ СЧЕТУ,,,,,,,,,,,{account_id},,,,,,,,,,
,,,,,,,,,,,,{account_name},,,,,,,,,,
,,за период с 01 января 2024 г.,,,,,,,,,,,, по ,31 декабря 2024 г.,,,,,,,
,,{currency},,,,,,,,,,Дата предыдущей операции по счету 11 декабря 2023 г. ,,,,,,,,,,
,,,,,,,,,,,,,,,,,,,,,,
,Дата проводки,,,Счет,,,,,Сумма по дебету,,,,Сумма по кредиту,№ документа,,ВО,Банк (БИК и наименование),,,Назначение платежа,,
,,,,Дебет,,,,Кредит,,,,,,,,,,,,,,\n"
));
        iner_result_content.push_str("\n>>> START TRANSACTIONS <<<\n");
        for tr in &cash_statement_data.transactions {
            // dbg!(&tr.id);
            match tr.credit_debit {
                BalanceAdjustType::Debit => {
                    iner_result_content.push_str(&format!(
                        "\n,{},,,\"{}\",,,,\"{}\",{},,,,,{},,01,\"{}\",,,{},",
                        tr.date_time,
                        tr.debit_account,
                        tr.credit_account,
                        tr.amount,
                        tr.id,
                        tr.target_bank,
                        tr.purpose // avoid newlines in CSV
                    ));
                }

                BalanceAdjustType::Credit => {
                    iner_result_content.push_str(&format!(
                        "\n,{},,,\"{}\",,,,\"{}\",,,,,{},{},,01,\"{}\",,,{},",
                        tr.date_time,
                        tr.debit_account,
                        tr.credit_account,
                        tr.amount,
                        tr.id,
                        tr.target_bank,
                        tr.purpose // avoid newlines in CSV
                    ));
                }
                BalanceAdjustType::WithoutInfo => iner_result_content.push_str("none"),
            };
        }
        iner_result_content.push_str(">>> END TRANSACTIONS <<<\n");
    }

    // let mut result_content: Vec<u8> = format!("result_content: {}\n", self)
    // ;
    Ok(iner_result_content.as_bytes().to_vec())
}

fn format_russian_naive_date(input_date: NaiveDate) -> String {
    static MONTHS: [&str; 12] = [
        "января", "февраля", "марта", "апреля", "мая", "июня",
        "июля", "августа", "сентября", "октября", "ноября", "декабря",
    ];
    let m_index = input_date.month().to_string().parse::<usize>().unwrap() -1;
    let month_name = MONTHS[m_index];
    format!("{:02} {} {}", input_date.day(), month_name, input_date.year())
}

pub fn render_content_as_csv_extra_fin(input_vec: Vec<Wallet>) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut iner_result_content = String::new();
    for cash_statement_data in &input_vec {
        let datetime_of_statemant = cash_statement_data
            .creation_time
            .clone()
            .unwrap();
        let creation_date = &datetime_of_statemant.format("%d.%m.%Y").to_string();
        let creation_datetime = &datetime_of_statemant.format("%d.%m.%Y в %H:%M:%S").to_string();
        let account_id = &cash_statement_data.id;
        let account_name = &cash_statement_data.account;
        let bank_maintainer = &cash_statement_data.bank_maintainer;
        let currency = &cash_statement_data.currency;
        let statement_id = &cash_statement_data.statement_id;
        let statement_period_start = format_russian_naive_date(cash_statement_data.statement_period_start.date());
        let statement_period_end = format_russian_naive_date(cash_statement_data.statement_period_end.date());
        iner_result_content.push_str(&format!(",,,,,,,,,,,,,,,,,,,,,,
,{creation_date},,,,СберБизнес. {statement_id},,,,,,,,,,,,,,,,,
,\"{bank_maintainer}\",,,,,,,,,,,,,,,,,,,,,
,Дата формирования выписки {creation_datetime},,,,,,,,,,,,,,,,,,,,,
,ВЫПИСКА ОПЕРАЦИЙ ПО ЛИЦЕВОМУ СЧЕТУ,,,,,,,,,,,{account_id},,,,,,,,,,
,,,,,,,,,,,,{account_name},,,,,,,,,,
,,за период с {statement_period_start} г.,,,,,,,,,,,, по ,{statement_period_end} г.,,,,,,,
,,{currency},,,,,,,,,,Дата предыдущей операции по счету TODo г. ,,,,,,,,,,
,,,,,,,,,,,,,,,,,,,,,,
,Дата проводки,,,Счет,,,,,Сумма по дебету,,,,Сумма по кредиту,№ документа,,ВО,Банк (БИК и наименование),,,Назначение платежа,,
,,,,Дебет,,,,Кредит,,,,,,,,,,,,,,\n"
));

        iner_result_content.push_str("\n>>> START TRANSACTIONS <<<\n");
        let mut wtr = csv::Writer::from_writer(Vec::new());
        for statement in &input_vec {
            for tr in &statement.transactions {
                let mut record = vec![String::new(); 23];
                record[1] = tr.date_time.format("%d.%m.%Y").to_string();
                record[4] = tr.debit_account.to_string();
                record[8] = tr.credit_account.to_owned();
                record[14] = tr.id.to_string();
                record[16] = "01".to_string(); // ВО 1/17 ?
                record[17] = tr.target_bank.to_owned();
                record[20] = tr.purpose.replace(['\n', '\r'], " "); // sanitize
                match tr.credit_debit {
                    BalanceAdjustType::Debit => {
                        record[9] = format!("{:.2}", tr.amount);
                    }
                    BalanceAdjustType::Credit => {
                        record[13] = format!("{:.2}", tr.amount);
                    }
                    BalanceAdjustType::WithoutInfo => {
                        continue;
                    }
                }
                let _ = wtr.write_record(&record);
            }
        }
        wtr.flush().unwrap();
        let csv_bytes = wtr.into_inner().unwrap();
        let csv_string = String::from_utf8(csv_bytes).unwrap();
        iner_result_content.push_str(&csv_string);
        iner_result_content.push_str(">>> END TRANSACTIONS <<<\n");
    }

    Ok(iner_result_content.as_bytes().to_vec())
}
