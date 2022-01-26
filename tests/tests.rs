use sekret::Secret;

#[test]
fn test_display_format() {
    let secret = Secret("sekret");
    assert_eq!(format!("{secret}"), "█████");
}

#[test]
fn test_debug_format() {
    let secret = Secret("sekret");
    assert_eq!(format!("{secret:?}"), r#"Secret("█████")"#);
}

#[test]
fn test_pretty_debug_format() {
    let secret = Secret("sekret");
    assert!(!format!("{secret:#?}").contains("sekret"));
}
