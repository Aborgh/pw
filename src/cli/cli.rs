use clap::{Parser, ValueEnum};
#[derive(Debug, Clone, ValueEnum)]
pub enum EncodingFormat {
    None,
    Base64,
    Url,
    Sha256,
    Sha512,
    Htpasswd,
}
#[derive(Parser, Debug)]
#[command(
    version,
    about = "Generate random easy to remember passwords",
    name = "pw",
    author = "Andreas Borgh",
    long_about = "Generates random password based on a pattern: Word Character Word Number and scramble these for output.\
    \nFor more controlled output use -p with option W (Word), N (Number) and C (Special Character)."
)]
pub struct Cli {
    /// Number of passwords to generate. This will create N number of random passwords
    #[arg(default_value = "1")]
    pub number_of_passwords: usize,
    
    /// Set a fixed length instead of using -m and -M
    #[arg(short = 'L', long)]
    pub length: Option<usize>,

    /// Force lowercase characters on generated passwords
    #[arg(short = 'l', long, default_value = "false")]
    pub lowercase: bool,
    
    /// Generate a completely random password
    #[arg(short = 'R', long, default_value = "false")]
    pub random: bool,
    
    /// Minimum word length
    #[arg(short, long)]
    pub min_length: Option<usize>,

    /// Maximum word length
    #[arg(short = 'M', long)]
    pub max_length: Option<usize>,

    /// Output format
    #[arg(short = 'O', long = "output", default_value = "text")]
    pub format: OutputFormat,

    /// Use seed for deterministic output, will be deterministic for multiple passwords
    #[arg(short, long)]
    pub seed: Option<u64>,

    /// Force capitalization of generated words
    #[arg(short, long)]
    pub capitalize: bool,

    /// Will print the seed
    #[arg(short, long)]
    pub verbose: bool,
    /// Pattern for password generation (W=Word, N=Number, C/S=Special Character). Use single or double quotes for fixed characters.
    /// Example: "'cool-'W-'cool'" will print cool-RANDOM_WORD-cool
    #[arg(short = 'p', long)]
    pub pattern: Option<String>,

    /// Encode the output password. htpasswd = slow by nature
    #[arg(short = 'e', long, value_enum, default_value = "none")]
    pub encode: EncodingFormat,
}

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum OutputFormat {
    Text,
    Yaml,
    Json
}
