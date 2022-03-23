use colored::Colorize;

pub fn info(msg: &str) {
    println!("{} {}", "==>".green(), msg);
}

pub fn warn(msg: &str) {
    println!("{} {}", "==>".red(), msg);
}

pub fn err(msg: &str) {
    panic!("{} {}", "==>".red(), msg);
}

pub fn extract_ticket_number(line: &str) -> Option<&str> {
    if !line.starts_with("KDB-") {
        return None;
    }

    let start_bytes = line.find("KDB-").unwrap_or(0);
    let end_del = line.find(" ").unwrap_or(line.len());
    let end_bytes = line.find("/").unwrap_or(end_del);

    let end = if end_bytes < end_del {
        end_bytes
    } else {
        end_del
    };

    Some(&line[start_bytes..end])
}

#[test]
fn test_extract_ticket_number() {
    assert_eq!(extract_ticket_number(""), None);
    assert_eq!(extract_ticket_number("KDB-123"), Some("KDB-123"));
    assert_eq!(extract_ticket_number("KDB-123 Hello"), Some("KDB-123"));
    assert_eq!(extract_ticket_number("KDB-123/name Hello"), Some("KDB-123"));
    assert_eq!(extract_ticket_number("KDB-123/name"), Some("KDB-123"));
}
