# PW - Easy-to-Remember Password Generator

<div align="center">

[![Status](https://img.shields.io/badge/status-active-success.svg)](https://github.com/Aborgh/pw)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![GitHub release](https://img.shields.io/github/v/release/Aborgh/pw?color=blue)](https://github.com/Aborgh/pw/releases)

  <img src="assets/demo.gif" alt="pw demo">

*A command-line tool for generating random English-sounding words. Some words **may** be real words, but that's
completely by chance.*
</div>

## ✨ Features

<table>
  <tr>
    <td>✅ Creates passwords with a mix of words, numbers, and special characters</td>
    <td>✅ Customize password length or use length ranges</td>
  </tr>
  <tr>
    <td>✅ Control capitalization options</td>
    <td>✅ Generate multiple passwords at once</td>
  </tr>
  <tr>
    <td>✅ Define custom patterns for password generation</td>
    <td>✅ Output in plain text, JSON, or YAML formats</td>
  </tr>
  <tr>
    <td>✅ Encode passwords in various formats (Base64, URL, SHA256, SHA512, htpasswd)</td>
    <td>✅ Use a specific seed for reproducible passwords</td>
  </tr>
</table>

## 📦 Installation

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

## 🚀 Usage

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
- Text in quotes: Literal text (e.g., `"hello-"W`). Note that words with characters: `W` `S` `C` or `N` needs to be
  surrounded with `''`. I.e.: `pw -p "'cool'-WW-'stuff'"`
- If used with `--length (-L)` the length parameter will be on the word. For instance: `pw -p "hello-"W -L 2` would
  generate `hello-si` or some other random 2-letter word

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

| Available Encodings |
  |:-------------------:|
|      `base64`       |
|        `url`        |
|      `sha256`       |
|      `sha512`       |
|     `htpasswd`      |

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

## 📋 Examples

<table>
  <tr>
    <th>Command</th>
    <th>Description</th>
  </tr>
  <tr>
    <td><code>pw 3 -L --lowercase</code></td>
    <td>Generate 3 passwords with length 15, all lowercase</td>
  </tr>
  <tr>
    <td><code>pw --pattern "W-N-C-W"</code></td>
    <td>Generate a password with pattern</td>
  </tr>
  <tr>
    <td><code>pw -O json --encode sha256</code></td>
    <td>Generate a JSON output with SHA256 encoding</td>
  </tr>
  <tr>
    <td><code>pw -p "'my-'W'@'N"</code></td>
    <td>Generate a fixed pattern with literal text</td>
  </tr>
</table>

## 📄 License

This project is licensed under the MIT License
