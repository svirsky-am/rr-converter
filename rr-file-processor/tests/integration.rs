use std::env;
use std::fs::File;
use std::io::{self, BufReader, BufWriter};
use std::path::Path;

use rr_parser_lib::{parse_input_and_serialize_via_trait, InputParserFormat, OutputParserFormat}; // Import io for Result

// fn test_real_file_reading() -> Result<(), Box<dyn std::error::Error>> {
//         let test_file = Path::new("tests/test_files/hello.txt");

//         assert_eq!(content.trim(), "Hello from the binary crate!");
//         Ok(())
//     }

fn print_cur_dir() -> io::Result<()> {
    let current_dir = env::current_dir()?; // Get the current directory as PathBuf
    println!("The current directory is: {}", current_dir.display()); // Print using .display()
    Ok(()) // Return Ok(()) for successful execution
}

#[test]
fn test_real_file_reading() {
    let _ = print_cur_dir();
    // let test_file = Path::new("tests/rust.txt");

    // let content = rr_parser_lib::read_file(test_file).expect("Failed to read test file");

    let input_file = File::open(Path::new("tests/test_files/data.csv")).unwrap();
        
    let reader_from_file = BufReader::new(input_file);

    // Create a new file (this will overwrite if it already exists)
    // let output_file_path = Path::new("output/csv_to_csv.txt");
    let outputfile = File::create(Path::new("output/rust_1.txt")).unwrap();

    let mut output_writer_file = BufWriter::new(outputfile);
    let _result_1 = parse_input_and_serialize_via_trait(
        reader_from_file,
        output_writer_file,
        InputParserFormat::CsvExtraFin,
        OutputParserFormat::Csv,
    );
    assert!(_result_1.is_ok());

    

    // assert_eq!(
    //     content.trim(),
    //     "Contant of integration test of 'rr-file-processor'!"
    // );
    // Ok(())
}
