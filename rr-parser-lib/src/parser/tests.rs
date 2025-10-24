// use std::cell::RefCell;
// use std::collections::HashMap;
// use std::fs;
use std::path::{Path, PathBuf};
use std::fmt;

pub struct TestConstants;

impl TestConstants {
    pub const PROJECT_ROOT_DIR: &'static str = "../";
    pub const TEST_SAMPLES_SUBDIR: &'static str = "tests/test_files";
    pub const OUTPUT_DIR: &'static str = "output";
    pub const LOG_FILE: &'static str = "conver.log";
    pub const TEST_SAMPLE_CSV: &'static str = "example_of_report_bill_1.csv";
    pub const OUTPUT_RESULT_XML: &'static str = "result.xml";

    pub fn project_root_dir() -> &'static Path {
        Path::new(Self::PROJECT_ROOT_DIR)
    }

    pub fn log_file() -> &'static Path {
        Path::new(Self::LOG_FILE)
    }

    pub fn get_output_path_xml() -> String {
         Path::new(Self::PROJECT_ROOT_DIR)
            .join(Self::OUTPUT_DIR)
            .join(Self::OUTPUT_RESULT_XML).to_string_lossy().into_owned()


    }

    pub fn get_test_sample_csv() -> String { 
        Path::new(Self::PROJECT_ROOT_DIR)
            .join(Self::TEST_SAMPLES_SUBDIR)
            .join(Self::TEST_SAMPLE_CSV).to_string_lossy().into_owned()
    }

    // pub fn cache_dir() -> &'static Path {
    //     // Path::new(Self::CACHE_DIR)
    //     let base_path = Path::new(Self::PROJECT_ROOT_DIR);
    //     let result = base_path.join("tests/test_files").join("example_of_report_bill_1.csv").as_path();
    //     result
    // }

    // Optional: return as PathBuf if you need owned paths
    pub fn log_file_buf() -> PathBuf {
        PathBuf::from(Self::LOG_FILE)
    }
}

#[cfg(test)]
mod tests {
    use crate::{ parser::{parse_input_and_serialize_via_fn, parse_input_and_serialize_via_trait, SerilyzerMT940, SerilyzerCAMT053, SerilyzerCSV, Parseble, ParserFormat, read_file, render_shape}};

    use super::*;
    use std::io::ErrorKind;

    // static PROJECT_ROOT_DIR = ;
    // static PROJECT_ROOT_DIR = Path::new("../");

    #[test]
    fn test_read_nonexistent_file() {
        let fake_path = Path::new("/definitely/does/not/exist.txt");
        let result = read_file(fake_path);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), ErrorKind::NotFound);
    }

    #[test]
    fn test_convert_csv_to_xml_via_fn() {
        // let fake_path = Path::new("../tests/test_files/example_of_report_bill_1.csv");
        // let test_constants = std::cell::RefCell::new(TestConstants);
        // let test_constants = TestConstants;
        // let fake_path = Path::new("../tests/test_files/example_of_report_bill_1.csv");
        let readed_file = read_file(Path::new(&TestConstants::get_test_sample_csv()));
        assert!(readed_file.is_ok());
        let save_result =     parse_input_and_serialize_via_fn(&TestConstants::get_test_sample_csv(),&ParserFormat::Csv, &ParserFormat::Xml, &TestConstants::get_output_path_xml() );
        assert!(save_result.is_ok());

        // assert_eq!(result.unwrap_err().kind(), ErrorKind::NotFound);
    }

    fn test_convert_csv_to_xml_via_trait() {
        // let fake_path = Path::new("../tests/test_files/example_of_report_bill_1.csv");
        // let test_constants = std::cell::RefCell::new(TestConstants);
        // let test_constants = TestConstants;
        // let fake_path = Path::new("../tests/test_files/example_of_report_bill_1.csv");
        let readed_file = read_file(Path::new(&TestConstants::get_test_sample_csv()));
        assert!(readed_file.is_ok());
        let save_result = parse_input_and_serialize_via_trait(&TestConstants::get_test_sample_csv(),&ParserFormat::Csv, &ParserFormat::Xml, &TestConstants::get_output_path_xml() );
        assert!(save_result.is_ok());

        // assert_eq!(result.unwrap_err().kind(), ErrorKind::NotFound);
    }

    // #[test]
    // fn test_parser_as_traite() {
    //     let input_parse_mt940 = SerilyzerMT940 { radius: 5.0 };
    //     let input_parser_csv = SerilyzerCSV { width: 10.0, height: 4.0 };
    //     let input_parser_camt053 = SerilyzerCAMT053 { base: 6.0, height: 8.0 };
    
    //     // Call draw directly
    //     input_parse_mt940.get_describe();
    //     input_parser_csv.get_describe();
    //     input_parser_camt053.get_describe();
    
    //     // Or use a generic function
    //     render_shape(input_parse_mt940);
    //     render_shape(input_parser_csv);
    //     render_shape(input_parser_camt053);
    
    // }


}
