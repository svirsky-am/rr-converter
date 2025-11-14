// use std::cell::RefCell;
// use std::collections::HashMap;
// use std::fs;

// use std::path::Path;

#[cfg(test)]
mod tests {

    use std::fs::File;
    use std::path::{Path, PathBuf};

    use std::io::{BufReader, BufWriter};

    pub struct TestConstants;

    impl TestConstants {
        pub const PROJECT_ROOT_DIR: &'static str = "../";
        pub const TEST_SAMPLES_SUBDIR: &'static str = "tests/test_files";
        pub const OUTPUT_DIR: &'static str = "output";
        pub const LOG_DIR_NAME: &'static str = "logs";
        pub const LOG_FILE: &'static str = "conver.log";
        pub const TEST_SAMPLE_CSV: &'static str = "example_of_report_bill_1.csv";
        pub const TEST_SAMPLE_CSV_NORMALIZED: &'static str =
            "example_of_report_bill_1_normalized_v1.csv";

        pub const OUTPUT_RESULT_XML: &'static str = "result.xml";

        pub fn project_root_dir() -> &'static Path {
            Path::new(Self::PROJECT_ROOT_DIR)
        }

        pub fn log_file() -> &'static Path {
            Path::new(Self::LOG_FILE)
        }

        pub fn get_output_path() -> PathBuf {
            Path::new(Self::PROJECT_ROOT_DIR).join(Self::OUTPUT_DIR)
            // .as_path()
            // .to_string_lossy().into_owned()
        }


        pub fn get_output_dir_path(output_filename: String) -> PathBuf {
            Path::new(Self::PROJECT_ROOT_DIR)
                .join(Self::OUTPUT_DIR)
                .join(Path::new(&output_filename))
                .to_string_lossy()
                .into_owned()
                .into()
        }

        pub fn get_log_dir() -> PathBuf {
            Path::new(Self::PROJECT_ROOT_DIR)
                .join(Self::LOG_DIR_NAME)
                .to_string_lossy()
                .into_owned()
                .into()
        }


        pub fn take_sample_file(sample_filename: String) -> PathBuf {
            Path::new(Self::PROJECT_ROOT_DIR)
                .join(Self::TEST_SAMPLES_SUBDIR)
                .join(Path::new(&sample_filename))
                .to_string_lossy()
                .into_owned()
                .into()
        }

        // pub fn get_test_sample_csv() -> String {
        //     Path::new(Self::PROJECT_ROOT_DIR)
        //         .join(Self::TEST_SAMPLES_SUBDIR)
        //         .join(Self::TEST_SAMPLE_CSV)
        //         .to_string_lossy()
        //         .into_owned()
        // }

        // pub fn get_test_sample_csv_normalized() -> String {
        //     Path::new(Self::PROJECT_ROOT_DIR)
        //         .join(Self::TEST_SAMPLES_SUBDIR)
        //         .join(Self::TEST_SAMPLE_CSV_NORMALIZED)
        //         .to_string_lossy()
        //         .into_owned()
        // }

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

    use crate::parser::{
        InputParserFormat, OutputParserFormat, parse_input_and_serialize_via_trait,
    };

    #[test]
    fn test_convert_csv_to_xml_via_trait() {

        let input_file =
            File::open(Path::new(&TestConstants::take_sample_file(
                // "example_of_report_bill_1_normalized_v1.csv".to_string(),
                                "example_of_report_bill_1.csv".to_string(),
            ))).unwrap();

        let reader_from_file = BufReader::new(input_file);

        let output_dir: &PathBuf = &TestConstants::get_output_path();
        let log_dir = &TestConstants::get_log_dir();
        std::fs::create_dir_all(&output_dir).unwrap();
        let outputfile = File::create(Path::new(&TestConstants::get_output_dir_path(
            "csv_to_csv.txt".to_string(),
        )))
        .unwrap();

        let output_writer_file = BufWriter::new(outputfile);
        let _result_1 = parse_input_and_serialize_via_trait(
            reader_from_file,
            output_writer_file,
            InputParserFormat::CsvExtraFin,
            OutputParserFormat::CsvExtraFin,
        );
        assert!(_result_1.is_ok());
    }

    #[test]
    fn test_parse_mt940() {

        let output_dir: &PathBuf = &TestConstants::get_output_path();
        std::fs::create_dir_all(&output_dir).unwrap();

        let _result_2 = parse_input_and_serialize_via_trait(
            File::open(Path::new(&TestConstants::take_sample_file(
                "MT_940_oracle.mt940".to_string(),
            )))
            .unwrap(),
            File::create(Path::new(&TestConstants::get_output_dir_path(
                "MT_940_oracle.mt940_to_csv.txt".to_string(),
            )))
            .unwrap(),
            InputParserFormat::Mt940,
            OutputParserFormat::CsvExtraFin,
        );

        let _result_3 = parse_input_and_serialize_via_trait(
            File::open(Path::new(&TestConstants::take_sample_file(
                "MT_940_aiophotoz.mt940".to_string(),
            )))
            .unwrap(),
            File::create(Path::new(&TestConstants::get_output_dir_path(
                "MT_940_aiophotoz.mt940.txt".to_string(),
            )))
            .unwrap(),
            InputParserFormat::Mt940,
            OutputParserFormat::Camt053,
        );

        let _result_4 = parse_input_and_serialize_via_trait(
            File::open(Path::new(&TestConstants::take_sample_file(
                "MT940_github_1.mt940".to_string(),
            )))
            .unwrap(),
            File::create(Path::new(&TestConstants::get_output_dir_path(
                "MT940_github_1.mt940.txt".to_string(),
            )))
            .unwrap(),
            InputParserFormat::Mt940,
            OutputParserFormat::Mt940,
        );


    }
    #[test]
    fn test_parse_camt053()
    {       
        let output_dir: &PathBuf = &TestConstants::get_output_path();
        std::fs::create_dir_all(&output_dir).unwrap();
        // let _result_5 = parse_input_and_serialize_via_trait(
        //     File::open(Path::new(&TestConstants::take_sample_file(
        //         "Camt.053_example_file_FI-CPE.xml".to_string(),
        //     )))
        //     .unwrap(),
        //     File::create(Path::new(&TestConstants::get_output_dir_path(
        //         "Camt.053_example_file_FI-CPE_camt_to_mt.tx".to_string(),
        //     )))
        //     .unwrap(),
        //     InputParserFormat::Camt053,
        //     OutputParserFormat::Mt940,
        // );
        let _result_6 = parse_input_and_serialize_via_trait(
        File::open(Path::new(&TestConstants::take_sample_file(
            "camt053_dk_example.xml".to_string(),
        )))
        .unwrap(),
        File::create(Path::new(&TestConstants::get_output_dir_path(
            "camt053_dk_example_camt_to_yaml.txt".to_string(),
        )))
        .unwrap(),
        InputParserFormat::Camt053,
        OutputParserFormat::Yaml,
    );

    let _result_7 = parse_input_and_serialize_via_trait(
        File::open(Path::new(&TestConstants::take_sample_file(
            "camt_053_danske_bank.xml".to_string(),
        )))
        .unwrap(),
        File::create(Path::new(&TestConstants::get_output_dir_path(
            "camt_053_danske_bank_yaml.txt".to_string(),
        )))
        .unwrap(),
        InputParserFormat::Camt053,
        OutputParserFormat::Yaml,
    );

    let _result_8 = parse_input_and_serialize_via_trait(
        File::open(Path::new(&TestConstants::take_sample_file(
            "camt_053_treasurease.xml".to_string(),
        )))
        .unwrap(),
        File::create(Path::new(&TestConstants::get_output_dir_path(
            "camt_053_treasurease.xml_yaml.txt".to_string(),
        )))
        .unwrap(),
        InputParserFormat::Camt053,
        OutputParserFormat::Yaml,
    );

    // let mut reader_from_file = BufReader::new(input_file);
    // let output_writer_stdout = BufWriter::new(stdout());
    // let reader_from_sdtdio: BufReader<std::io::Stdin> = BufReader::new(stdin());
}
    

}
