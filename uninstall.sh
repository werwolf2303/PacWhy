#!/usr/bin/env bash

set -euo pipefail

if [[ $EUID -ne 0 ]]; then
    echo "This script must be run as root!" >&2
    exit 1
fi

INSTALL_DIR="/opt/PacWhy"
WRAPPER="$INSTALL_DIR/scripts/pacman"
SYSTEM_PACMAN="/usr/bin/pacman"
PROFILE_D="/etc/profile.d/pacwhy.sh"

cat << 'EOF'
PacWhy Uninstaller
==================
This will:
  • Read PACMAN=... line from the wrapper script to find the real pacman binary
  • Remove /opt/PacWhy completely
  • Remove PATH file
  • Restore original pacman from the path read from wrapper
EOF

echo ""
ls -l "$SYSTEM_PACMAN" 2>/dev/null || echo "  (pacman missing)"
echo ""

read -p "Proceed with uninstall and restore? [y/N] " confirm
[[ "$confirm" =~ ^[Yy]$ ]] || exit 1

# Extract PACMAN= value from wrapper
REAL_BACKUP=""
if [[ -f "$WRAPPER" ]]; then
    # Extract value between quotes after PACMAN=
    REAL_BACKUP=$(sed -n 's/^PACMAN="\([^"]*\)".*/\1/p' "$WRAPPER")

    # Fallback: more lenient parsing (handles spaces, single quotes, no quotes)
    if [[ -z "$REAL_BACKUP" ]]; then
        REAL_BACKUP=$(grep '^PACMAN=' "$WRAPPER" | cut -d= -f2- | tr -d " '\"" | grep '^/usr/bin/pacman\.[a-z0-9]\{8\}$' || echo "")
    fi

    if [[ -n "$REAL_BACKUP" && -f "$REAL_BACKUP" ]]; then
        echo "Found real pacman path in wrapper: $REAL_BACKUP"
    else
        echo "WARNING: Could not extract valid PACMAN path from $WRAPPER"
        REAL_BACKUP=""
    fi
else
    echo "WARNING: Wrapper script $WRAPPER not found"
fi

# Safety backup of current /usr/bin/pacman
if [[ -e "$SYSTEM_PACMAN" ]]; then
    cp -a "$SYSTEM_PACMAN" "/usr/bin/pacman.pacwhy-old-$(date +%Y-%m-%d-%H%M)" 2>/dev/null || true
fi

echo "Removing PacWhy installation..."
rm -rf "$INSTALL_DIR"

echo "Removing profile.d file..."
rm -f "$PROFILE_D"

if [[ -n "$REAL_BACKUP" && -f "$REAL_BACKUP" ]]; then
    echo "Restoring original pacman from $REAL_BACKUP → $SYSTEM_PACMAN"
    mv "$REAL_BACKUP" "$SYSTEM_PACMAN"
    chmod 755 "$SYSTEM_PACMAN"
else
    echo "ERROR: No valid backup binary found or file does not exist."
    echo "pacman is likely broken now."
    echo "Manual recovery options:"
    echo "  - Check /usr/bin for pacman.* files and move one back manually"
    echo "  - Reinstall pacman package from live USB"
    exit 2
fi

# Quick validation
echo ""
echo "Verifying restored pacman..."
if "$SYSTEM_PACMAN" --version >/dev/null 2>&1; then
    echo "OK: pacman --version works"
else
    echo "WARNING: Restored pacman does not appear to be functional"
    echo "Check: ls -l $SYSTEM_PACMAN"
    echo "       file $SYSTEM_PACMAN"
fi

echo ""
echo "Uninstall finished."
echo "If everything went well, pacman is now the original binary again."
echo "Start a new shell if needed."