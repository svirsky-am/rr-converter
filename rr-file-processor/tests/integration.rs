use std::env;
use std::io;
use std::path::Path; // Import io for Result

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
    let test_file = Path::new("tests/rust.txt");

    let content = rr_parser_lib::read_file(test_file).expect("Failed to read test file");

    assert_eq!(
        content.trim(),
        "Contant of integration test of 'rr-file-processor'!"
    );
    // Ok(())
}
