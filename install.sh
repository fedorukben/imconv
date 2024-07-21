#!/bin/bash

check_error() {
	if [ $? -ne 0 ]; then
		echo "An error has occured. Exiting..."
		exit 1
	fi
}

if ! command -v cargo &> /dev/null; then
	echo "Rust is not installed. Please install Rust first."
	exit 1
fi

echo "Building the project..."
cargo build --release
check_error

BINARY_PATH="target/release/imgconv"
INSTALL_PATH="/usr/local/bin/imgconv"
USER_INSTALL_PATH="$HOME/.local/bin/imgconv"

if [ -w /usr/local/bin ]; then
	INSTALL_DIR="/usr/local/bin"
else
	INSTALL_DIR="$HOME/.local/bin"
	mkdir -p "$INSTALL_DIR"
	if [[ ":$PATH:" != *":$HOME/.local/bin:"* ]]; then
		echo 'export PATH="$HOME/.local/bin:$PATH"' >> "$HOME/.bashrc"
		source "$HOME/.bashrc" 2> /dev/null || true
		echo "$HOME/.local/bin has been added to your PATH. Please restart your terminal."
	fi
fi

echo "Installing the binary to $INSTALL_DIR..."
mv "$BINARY_PATH" "$INSTALL_DIR"
check_error

echo "Installation complete."
