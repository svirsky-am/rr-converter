use crate::parser::Wallet;

pub fn render_content_as_yaml(input_vec: Vec<Wallet>) -> Vec<u8> {
    let mut result_content = String::from("---\n");

    result_content.push_str(&format!("account_info: {:#?}\n", input_vec));
    // let mut result_content: Vec<u8> = format!("result_content: {}\n", self)
    // ;
    result_content.as_bytes().to_vec()
}

pub fn render_content_as_camt053(input_vec: Vec<Wallet>) -> Vec<u8> {
    let mut result_content = String::from("---\n");

    result_content.push_str(&format!("account_info: {:#?}\n", input_vec));
    // let mut result_content: Vec<u8> = format!("result_content: {}\n", self)
    // ;
    result_content.as_bytes().to_vec()
}

pub fn render_content_as_mt940(input_vec: Vec<Wallet>) -> Vec<u8> {
    let mut result_content = String::from("---\n");

    result_content.push_str(&format!("account_info: {:#?}\n", input_vec));
    // let mut result_content: Vec<u8> = format!("result_content: {}\n", self)
    // ;
    result_content.as_bytes().to_vec()
}


pub fn render_content_as_csv(input_vec: Vec<Wallet>) -> Vec<u8> {
    let mut result_content = String::from("---\n");

    result_content.push_str(&format!("account_info: {:#?}\n", input_vec));
    // let mut result_content: Vec<u8> = format!("result_content: {}\n", self)
    // ;
    result_content.as_bytes().to_vec()
}

pub fn render_content_as_csv_extra_fin(input_vec: Vec<Wallet>) -> Vec<u8> {
    let mut result_content = String::from("---\n");

    result_content.push_str(&format!("account_info: {:#?}\n", input_vec));
    // let mut result_content: Vec<u8> = format!("result_content: {}\n", self)
    // ;
    result_content.as_bytes().to_vec()
}