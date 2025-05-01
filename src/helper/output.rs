// Helper function to determine if cutting at an index would be clean
pub fn is_clean_break(word: &str, index: usize) -> bool {
    if index == 0 || index >= word.len() {
        return true;
    }

    let chars: Vec<char> = word.chars().collect();

    if index > 0 && index < chars.len() {
        let curr = chars[index - 1];
        let next = chars[index];
        if "aeiouy".contains(curr) && "aeiouy".contains(next) {
            return false;
        }
    }

    if index > 0 && index < chars.len() {
        let pair = format!("{}{}", chars[index - 1], chars[index]);
        let common_digraphs = ["ch", "ck", "gh", "ng", "ph", "sh", "th", "wh"];
        if common_digraphs.contains(&pair.as_str()) {
            return false;
        }
    }

    true
}

