use crate::generator::password::random_symbol;
use rand::Rng;

// Longer words are split, like "tornek7mely"
const LONGEST_WORD: usize = 9;
const SHORTEST_PART: usize = 4;

// A word is built like this:
//   start sound + vowel + (middle syllables) + ending
// Example: "br" + "a" + "nd" + "o" + "ley" = "brandoley"

// Sounds that can start a word
const STARTS: &[&str] = &[
    "b", "c", "d", "f", "g", "h", "j", "k", "l", "m", "n", "p", "r", "s", "t", "v", "w", "y",
    "b", "c", "d", "h", "l", "m", "p", "r", "s", "t",
    "bl", "br", "ch", "cl", "cr", "dr", "fl", "fr", "gl", "gr", "pl", "pr", "sh", "sk", "sl",
    "sn", "sp", "st", "str", "sw", "th", "tr", "wh",
];

// Single vowels are much more common than vowel pairs
const VOWELS: &[&str] = &[
    "a", "a", "a", "a", "a", "e", "e", "e", "e", "e", "e", "i", "i", "i", "i", "o", "o", "o",
    "o", "u", "u", "ai", "ea", "ee", "oo", "ou",
];

// Sounds that can sit between two vowels
const MIDDLES: &[&str] = &[
    "b", "c", "d", "f", "g", "k", "l", "m", "n", "p", "r", "s", "t", "v", "z",
    "l", "m", "n", "r", "s", "t",
    "ch", "sh", "th", "ld", "lm", "lt", "mb", "nd", "ng", "nt", "rb", "rd", "rk", "rm", "rn",
    "rt", "sk", "st",
];

// Short endings are more common than long ones
const ENDING_LENGTHS: &[usize] = &[0, 1, 1, 1, 2, 2, 2, 3, 3, 4];

// Sounds that can end a word, grouped by length
const ENDINGS: &[&[&str]] = &[
    &[""],
    &["d", "l", "m", "n", "r", "s", "t", "x"],
    &[
        "ck", "ft", "ld", "ll", "nd", "ng", "nt", "rd", "rk", "rn", "rt", "sh", "ss", "st", "th",
        "by", "dy", "la", "ly", "na", "ne", "ny", "ra", "re", "ro", "ry", "te",
    ],
    &[
        "ble", "der", "dle", "ham", "kin", "len", "ley", "lyn", "mer", "ner", "nor", "rin",
        "sen", "son", "ter", "ton", "tor", "ven", "ver", "vin",
    ],
    &[
        "bury", "dale", "dell", "ford", "land", "ling", "mond", "more", "ston", "well", "wick",
        "wood",
    ],
];

pub fn generate_random_word(min_length: usize, max_length: usize, rng: &mut impl Rng) -> String {
    let length = rng.random_range(min_length..=max_length);
    build_words(length, rng)
}

fn build_words(length: usize, rng: &mut impl Rng) -> String {
    if length <= LONGEST_WORD {
        return build_one_word(length, rng);
    }

    // Leave room for the separator and the next part
    let longest_first = LONGEST_WORD.min(length - 1 - SHORTEST_PART);
    let first_length = rng.random_range(SHORTEST_PART..=longest_first);
    let rest_length = length - first_length - 1;

    let first = build_one_word(first_length, rng);
    let separator = random_separator(rng);
    let rest = build_words(rest_length, rng);

    format!("{first}{separator}{rest}")
}

fn random_separator(rng: &mut impl Rng) -> char {
    if rng.random_bool(0.5) {
        char::from(b'0' + rng.random_range(0..10))
    } else {
        random_symbol(rng)
    }
}

fn build_one_word(length: usize, rng: &mut impl Rng) -> String {
    // Some attempts overshoot the length, so keep trying
    loop {
        if let Some(word) = build_word(length, rng) {
            return word;
        }
    }
}

fn build_word(length: usize, rng: &mut impl Rng) -> Option<String> {
    let mut word = String::new();

    // Some words start with a vowel, like "amber"
    if rng.random_bool(0.8) {
        word.push_str(pick(STARTS, rng));
    }
    word.push_str(pick(VOWELS, rng));

    let ending_length = ENDING_LENGTHS[rng.random_range(0..ENDING_LENGTHS.len())];
    while word.len() + ending_length < length {
        word.push_str(pick(MIDDLES, rng));
        word.push_str(pick(VOWELS, rng));
    }

    // Pick an ending that makes the word exactly the right length
    let missing = length.checked_sub(word.len())?;
    word.push_str(pick(ENDINGS[missing], rng));

    Some(word)
}

fn pick<'a>(options: &[&'a str], rng: &mut impl Rng) -> &'a str {
    options[rng.random_range(0..options.len())]
}

// Uppercase the first letter of every part, like "Tornek7Mely"
pub fn capitalize(s: &str) -> String {
    let mut result = String::new();
    let mut at_start = true;

    for c in s.chars() {
        if at_start {
            result.extend(c.to_uppercase());
        } else {
            result.push(c);
        }
        at_start = !c.is_alphabetic();
    }

    result
}

pub fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha20Rng;

    #[test]
    fn test_words_have_exact_length_and_short_parts() {
        let mut rng = ChaCha20Rng::from_seed([0; 32]);

        for length in 1..=40 {
            for _ in 0..200 {
                let word = generate_random_word(length, length, &mut rng);
                assert_eq!(word.len(), length, "Wrong length: '{}'", word);

                for part in word.split(|c: char| !c.is_alphabetic()) {
                    assert!(part.len() <= LONGEST_WORD, "Part too long in '{}'", word);
                }
            }
        }
    }
}
