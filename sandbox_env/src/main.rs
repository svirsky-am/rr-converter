use std::fs::File;
use std::io::Read;
use std::path::Path;

// use quick_xml::events::Event;
// use quick_xml::Reader;
use quick_xml::events::Event;
use quick_xml::reader::Reader;

fn main() {
    
    let input_file = File::open(Path::new("sandbox_env/books.xml")).unwrap();
    // let input_file = File::open("books.xml").unwrap();
    let mut reader_from_file = std::io::BufReader::new(input_file);
    

    // let mut buffer = Vec::new();
    // file.read_to_end(&mut buffer)?;

    let mut content = String::new();
    reader_from_file.read_to_string(&mut content).unwrap();

    // Print the content
    // println!("File content:\n{}", content);
    let xml = content;
    // let xml = r#"<tag1 att1 = "test">
    //     <tag2><!--Test comment-->Test</tag2>
    //     <tag2>Test 2</tag2>
    // </tag1>"#;


    let mut reader = Reader::from_str(&xml);
    reader.config_mut().trim_text(true);

    let mut count = 0;
    let mut txt = Vec::new();
    let mut buf = Vec::new();

    // The `Reader` does not implement `Iterator` because it outputs borrowed data (`Cow`s)
    loop {
    // NOTE: this is the generic case when we don't know about the input BufRead.
    // when the input is a &str or a &[u8], we don't actually need to use another
    // buffer, we could directly call `reader.read_event()`
    match reader.read_event_into(&mut buf) {
    Err(e) => panic!("Error at position {}: {:?}", reader.error_position(), e),
    // exits the loop when reaching end of file
    Ok(Event::Eof) => break,

    Ok(Event::Start(e)) => {
    match e.name().as_ref() {
        b"book" => println!("attributes values: {:?}",
                            e.attributes().map(|a| a.unwrap().value)
                            .collect::<Vec<_>>()),
        b"title" => count += 1,
        _ => (),
    }
    }
    Ok(Event::Text(e)) => txt.push(e.decode().unwrap().into_owned()),

    // There are several other `Event`s we do not consider here
    _ => (),
    }
    // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
    buf.clear();
    }
}