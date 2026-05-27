#!/usr/bin/env bash
set -e

VERSION="latest"
REPO="Aborgh/pw"

# Text formatting
BOLD="\033[1m"
GREEN="\033[0;32m"
YELLOW="\033[0;33m"
RED="\033[0;31m"
BLUE="\033[0;34m"
RESET="\033[0m"

# Print a formatted message
print_message() {
  echo -e "${BOLD}${BLUE}==>${RESET} ${BOLD}$1${RESET}"
}

print_success() {
  echo -e "${BOLD}${GREEN}==>${RESET} ${BOLD}$1${RESET}"
}

print_warning() {
  echo -e "${BOLD}${YELLOW}==>${RESET} ${BOLD}$1${RESET}"
}

print_error() {
  echo -e "${BOLD}${RED}==>${RESET} ${BOLD}$1${RESET}"
}

# Determine OS and architecture
detect_os_arch() {
  OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
  ARCH="$(uname -m)"
  
  # Map architecture names
  case "$ARCH" in
    x86_64)
      ARCH="x86_64"
      ;;
    aarch64|arm64)
      ARCH="arm64"
      ;;
    *)
      print_error "Unsupported architecture: $ARCH"
      exit 1
      ;;
  esac
  
  # Map OS names
  case "$OS" in
    linux)
      OS="linux"
      ;;
    darwin)
      OS="macos"
      ;;
    *)
      print_error "Unsupported operating system: $OS. This installer only supports Linux and macOS."
      exit 1
      ;;
  esac
}

# Determine install directory based on OS
get_install_dir() {
  if [ -w "/usr/local/bin" ]; then
    INSTALL_DIR="/usr/local/bin"
  else
    INSTALL_DIR="$HOME/.local/bin"
  fi

  # Create directory if it doesn't exist
  if [ ! -d "$INSTALL_DIR" ]; then
    mkdir -p "$INSTALL_DIR"
  fi
}

# Check if the directory is in PATH
check_path() {
  if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    print_warning "The installation directory $INSTALL_DIR is not in your PATH!"
    
    # Suggest how to add to PATH based on shell
    SHELL_NAME="$(basename "$SHELL")"
    case "$SHELL_NAME" in
      bash)
        print_warning "Run this command to add it to your path:"
        echo -e "echo 'export PATH=\"\$PATH:$INSTALL_DIR\"' >> ~/.bashrc && source ~/.bashrc"
        ;;
      zsh)
        print_warning "Run this command to add it to your path:"
        echo -e "echo 'export PATH=\"\$PATH:$INSTALL_DIR\"' >> ~/.zshrc && source ~/.zshrc"
        ;;
      *)
        print_warning "Add $INSTALL_DIR to your PATH to use the pw command without specifying the full path."
        ;;
    esac
  fi
}

# Download binary from GitHub releases
download_binary() {
  if [ "$VERSION" = "latest" ]; then
    RELEASE_URL="https://api.github.com/repos/$REPO/releases/latest"
    print_message "Fetching latest release info..."
    
    if command -v curl &> /dev/null; then
      VERSION=$(curl -s $RELEASE_URL | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')
    elif command -v wget &> /dev/null; then
      VERSION=$(wget -q -O- $RELEASE_URL | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')
    else
      print_error "Neither curl nor wget found. Please install one of them and try again."
      exit 1
    fi
    
    if [ -z "$VERSION" ]; then
      print_error "Failed to fetch the latest version. Check your internet connection or GitHub rate limits."
      exit 1
    fi
  fi
  
  print_message "Installing pw version $VERSION for $OS on $ARCH..."
  
  # Set download URL
  BINARY_NAME="pw-$OS-$ARCH"
  DOWNLOAD_URL="https://github.com/$REPO/releases/download/$VERSION/$BINARY_NAME"
  LOCAL_BINARY="$INSTALL_DIR/pw"
  
  # Download the binary
  print_message "Downloading from $DOWNLOAD_URL..."
  if command -v curl &> /dev/null; then
    curl -L -s --progress-bar "$DOWNLOAD_URL" -o "$LOCAL_BINARY"
  elif command -v wget &> /dev/null; then
    wget --show-progress -q "$DOWNLOAD_URL" -O "$LOCAL_BINARY"
  else
    print_error "Neither curl nor wget found. Please install one of them and try again."
    exit 1
  fi
  
  # Make it executable
  chmod +x "$LOCAL_BINARY"
}

# Verify installation
verify_installation() {
  if command -v "$INSTALL_DIR/pw" &> /dev/null; then
    print_success "Installation successful! pw is now available at $INSTALL_DIR/pw"
    if [ -x "$INSTALL_DIR/pw" ]; then
      VERSION_OUTPUT=$("$INSTALL_DIR/pw" --version)
      print_success "Installed version: $VERSION_OUTPUT"
      print_message "Try it out with: pw --help"
    else
      print_warning "Binary exists but may not be executable. Check permissions."
    fi
  else
    print_error "Installation may have failed. Check if the binary exists at $INSTALL_DIR/pw"
    exit 1
  fi
}

# Main installation process
main() {
  print_message "Welcome to the pw installer!"
  
  # Check if we're running with root permissions
  if [ "$(id -u)" -eq 0 ]; then
    print_warning "Running as root. The binary will be installed system-wide."
  fi
  
  detect_os_arch
  get_install_dir
  download_binary
  verify_installation
  check_path
  
  print_success "Thank you for installing pw!"
}

main "$@"