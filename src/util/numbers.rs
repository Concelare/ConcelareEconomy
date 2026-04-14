
pub fn format_money(value: f32) -> String {
    let negative = value.is_sign_negative();
    let abs = value.abs();

    let integer_part = abs.trunc() as i64;
    let decimal_part = ((abs.fract() * 100.0).round() as i64).min(99);

    let mut int_str = integer_part.to_string();
    let mut formatted_int = String::new();

    while int_str.len() > 3 {
        let split = int_str.split_off(int_str.len() - 3);
        if formatted_int.is_empty() {
            formatted_int = split;
        } else {
            formatted_int = format!("{},{}", split, formatted_int);
        }
    }

    if formatted_int.is_empty() {
        formatted_int = int_str;
    } else {
        formatted_int = format!("{},{}", int_str, formatted_int);
    }

    let result = format!("{}.{}", formatted_int, format!("{:02}", decimal_part));

    if negative {
        format!("-{}", result)
    } else {
        result
    }
}