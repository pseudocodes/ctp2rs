/// XML 错误 ID → Rust 枚举变体名（PascalCase）
///
/// 例如: "NONE" → "None", "NO_TRADING_RIGHT" → "NoTradingRight"
pub fn to_rust_enum_name(id: &str) -> String {
    if id == "NONE" {
        return "None".to_string();
    }
    let mut result = String::new();
    let mut capitalize_next = true;
    for c in id.chars() {
        if c == '_' {
            capitalize_next = true;
        } else if capitalize_next {
            result.push(c.to_ascii_uppercase());
            capitalize_next = false;
        } else {
            result.push(c.to_ascii_lowercase());
        }
    }
    result
}
