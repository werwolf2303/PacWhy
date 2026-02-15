#!/usr/bin/env bash

set -euo pipefail

if [[ $EUID -ne 0 ]]; then
    echo "This script must be run as root!" >&2
    exit 1
fi

INSTALL_DIR="/opt/PacWhy"
BIN_DIR="$INSTALL_DIR"
SCRIPTS_DIR="$INSTALL_DIR/scripts"
WRAPPER="$SCRIPTS_DIR/pacman"
SYSTEM_PACMAN="/usr/bin/pacman"
PROFILE_D="/etc/profile.d/pacwhy.sh"

RANDOM_SUFFIX=$(printf '%08x' "$RANDOM$RANDOM$RANDOM" | cut -c1-8)
REAL_PACMAN_RANDOM="/usr/bin/pacman.${RANDOM_SUFFIX}"

cat << 'EOF'
PacWhy Installer
================
This will:
  • Build PacWhy in release mode
  • Install to /opt/PacWhy
  • Place wrapper at /opt/PacWhy/scripts/pacman
  • Move real pacman to a randomized name (/usr/bin/pacman.XXXXXXXX)
  • Replace %PACMAN_PATH% in the wrapper with the real randomized path
    (written as: PACMAN="/usr/bin/pacman.XXXXXXXX")
  • Symlink wrapper → /usr/bin/pacman
  • Add /opt/PacWhy/bin to PATH

EOF

read -p "Continue? This shadows the system pacman binary [y/N] " confirm
[[ "$confirm" =~ ^[Yy]$ ]] || exit 1

# Safety check
if [[ -L "$SYSTEM_PACMAN" ]]; then
    echo "Warning: $SYSTEM_PACMAN is already a symlink."
    read -p "Overwrite anyway? [y/N] " ow
    [[ "$ow" =~ ^[Yy]$ ]] || exit 1
fi

mkdir -p "$BIN_DIR" "$SCRIPTS_DIR"
chown -R root:root "$INSTALL_DIR"
chmod -R 755 "$INSTALL_DIR"

echo "Building PacWhy (release mode)..."
cargo build --release || { echo "Build failed"; exit 1; }

echo "Installing files..."
cp target/release/PacWhy "$BIN_DIR/PacWhy"
chmod 755 "$BIN_DIR/PacWhy"

cp -r scripts/* "$SCRIPTS_DIR/" 2>/dev/null || true
chmod 755 "$SCRIPTS_DIR/"*

# Move real pacman to randomized name (only if not already done)
if [[ ! -f "$REAL_PACMAN_RANDOM" && -f "$SYSTEM_PACMAN" && ! -L "$SYSTEM_PACMAN" ]]; then
    mv "$SYSTEM_PACMAN" "$REAL_PACMAN_RANDOM"
    chmod 755 "$REAL_PACMAN_RANDOM"
fi

if [[ ! -f "$REAL_PACMAN_RANDOM" ]]; then
    echo "ERROR: Could not create or find randomized pacman binary"
    exit 1
fi

# Inject the real path into the wrapper
if [[ ! -f "$WRAPPER" ]]; then
    echo "ERROR: Wrapper script not found at $WRAPPER"
    exit 1
fi

sed -i "s|%PACMAN_PATH%|$REAL_PACMAN_RANDOM|g" "$WRAPPER"

# Create symlink to wrapper
ln -sf "$WRAPPER" "$SYSTEM_PACMAN"

# Add /opt/PacWhy/bin to PATH
if [[ ! -f "$PROFILE_D" ]] || ! grep -q "PacWhy" "$PROFILE_D" 2>/dev/null; then
    cat > "$PROFILE_D" << 'EOF'
# PacWhy - allow running the PacWhy binary directly
if [[ ":${PATH}:" != *":/opt/PacWhy/bin:"* ]]; then
    export PATH="/opt/PacWhy/bin${PATH:+:$PATH}"
fi
EOF
    chmod 644 "$PROFILE_D"
fi

cat << EOF

Installation complete.

  Wrapper:              $WRAPPER  → symlinked to $SYSTEM_PACMAN
  Real pacman:          $REAL_PACMAN_RANDOM
  PacWhy binary:        $BIN_DIR/PacWhy   (run as: PacWhy ...)

EOF