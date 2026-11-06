mod cli;
mod generator;
mod helper;
mod models;

use crate::cli::cli::{Cli, EncodingFormat, OutputFormat};
use crate::generator::password::{
    generate_password_with_target_length, generate_pattern_password, random_chars,
};
use crate::helper::encoder::encode_password;
use crate::models::password_output::PasswordOutput;
use anyhow::Result;
use clap::error::ErrorKind;
use clap::{CommandFactory, Parser};
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;

const MIN_PASSWORD_LENGTH: usize = 12;
const MAX_PASSWORD_LENGTH: usize = 20;

// With --pattern, the lengths apply to each word instead
const MIN_WORD_LENGTH: usize = 8;
const MAX_WORD_LENGTH: usize = 15;

fn main() -> Result<()> {
    let cli = Cli::parse();
    let (min_length, max_length) = length_range(&cli);
    command_validation(&cli, min_length, max_length);

    let mut rng = match cli.seed {
        Some(seed) => ChaCha20Rng::seed_from_u64(seed),
        None => ChaCha20Rng::from_os_rng(),
    };

    if cli.verbose {
        match cli.seed {
            Some(seed) => println!("Seed: {}", seed),
            None => println!("Seed: none (random, not reproducible)"),
        }
    }

    let num_passwords = cli.number_of_passwords;
    let mut passwords = Vec::with_capacity(num_passwords);

    for _ in 0..num_passwords {
        let mut password = if cli.random {
            random_chars(&mut rng, min_length, max_length)
        } else if let Some(pattern) = &cli.pattern {
            generate_pattern_password(&mut rng, pattern, min_length, max_length, cli.capitalize)?
        } else {
            generate_password_with_target_length(&mut rng, min_length, max_length, cli.capitalize)?
        };
        

        if cli.lowercase {
            password = password.to_lowercase();
        }

        if matches!(cli.format, OutputFormat::Text) {
            println!("{}", password);
        } else {
            let encoded = match &cli.encode {
                EncodingFormat::None => None,
                encoding_format => Some(encode_password(&password, encoding_format)?),
            };

            let output = PasswordOutput {
                decoded: password,
                encoded,
            };

            passwords.push(output);
        }
    }

    // Output structured formats if needed
    match cli.format {
        OutputFormat::Json => {
            let json = serde_json::to_string_pretty(&passwords)?;
            println!("{}", json);
        }
        OutputFormat::Yaml => {
            let yaml = serde_yml::to_string(&passwords)?;
            println!("{}", yaml);
        }
        _ => {}
    }

    Ok(())
}

fn length_range(cli: &Cli) -> (usize, usize) {
    if let Some(length) = cli.length {
        return (length, length);
    }

    let (default_min, default_max) = if cli.pattern.is_some() {
        (MIN_WORD_LENGTH, MAX_WORD_LENGTH)
    } else {
        (MIN_PASSWORD_LENGTH, MAX_PASSWORD_LENGTH)
    };

    let min_length = cli.min_length.unwrap_or(default_min);
    let max_length = cli.max_length.unwrap_or(default_max);
    (min_length, max_length)
}

fn command_validation(cli: &Cli, min_length: usize, max_length: usize) {
    let mut cmd = Cli::command();

    if matches!(cli.format, OutputFormat::Text) && !matches!(cli.encode, EncodingFormat::None) {
        cmd.error(
            ErrorKind::ArgumentConflict,
            "Encoding options (-e) cannot be used with text output format (-O text)",
        )
        .exit();
    }

    if (cli.length.is_some() && cli.max_length.is_some())
        || (cli.length.is_some() && cli.min_length.is_some())
    {
        cmd.error(
            ErrorKind::ValueValidation,
            "Cannot specify both length and min/max length\n\nUse --length to specify a single length\nUse --min-length and --max-length to specify a range of lengths\n",
        ).exit();
    }

    if min_length > max_length {
        cmd.error(
            ErrorKind::ValueValidation,
            "Minimum length cannot be greater than maximum length",
        )
        .exit();
    }

    if cli.random && cli.pattern.is_some() {
        cmd.error(
            ErrorKind::ArgumentConflict,
            "Cannot specify both random and pattern",
        )
        .exit();
    }

    if min_length < 1 {
        cmd.error(
            ErrorKind::ValueValidation,
            "Minimum length must be greater than 0",
        )
        .exit();
    }

    if cli.pattern.is_none() && min_length < MIN_PASSWORD_LENGTH {
        cmd.error(
            ErrorKind::ValueValidation,
            format!("Passwords must be at least {MIN_PASSWORD_LENGTH} characters long"),
        )
        .exit();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generator::password::get_random_special_chars;
    use rand::Rng;

    #[test]
    fn test_password_uniqueness() {
        let mut rng = ChaCha20Rng::from_os_rng();
        let mut passwords: Vec<String> = Vec::new();

        let min = rng.random_range(5..8);
        let max = rng.random_range(min..15);
        
        for _ in 0..1000 {
            let password = generate_password_with_target_length(&mut rng, min, max, false);
            passwords.push(password.unwrap());
        }

        for i in 0..passwords.len() {
            let current_password = &passwords[i];
            for j in (i + 1)..passwords.len() {
                let other_password = &passwords[j];
                assert_ne!(
                    current_password,
                    other_password,
                    "Found duplicate passwords at positions {} and {}: '{}'",
                    i, j, current_password
                );
            }
        }
    }

    #[test]
    fn test_password_substring_containment() {
        let mut rng = ChaCha20Rng::from_seed([0; 32]);
        let mut passwords: Vec<String> = Vec::new();

        for _ in 0..1000 {
            let password = generate_password_with_target_length(&mut rng, 8, 15, false);
            passwords.push(password.unwrap());
        }

        for i in 0..passwords.len() {
            let current_password = &passwords[i];
            for j in 0..passwords.len() {
                if i != j {
                    let other_password = &passwords[j];
                    assert!(
                        !other_password.contains(current_password),
                        "Password at position {} ('{}') is contained within password at position {} ('{}')",
                        i, current_password, j, other_password
                    );
                }
            }
        }
    }

    #[test]
    fn test_combined_password_uniqueness_and_containment() {
        let mut rng = ChaCha20Rng::from_seed([0; 32]);
        let mut passwords: Vec<String> = Vec::new();

        for _ in 0..1000 {
            let password = generate_password_with_target_length(&mut rng, 8, 15, false);
            passwords.push(password.unwrap());
        }

        for i in 0..passwords.len() {
            let current_password = &passwords[i];
            for j in (i + 1)..passwords.len() {
                let other_password = &passwords[j];

                assert_ne!(
                    current_password,
                    other_password,
                    "Found duplicate passwords at positions {} and {}: '{}'",
                    i, j, current_password
                );

                assert!(
                    !current_password.contains(other_password),
                    "Password at position {} ('{}') contains password at position {} ('{}')",
                    i, current_password, j, other_password
                );

                assert!(
                    !other_password.contains(current_password),
                    "Password at position {} ('{}') contains password at position {} ('{}')",
                    j, other_password, i, current_password
                );
            }
        }
    }

    #[test]
    fn test_symbols_are_never_skipped() {
        let mut rng = ChaCha20Rng::from_seed([0; 32]);

        for _ in 0..10_000 {
            assert_eq!(get_random_special_chars(&mut rng, 3).len(), 3);
        }
    }

    #[test]
    fn test_default_passwords_have_a_symbol() {
        let mut rng = ChaCha20Rng::from_seed([0; 32]);

        for _ in 0..1000 {
            let password = generate_password_with_target_length(
                &mut rng,
                MIN_PASSWORD_LENGTH,
                MAX_PASSWORD_LENGTH,
                false,
            )
            .unwrap();

            let has_symbol = password.chars().any(|c| !c.is_alphanumeric());
            assert!(has_symbol, "No symbol in '{}'", password);
        }
    }
}