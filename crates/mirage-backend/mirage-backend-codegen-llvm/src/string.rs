

pub fn to_string_with_special_char(s: &str) -> String {
    s
        .replace("\\n", "\n")
        .replace("\\t", "\t")
        .replace("\\r", "\r")
        .replace("\\0", "\0")
        .replace("\\'", "'")
        .replace("\\\"", "\"")
        .replace("\\\\", "\\")
}