# PacWhy

**The solution when you have ever asked: *Why did I install this package?***

PacWhy helps you keep track of the packages you install and the reasons behind them.
Each package is stored in a local database along with a description and a user-defined reason.

Example:

```bash
sudo pacman -S example-package
Enter reason: Required for project XYZ
```

Instead of forgetting why something was installed months ago, PacWhy provides a clear and searchable record.

---

## Overview

PacWhy is a system package organization manager designed for Arch Linux. It allows you to:

* Record installed packages with a description and reason
* Update or remove stored entries
* Search through previously recorded packages
* Maintain a documented history of installation decisions

This is particularly useful for development environments, long-running systems, or minimal installations where clarity matters.

---

## Disclaimer

PacWhy wraps the pacman binary to intercept installation commands and prompt for a reason before proceeding.

- The original ```pacman``` binary is not modified.

- PacWhy acts as a wrapper and forwards commands to the real ```pacman```.

- Ensure you understand how the wrapper is installed and how your $PATH is configured.

- PacWhy is provided as-is, without any warranties or guarantees.

---

## Prerequisites

* Arch Linux
  (Arch-based distributions have not been tested)
* Rust compiler toolchain

---

## Installation

1. Clone the repository from GitHub:

   ```bash
   git clone https://github.com/werwolf2303/PacWhy
   cd pacwhy
   ```

2. Run the installation script:

   ```bash
   ./install.sh
   ```

---

## Uninstalling

Run the uninstall script:

```bash
./uninstall.sh
```

---

## Commands

PacWhy provides the following commands:

* **Add**
  Add a package with name, description, and reason.

* **Remove**
  Remove a package entry by name.

* **Update**
  Update the name, description, or reason of a stored package.

* **List**
  List all packages stored in the PacWhy database.

* **Find**
  Search for a package by name, description, or install date.

---

## Usage

```
A System Package Organization Manager

Usage: PacWhy <COMMAND>

Commands:
  add, -a, --add        Add Package
  remove, -r, --remove  Remove Package
  update, -u, --update  Update Package
  list, -l, --list      List Package
  find, -f, --find      Find Package
  help                  Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

---
