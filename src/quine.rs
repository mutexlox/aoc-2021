static S: &str = r#"static S: &str = r#"S_HERE"POUND_HERE;
fn main() {
    println!(
        "{}",
        S.replacen("POUND_HERE", "\x23", 1).replacen("S_HERE", S, 1)
    );
}"#;
fn main() {
    println!(
        "{}",
        S.replacen("POUND_HERE", "\x23", 1).replacen("S_HERE", S, 1)
    );
}
