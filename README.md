# procon_rs

A CLI tool for creating C++ competitive programming projects.

## Overview

`procon_rs` helps competitive programmers quickly create C++ project structures with templates and CMake configuration.

## Installation

### From Source (Current)

```bash
git clone https://github.com/yourusername/procon_rs.git
cd procon_rs
cargo install --path .
```

### From crates.io (Future)

```bash
# This will be available once published to crates.io
cargo install procon_rs
```

## Commands

### `new` - Create a new project

```bash
procon_rs new <project-name> [options]
```

**Options:**

- `-t, --template <name>`: Template to use (default: "default")
- `-p, --path <path>`: Directory to create the project in

**Examples:**

```bash
procon_rs new abc300_a
procon_rs new codeforces_1234_b --template advanced
```

### `init` - Initialize existing directory

```bash
procon_rs init
```

### `config` - Manage settings

```bash
procon_rs config <key> [value]
```

## Quick Start

```bash
# Create a new project
procon_rs new abc300_a
cd abc300_a

# Your project is ready with:
# - main.cpp (template code)
# - CMakeLists.txt (build configuration)
# - .gitignore
```

## Templates

Templates can be placed in `~/.config/procon_rs/templates/`. Each template must include:

- `main.cpp` - Main C++ source file
- `CMakeLists.txt` - CMake build configuration

Example custom template:

```bash
mkdir -p ~/.config/procon_rs/templates/my-template
cd ~/.config/procon_rs/templates/my-template
# Create main.cpp and CMakeLists.txt files
