// use quick_xml::events::Event;
// use quick_xml::Reader;
use quick_xml::events::Event;
use quick_xml::reader::Reader;

fn main() {
    let xml_string = r#"
        <root>
            <item id="1">
                <name>Apple</name>
                <price>1.00</price>
            </item>
            <item id="2">
                <name>Banana</name>
                <price>0.50</price>
            </item>
        </root>
    "#;

    let mut reader = Reader::from_str(xml_string);
    reader.trim_text(true); // Optional: Trim whitespace from text events

    let mut buf = Vec::new();
    let mut count = 0;

    loop {
        match reader.read_event_into(&mut buf) {
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            Ok(Event::Eof) => break,
            Ok(Event::Start(e)) => {
                let tag_name = e.name().as_ref();
                println!("Start Tag: {}", String::from_utf8_lossy(tag_name));
                for attr in e.attributes() {
                    let attr = attr.unwrap();
                    println!(
                        "  Attribute: {}='{}'",
                        String::from_utf8_lossy(attr.key.as_ref()),
                        String::from_utf8_lossy(&attr.value)
                    );
                }
            }
            Ok(Event::Text(e)) => {
                let text = e.unescape().unwrap();
                if !text.trim().is_empty() { // Only print non-empty text content
                    println!("Text: {}", text);
                }
            }
            Ok(Event::End(e)) => {
                let tag_name = e.name().as_ref();
                println!("End Tag: {}", String::from_utf8_lossy(tag_name));
            }
            _ => (), // Ignore other events like comments, processing instructions, etc.
        }
        buf.clear(); // Clear the buffer for the next event
    }
}