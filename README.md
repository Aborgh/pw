# PW - Easy-to-Remember Password Generator

A command-line tool for generating random English-sounding words. Some words *may* be real words, but that's completely by chance. 

## Features

- Creates passwords with a mix of words, numbers, and special characters
- Customize password length or use length ranges
- Control capitalization options
- Generate multiple passwords at once
- Define custom patterns for password generation
- Output in plain text, JSON, or YAML formats
- Encode passwords in various formats (Base64, URL, SHA256, SHA512, htpasswd)
- Use a specific seed for reproducible passwords

## Installation

### Install (Linux/macOS)

Install the latest version directly with:

```bash
curl -fsSL https://raw.githubusercontent.com/Aborgh/pw/main/install.sh | bash
```

Or using wget:

```bash
wget -q -O- https://raw.githubusercontent.com/Aborgh/pw/main/install.sh | bash
```

### Install from Source with Cargo

If you have Rust and Cargo installed, you can build and install from source:

```bash
# Clone the repository
git clone https://github.com/Aborgh/pw.git
cd pw

# Install directly
cargo install --path .
```

### Download Binaries

Pre-compiled binaries for various platforms are available on the [releases page](https://github.com/Aborgh/pw/releases).

## Usage

### Basic Usage

Generate a random, easy-to-remember password:

```bash
pw
```

### Multiple Passwords

Generate 5 passwords:

```bash
pw 5
```

### Set Specific Length

Generate a password with exactly 12 characters:

```bash
pw --length 12
# or
pw -L 12
```

### Set Length Range

Generate a password between 10 and 16 characters:

```bash
pw --min-length 10 --max-length 16
#or
pw -m 10 -M 16
```

### Force Lowercase

Generate a lowercase password:

```bash
pw --lowercase
# or
pw -l
```

### Force Capitalization

Capitalize first letter of each word:

```bash
pw --capitalize
# or
pw -c
```

### Custom Pattern

Generate a password with a specific pattern:

```bash
pw --pattern WNC  # Word-Number-SpecialCharacter
# or
pw -p WNC
#or 
pw -p "'cool'-W-'cool'" # Will generate "cool-RANDOM_WORD-cool" 
```

#### Pattern Characters:
- `W` or `w`: Word
- `N` or `n`: Number
- `C`, `c`, `S` or `s`: Special Character
- Any other character: Literal
- Text in quotes: Literal text (e.g., `"hello-"W`). Note that words with characters: `W` `S` `C` or `N` needs to be surrounded with `''`. I.e.: `pw -p "'cool'-WW-'stuff'"` 
- If used with `--length (-L)` the length parameter will be on the word. For instance: `pw -p "hello-"W -L 2` would generate `hello-si` or some other random 2-letter word 
### JSON Output

```bash
pw --output json
```

### Encode Password

Generate a password and encode it:

```bash
pw --output json --encode base64
# or 
pw -O json -e base64
```

Available encodings: `base64`, `url`, `sha256`, `sha512`, `htpasswd`

### Deterministic Output

Use a specific seed for reproducible passwords:

```bash
pw --seed 12345
# or
pw -s 12345
```

### Completely Random Password

Generate a random string instead of word-based password:

```bash
pw --random
# or
pw -R
```

## Examples

```bash
# Generate 3 passwords with length 15, all lowercase
pw 3 -L --lowercase
# Generate a password with pattern
pw --pattern "W-N-C-W"

# Generate a JSON output with SHA256 encoding
pw -O json --encode sha256

# Generate a fixed pattern with literal text
pw -p "'my-'W'@'N"
```

## License

This project is licensed under the MIT License
