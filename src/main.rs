mod cli;
mod generator;
mod helper;
mod models;

use crate::cli::cli::{Cli, EncodingFormat, OutputFormat};
use crate::generator::password::{
    generate_password_with_target_length, generate_pattern_password, random_chars,
    random_uppercase_char,
};
use crate::helper::encoder::encode_password;
use crate::models::password_output::PasswordOutput;
use anyhow::{Context, Result};
use clap::error::ErrorKind;
use clap::{CommandFactory, Parser};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use rand_chacha::rand_core::TryRngCore;
fn main() -> Result<()> {
    let cli = Cli::parse();
    command_validation(&cli);

    let mut rng = match cli.seed {
        Some(seed) => {
            if cli.verbose {
                println!("Seed: {}", seed);
            }
            ChaCha8Rng::seed_from_u64(seed)
        },
        None => {
            let random_seed = ChaCha8Rng::from_os_rng()
                .try_next_u64()
                .context("Failed to generate random seed")?;
            if cli.verbose {
                println!("Seed: {}", random_seed);
            }
            ChaCha8Rng::seed_from_u64(random_seed)
        }
    };

    let mut min_length = cli.min_length.unwrap_or(8);
    let mut max_length = cli.max_length.unwrap_or(15);
    if let Some(length) = cli.length {
        min_length = length;
        max_length = length;
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

        if !cli.random {
            if !cli.capitalize && !cli.lowercase {
                password = random_uppercase_char(&password, &mut rng);
            }
        }

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

fn command_validation(cli: &Cli) {
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

    if cli.min_length > cli.max_length {
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

    if cli.min_length.is_some() && cli.min_length.unwrap() < 1 {
        cmd.error(
            ErrorKind::ValueValidation,
            "Minimum length must be greater than 0",
        )
        .exit();
    }
}
