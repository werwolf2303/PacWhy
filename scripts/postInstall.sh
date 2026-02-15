#!/usr/bin/env bash

# Written by AI

set -euo pipefail

# For testing — change to real path when ready
PACWHY_BIN="/opt/PacWhy/PacWhy"

# ────────────────────────────────────────────────
# Parse arguments
# ────────────────────────────────────────────────

main_pkgs=()

for arg in "$@"; do
    main_pkgs+=("$arg")
done

# Interactive fallback
if [[ ${#main_pkgs[@]} -eq 0 ]]; then
    echo ""
    echo "  PacWhy"
    echo "  ─────────────────────────────────"
    echo -n "Main packages (explicitly installed): "
    read -r input
    read -ra main_pkgs <<< "$input"
fi

[[ ${#main_pkgs[@]} -eq 0 ]] && {
    echo "No packages provided."
    exit 1
}

# ────────────────────────────────────────────────
# Resolve ambiguous / partial package names
# ────────────────────────────────────────────────

resolve_packages() {
    local -a input=("$@")
    local -a resolved=()

    for name in "${input[@]}"; do
        [[ -z "$name" ]] && continue

        mapfile -t candidates < <(pacman -Qq | grep -F -- "$name" 2>/dev/null || true)

        if [[ ${#candidates[@]} -eq 0 ]]; then
            resolved+=("$name")
            continue
        fi

        if [[ ${#candidates[@]} -eq 1 ]]; then
            resolved+=("${candidates[0]}")
            continue
        fi

        # Multiple matches → user choice
        echo "Multiple packages match '$name':"
        local i=1
        local -A choice_map
        for cand in "${candidates[@]}"; do
            local ver
            ver=$(pacman -Q "$cand" 2>/dev/null | cut -d' ' -f2 || echo "unknown")
            printf "  %2d) %s (%s)\n" "$i" "$cand" "$ver"
            choice_map[$i]="$cand"
            ((i++))
        done

        echo -n "Select (number(s) comma-separated, all, skip): "
        read -r selection
        selection="${selection//,/ }"

        if [[ "$selection" =~ ^(all|All|a|A)$ ]]; then
            resolved+=("${candidates[@]}")
        elif [[ -z "$selection" || "$selection" =~ ^(skip|s|no|Skip)$ ]]; then
            echo "  ↳ skipping $name"
        else
            for num in $selection; do
                [[ ${choice_map[$num]:-} ]] && resolved+=("${choice_map[$num]}")
            done
        fi
    done

    printf '%s\n' "${resolved[@]}"
}

echo ""
echo "Resolving package names..."
echo "───────────────────────────"

readarray -t final_main < <(resolve_packages "${main_pkgs[@]}")

echo ""
echo "Packages to record:"
printf '  - %s\n' "${final_main[@]}"
echo ""

# ────────────────────────────────────────────────
# Process packages
# ────────────────────────────────────────────────

declare -A seen

for pkg in "${final_main[@]}"; do
    [[ ${seen[$pkg]:-0} -eq 1 ]] && continue
    seen[$pkg]=1

    # Get version and description from pacman
    if pacman -Qq "$pkg" &>/dev/null; then
        # Package is installed - use -Qi
        desc=$(pacman -Qi "$pkg" 2>/dev/null | grep "^Description" | sed 's/^Description[[:space:]]*:[[:space:]]*//' || echo "$pkg")
    else
        # Package not installed yet - use -Si
        desc=$(pacman -Si "$pkg" 2>/dev/null | grep "^Description" | sed 's/^Description[[:space:]]*:[[:space:]]*//' || echo "$pkg")
    fi

    echo "Description: ${desc}"
    echo ""

    echo "PacWhy: Reason for installing ${pkg}?"
    read -r reason

    # Trim whitespace
    reason="${reason#"${reason%%[![:space:]]*}"}"
    reason="${reason%"${reason##*[![:space:]]}"}"

    echo "  would execute:"
    $PACWHY_BIN add \
        --name "$pkg" \
        --description "$desc" \
        --reason "$reason" \
        --isDependency false
done

echo ""
echo "Package added to PacWhy"
echo ""