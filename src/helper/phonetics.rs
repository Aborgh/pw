pub fn ends_with_vowel(word: &str) -> bool {
    if word.is_empty() {
        return false;
    }

    let last_char = word.chars().last().unwrap().to_ascii_lowercase();
    match last_char {
        'a' | 'e' | 'i' | 'o' | 'u' | 'y' => true,
        _ => false,
    }
}

// Helper function to check if a string contains a vowel
pub fn contains_vowel(word: &str) -> bool {
    for c in word.chars() {
        if "aeiouy".contains(c.to_ascii_lowercase()) {
            return true;
        }
    }
    false
}

// Define phonetic patterns for different styles
pub fn get_phonetics() -> (
    Vec<&'static str>,
    Vec<&'static str>,
    Vec<&'static str>,
    Vec<&'static str>,
    Vec<&'static str>,
) {
    // Initial consonants (word beginnings)
    let initial_consonants = vec![
        // Common single consonants (higher weight for common ones)
        "b", "b", "c", "c", "d", "d", "f", "f", "g", "h", "h", "j", "k", "k", "l", "l", "m", "m",
        "n", "n", "p", "p", "r", "r", "s", "s", "s", "t", "t", "t", "v", "w", "w", "y", "z",
        // Common consonant blends for word starts
        "bl", "br", "ch", "cl", "cr", "dr", "fl", "fr", "gl", "gr", "pl", "pr", "sc", "sh", "sk",
        "sl", "sm", "sn", "sp", "st", "sw", "th", "tr", "tw", "wh",
    ];

    // Middle consonants (within words)
    let middle_consonants = vec![
        // Single consonants (higher weight for common ones)
        "b", "c", "c", "d", "d", "f", "g", "k", "l", "l", "l", "m", "m", "n", "n", "n", "p", "r",
        "r", "r", "s", "s", "s", "t", "t", "t", "v", "x", "z",
        "bb", "cc", "dd", "ff", "gg", "ll", "mm", "nn", "pp", "rr", "ss", "tt",
        // Common blends within words
        "ch", "ck", "ct", "ld", "lf", "lk", "ll", "lm", "ln", "lp", "lt", "mb", "mm", "mp", "nc",
        "nd", "ng", "nk", "nn", "nt", "ph", "pt", "rb", "rc", "rd", "rf", "rg", "rk", "rl", "rm",
        "rn", "rp", "rr", "rs", "rt", "rth", "sc", "sh", "sk", "sl", "sp", "ss", "st", "th",
    ];

    // Final consonants (word endings)
    let final_consonants = vec![
        // Single ending consonants
        "b", "ch", "ck", "d", "f", "ff", "ft", "g", "gh", "k", "l", "ld", "ll", "lt", "m", "n",
        "nd", "ng", "nk", "nt", "p", "r", "rd", "rk", "rl", "rm", "rn", "rp", "rt", "s", "sh",
        "sk", "sp", "ss", "st", "t", "th", "w", "wn", "x", "y", "z",
    ];

    // Vowels with frequency weighting (common vowels appear multiple times)
    let vowels = vec![
        // Single vowels (with frequency weighting)
        "a", "a", "a", "e", "e", "e", "e", "i", "i", "i", "o", "o", "o", "u", "u", "y",
        // Common vowel combinations (digraphs)
        "ae", "ai", "ay", "ea", "ea", "ee", "ee", "ei", "eo", "eu", "ie", "io", "oa", "oe", "oi",
        "oo", "oo", "ou", "oy", "ue", "ui",
    ];

    // Endings that sound natural in English
    let endings = vec![
        // Word endings with weighted frequency
        "able", "acy", "age", "al", "ance", "ate", "ation", "ble", "dom", "ed", "en", "ence", "ent",
        "er", "ern", "ese", "esque", "ess", "est", "ful", "hood", "ia", "ial", "ible", "ic",
        "ical", "ice", "ify", "ile", "ing", "ion", "ious", "ish", "ism", "ist", "ite", "ity",
        "ive", "ize", "less", "ly", "ment", "ness", "or", "ory", "ous", "ship", "sion", "some",
        "tion", "ty", "ure", "ward", "wise", "y", // Short endings
        "a", "o", "y", "ar", "er", "or", "ur", "us", "um", "ix", "ex", "ax",
    ];

    (
        initial_consonants,
        middle_consonants,
        final_consonants,
        vowels,
        endings,
    )
}