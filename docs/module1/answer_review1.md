
```sh

use swift_mt_message::MT940;
use swift_mt_message::SwiftParser;


match SwiftParser::parse::<MT940>(content) {
    Ok(parsed) => {
        let json = serde_json::to_string_pretty(&parsed).unwrap();
        println!("Parsed Message:\n{}", json);
    }
    Err(e) => {
        // Rich error reporting
        eprintln!("Parse error: {}", e.brief_message());
        eprintln!("\nDetails:\n{}", e.debug_report());
        eprintln!("\n{}", e.format_with_context(content));
    }
}
```

Для файла  `tests/test_files/MT940_github_1.mt940` получилась следующая ошибка:
```
Parse error: Block 2 structure invalid

Details:
Block Structure Error:
├─ Block: 2
├─ Error: Output Block 2 too short: expected at least 46 characters, got 17
└─ Hint: Ensure block 2 follows SWIFT message structure

Block Structure Error:
├─ Block: 2
├─ Error: Output Block 2 too short: expected at least 46 characters, got 17
└─ Hint: Ensure block 2 follows SWIFT message structure
```

Для файла  `tests/test_files/MT_940_aiophotoz.mt940` получилась следующая ошибка:
```
Block Structure Error:
├─ Block: 1
├─ Error: Block 1 must be exactly 25 characters, got 0
└─ Hint: Ensure block 1 follows SWIFT message structure
```