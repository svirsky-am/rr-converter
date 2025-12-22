// ====== Helpers ======
pub fn get_text(node: roxmltree::Node, tag: (&str, &str)) -> String {
    node.children()
        .find(|n| n.has_tag_name(tag))
        .and_then(|n| n.text())
        .unwrap_or("")
        .trim()
        .to_string()
}

pub fn find_nested_text(parent: roxmltree::Node, path: &[(&str, &str)]) -> String {
    let mut current = parent;
    for &tag in path {
        current = match current.children().find(|n| n.has_tag_name(tag)) {
            Some(n) => n,
            None => return String::new(),
        };
    }
    current.text().unwrap_or("").trim().to_string()
}

