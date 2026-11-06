use crate::generator::word::{capitalize, capitalize_first, generate_random_word};
use anyhow::{Result, anyhow};
use rand::Rng;
use rand::distr::{Alphanumeric, SampleString};
use rand::prelude::SliceRandom;

const SYMBOLS: &[u8] = b"!@#$%^&*()-_=+[]{}|;:,.<>?~";

// A password is: words (split by digits or symbols) + a number + a symbol
// Example: "Tornek7mely42!"
pub fn generate_password_with_target_length(
    rng: &mut impl Rng,
    min_length: usize,
    max_length: usize,
    force_capitalize: bool,
) -> Result<String> {
    let length = rng.random_range(min_length..=max_length);

    let number = random_number(rng);
    let symbol = random_symbol(rng).to_string();

    let words_length = length.saturating_sub(number.len() + 1);
    if words_length == 0 {
        return Err(anyhow!("Password length {length} is too short"));
    }

    let words = generate_random_word(words_length, words_length, rng);
    let words = if force_capitalize {
        capitalize(&words)
    } else {
        capitalize_first(&words)
    };

    // Most passwords start with the words, the rest are fully mixed
    let mut parts = vec![number, symbol];
    parts.shuffle(rng);
    if rng.random_bool(0.7) {
        parts.insert(0, words);
    } else {
        let position = rng.random_range(0..=parts.len());
        parts.insert(position, words);
    }

    Ok(parts.join(""))
}

// A number with 1 to 3 digits
fn random_number(rng: &mut impl Rng) -> String {
    let digits = rng.random_range(1..=3);
    let smallest = if digits == 1 { 0 } else { 10usize.pow(digits - 1) };
    let largest = 10usize.pow(digits) - 1;

    rng.random_range(smallest..=largest).to_string()
}

pub fn random_symbol(rng: &mut impl Rng) -> char {
    let index = rng.random_range(0..SYMBOLS.len());
    SYMBOLS[index] as char
}

pub fn get_random_special_chars(rng: &mut impl Rng, count: usize) -> String {
    (0..count).map(|_| random_symbol(rng)).collect()
}

/// Generate a password based on a specific pattern
/// W = Word, N = Number, C/S = Special Character
/// Any other character = Literal character (no quotes needed)
/// "text" or 'text' = Optional quoted literal text
pub fn generate_pattern_password(
    rng: &mut impl Rng,
    pattern: &str,
    min_word_length: usize,
    max_word_length: usize,
    force_capitalize: bool,
) -> Result<String> {
    let mut components = Vec::new();
    let mut chars = pattern.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '"' => {
                let mut literal = String::new();
                let mut found_closing = false;

                while let Some(next_char) = chars.next() {
                    if next_char == '"' {
                        found_closing = true;
                        break;
                    } else if next_char == '\\' {
                        if let Some(escaped_char) = chars.next() {
                            literal.push(escaped_char);
                        }
                    } else {
                        literal.push(next_char);
                    }
                }

                if !found_closing {
                    return Err(anyhow!("Unclosed double quote in pattern"));
                }

                components.push(literal);
            }
            '\'' => {
                let mut literal = String::new();
                let mut found_closing = false;

                while let Some(next_char) = chars.next() {
                    if next_char == '\'' {
                        found_closing = true;
                        break;
                    } else if next_char == '\\' {
                        if let Some(escaped_char) = chars.next() {
                            literal.push(escaped_char);
                        }
                    } else {
                        literal.push(next_char);
                    }
                }

                if !found_closing {
                    return Err(anyhow!("Unclosed single quote in pattern"));
                }

                components.push(literal);
            }
            'W' | 'w' => {
                let word = generate_random_word(min_word_length, max_word_length, rng);
                let word = if force_capitalize {
                    capitalize(&word)
                } else {
                    word
                };
                components.push(word);
            }
            'N' | 'n' => {
                components.push(random_number(rng));
            }
            'C' | 'c' | 'S' | 's' => {
                components.push(get_random_special_chars(rng, 1));
            }
            _ => {
                components.push(c.to_string());
            }
        }
    }

    Ok(components.join(""))
}

pub fn random_chars(rng: &mut impl Rng, minimum_length: usize, maximum_length: usize) -> String {
    let mut password = String::new();
    let password_length = rng.random_range(minimum_length..=maximum_length);

    for _ in 0..password_length {
        if rng.random_bool(0.1) {
            password.push(random_symbol(rng));
        } else {
            password.push(Alphanumeric.sample_string(rng, 1).chars().nth(0).unwrap());
        }
    }

    password
}
