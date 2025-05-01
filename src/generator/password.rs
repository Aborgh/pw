use crate::generator::word::{capitalize, generate_random_word};
use anyhow::{Result, anyhow};
use rand::Rng;
use rand::distr::{Alphanumeric, SampleString};
use rand::prelude::SliceRandom;
use rand_chacha::ChaCha8Rng;

pub fn generate_password_with_target_length(
    rng: &mut ChaCha8Rng,
    min_length: usize,
    max_length: usize,
    force_capitalize: bool,
) -> Result<String> {
    let target_length = rng.random_range(min_length..=max_length);

    let mut remaining_length = target_length;

    let mut components = Vec::new();

    let start_with_word = rng.random_bool(0.7);

    let special_char = get_random_special_chars(rng, 1);
    components.push(special_char.clone());
    remaining_length = remaining_length.saturating_sub(special_char.len());

    let num_digits = rng.random_range(1..=3);
    let max_val = 10usize.pow(num_digits as u32) - 1;
    let min_val = if num_digits > 1 {
        10usize.pow((num_digits - 1) as u32)
    } else {
        0
    };
    let number = rng.random_range(min_val..=max_val).to_string();
    components.push(number.clone());
    remaining_length = remaining_length.saturating_sub(number.len());

    if remaining_length >= 3 {
        let num_words = if remaining_length >= 8 { 2 } else { 1 };

        for i in 0..num_words {
            let is_last_word = i == num_words - 1;

            let word_min_length = 3;
            let word_max_length = if is_last_word {
                remaining_length
            } else {
                remaining_length.saturating_sub(3).min(9)
            };

            if word_max_length < word_min_length {
                break;
            }

            let word = generate_random_word(word_min_length, word_max_length, rng);

            let word = if force_capitalize {
                capitalize(&word)
            } else {
                word
            };

            components.push(word.clone());
            remaining_length = remaining_length.saturating_sub(word.len());

            if remaining_length < 3 {
                break;
            }
        }
    }

    // These will always run if --length is 1 or 2
    if remaining_length >= 2 {
        if rng.random_bool(0.5) {
            // Add another number
            let remaining_digits = remaining_length.min(3);
            let max_val = 10usize.pow(remaining_digits as u32) - 1;
            let number = rng.random_range(0..=max_val).to_string();
            components.push(number);
        } else {
            // Add more special characters
            let special = get_random_special_chars(rng, remaining_length);
            components.push(special);
        }
    }

    // Shuffle everything
    if start_with_word {
        // Find word
        let word_indices: Vec<usize> = components
            .iter()
            .enumerate()
            .filter(|(_, component)| component.chars().any(|c| c.is_alphabetic()))
            .map(|(i, _)| i)
            .collect();

        if !word_indices.is_empty() {
            // Select a random word
            let word_idx = word_indices[rng.random_range(0..word_indices.len())];
            let word_component = components[word_idx].clone();

            // Remove it and place it at the beginning
            components.remove(word_idx);
            components.shuffle(rng);
            components.insert(0, word_component);
        } else {
            // No word components found, shuffle everything
            components.shuffle(rng);
        }
    } else {
        // Randomize order
        components.shuffle(rng);
    }

    // Combine components into a password
    let password = components.join("");

    // Verify the password meets our length requirements
    if password.len() < min_length || password.len() > max_length {
        return generate_password_with_target_length(rng, min_length, max_length, force_capitalize);
    }

    Ok(password)
}

pub fn get_random_special_chars(rng: &mut ChaCha8Rng, count: usize) -> String {
    let special_chars = "!@#$%^&*()-_=+[]{}|;:,.<>?~";
    let mut result = String::with_capacity(count);

    for _ in 0..count {
        let random_special_char_id = rng.random_range(0..=special_chars.len());
        if let Some(c) = special_chars.chars().nth(random_special_char_id) {
            result.push(c);
        }
    }

    result
}

/// Generate a password based on a specific pattern
/// W = Word, N = Number, C/S = Special Character
/// "text" or 'text' = Literal text to include
/// Generate a password based on a specific pattern
/// W = Word, N = Number, C/S = Special Character
/// "text" or 'text' = Literal text to include at that position
/// Generate a password based on a specific pattern
/// W = Word, N = Number, C/S = Special Character
/// Any other character = Literal character (no quotes needed)
/// "text" or 'text' = Optional quoted literal text
pub fn generate_pattern_password(
    rng: &mut ChaCha8Rng,
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
                let num_digits = rng.random_range(1..=3);
                let max_val = 10usize.pow(num_digits as u32) - 1;
                let min_val = if num_digits > 1 {
                    10usize.pow((num_digits - 1) as u32)
                } else {
                    0
                };
                let number = rng.random_range(min_val..=max_val).to_string();
                components.push(number);
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

pub fn random_uppercase_char(word: &str, rng: &mut ChaCha8Rng) -> String {
    let mut final_word = String::with_capacity(word.len());
    for c in word.chars() {
        if c.is_alphabetic() & rng.random_bool(0.1) {
            final_word.push(c.to_ascii_uppercase())
        } else {
            final_word.push(c);
        }
    }
    final_word
}

pub fn random_chars(rng: &mut ChaCha8Rng, minimum_length: usize, maximum_length: usize) -> String {
    let mut password = String::new();
    let special_chars = "!@#$%^&*()-_=+[]{}|;:,.<>?~";
    let password_length = rng.random_range(minimum_length..=maximum_length);

    for _ in 0..password_length {
        let special_char_index = rng.random_range(0..special_chars.len());
        if rng.random_bool(0.1) {
            password.push(special_chars.chars().nth(special_char_index).unwrap());
        } else {
            password.push(Alphanumeric.sample_string(rng, 1).chars().nth(0).unwrap());
        }
    }

    password
}
