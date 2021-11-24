static S: &str = r#"static S: &str = r#"S_HERE"POUND_HERE;
fn main() {
    let p = "\x23";
    println!(
        "{}",
        S.replacen("POUND_HERE", p, 1).replacen("S_HERE", S, 1)
    );
}"#;
fn main() {
    let p = "\x23";
    println!(
        "{}",
        S.replacen("POUND_HERE", p, 1).replacen("S_HERE", S, 1)
    );
}
