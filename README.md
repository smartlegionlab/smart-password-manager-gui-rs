# Smart Password Manager (Rust, GUI)

> **⚠️ Experimental implementation.** This is an experimental Rust GUI built on `egui`. It is functional, but the interface and behaviour may change. For production use on Windows or Linux desktop, please prefer the mature versions from the Smart Password ecosystem:
>
> - **[Desktop Smart Password Manager (Python)](https://github.com/smartlegionlab/smart-password-manager-desktop)** — full-featured desktop GUI, stable
> - **[Desktop Smart Password Manager (C#)](https://github.com/smartlegionlab/SmartPasswordManagerCsharpDesktop)** — Windows-native desktop GUI, stable
>
> This GUI is primarily a demonstration that the `smartpasslib` algorithm works consistently across all interfaces, including a Rust desktop window. Use it if you want to experiment, not as your daily driver.

**A cross-platform desktop GUI for smart password management built on [smartpasslib](https://github.com/smartlegionlab/smartpasslib-rs). Generate deterministic passwords from secret phrases without storing them. No cloud, no database, no trust required.**

## Overview

Smart Password Manager is a self-contained desktop application for managing smart passwords — passwords that are regenerated on demand from a secret phrase instead of being stored. It is built on top of the [smartpasslib](https://crates.io/crates/smartpasslib) library and uses the same cryptographic algorithm as the Python, JavaScript, Kotlin, Go and C# implementations.

The application stores only public verification keys, descriptions and lengths. Your secret phrase and the actual passwords are never written to disk. Passwords are recalculated whenever you need them, using the same deterministic algorithm.

**Decentralized by design:** no cloud, no database, no third-party trust required. Your secrets never leave your device.

---

[![Crates.io](https://img.shields.io/crates/v/smart-password-manager)](https://crates.io/crates/smart-password-manager)
[![Documentation](https://docs.rs/smart-password-manager/badge.svg)](https://docs.rs/smart-password-manager)
[![GitHub top language](https://img.shields.io/github/languages/top/smartlegionlab/smart-password-manager-gui-rs)](https://github.com/smartlegionlab/smart-password-manager-gui-rs)
[![GitHub license](https://img.shields.io/github/license/smartlegionlab/smart-password-manager-gui-rs)](https://github.com/smartlegionlab/smart-password-manager-gui-rs/blob/master/LICENSE)
[![GitHub release](https://img.shields.io/github/v/release/smartlegionlab/smart-password-manager-gui-rs)](https://github.com/smartlegionlab/smart-password-manager-gui-rs/)
[![GitHub stars](https://img.shields.io/github/stars/smartlegionlab/smart-password-manager-gui-rs?style=social)](https://github.com/smartlegionlab/smart-password-manager-gui-rs/stargazers)
[![GitHub forks](https://img.shields.io/github/forks/smartlegionlab/smart-password-manager-gui-rs?style=social)](https://github.com/smartlegionlab/smart-password-manager-gui-rs/network/members)
![Platform](https://img.shields.io/badge/platform-linux%20%7C%20windows-lightgrey)
[![Crates.io Downloads](https://img.shields.io/crates/d/smart-password-manager)](https://crates.io/crates/smart-password-manager)

---

## Core Principles

- **Zero-Storage Security**: No passwords or secret phrases are ever stored or transmitted
- **Decentralized Architecture**: No central servers, no cloud dependency, no third-party trust required
- **Deterministic Regeneration**: Passwords are recreated identically from your secret phrase
- **Metadata Only**: Only public keys, descriptions and lengths are stored
- **Local Processing**: All cryptographic operations happen on your device
- **On-Demand Discovery**: Passwords exist only when you generate them

**What you can do:**

1. **Add entries**: Register a service description and a secret phrase, choose a length, and store only the public verification key
2. **Get passwords**: Enter the secret phrase for an entry and regenerate the password on demand
3. **Verify secrets**: Prove knowledge of a secret phrase without exposing it or generating a password
4. **Edit metadata**: Change the description or the length at any time
5. **Delete entries**: Remove metadata when you no longer need it
6. **Copy to clipboard**: One click copies a generated password to the system clipboard
7. **Search**: Filter entries by description or public key as you type

## Key Features

- **Decentralized & Serverless**: No central database, no cloud lock-in, complete user sovereignty
- **No Password Database**: Eliminates password storage completely
- **Deterministic Password Generation**: Same secret + same length = same password, always, on every platform
- **Public Key Verification**: Verify secret knowledge without exposure
- **Dark Theme Interface**: Clean, modern dark UI built with `egui`
- **Full CRUD**: Create, read, update, delete entries from a desktop window
- **Live Search**: Filter entries by description or public key in real time
- **Secure Input**: Secret phrase hidden by default with Show / Hide toggle
- **Live Validation**: Buttons enable only when the form is valid; counters for description and secret length
- **Copy to Clipboard**: System clipboard on X11, Wayland and Windows; OSC 52 fallback for compatible terminals
- **Keyboard Shortcuts**: Full keyboard navigation and hotkeys
- **About / Help / Shortcuts Dialogs**: Built-in documentation and reference
- **Cross-Platform Data**: Same `passwords.json` format as the Python, JavaScript, Kotlin, Go and C# implementations
- **No Network Access**: Runs entirely offline, no runtime dependencies beyond the OS graphics stack

---

## Screenshots

![Main view](https://raw.githubusercontent.com/smartlegionlab/smart-password-manager-gui-rs/master/data/images/smart-password-manager-gui-rs.png)

---

## Key Bindings

| Key             | Action                                    |
|-----------------|-------------------------------------------|
| `Ctrl+N`        | Add a new entry                           |
| `Ctrl+G`        | Get password for selected entry           |
| `Ctrl+E`        | Edit selected entry (description, length) |
| `Ctrl+R`        | Verify secret against selected entry      |
| `Del`           | Delete selected entry                     |
| `Enter`         | Get password for selected entry           |
| `Up` / `k`      | Move selection up                         |
| `Down` / `j`    | Move selection down                       |
| `F5`            | Refresh list from disk                    |
| `F1`            | Open Help                                 |
| `Ctrl+/`        | Open keyboard shortcuts reference         |
| `Ctrl+Q`        | Quit                                      |
| `Esc`           | Close any open dialog                     |

Note: shortcuts are ignored while a text input has focus, so typing in the search box works normally.

---

## Copy Behavior

When you click **Copy to clipboard** in the password dialog, or press **Ctrl+C** while the password is shown, the application tries two methods in order:

1. **System clipboard** via the `arboard` crate. Works on X11, Wayland and Windows sessions.
2. **OSC 52** escape sequence written to stdout. Works in most modern terminals (`kitty`, `alacritty`, `wezterm`, `iTerm2`, `Windows Terminal`, `GNOME Terminal`, `Konsole`, `tmux` with `set-clipboard on`) and also over SSH.

If neither method is available, the status bar reports `No clipboard available`, and the password can be selected manually from the password field.

---

## Command Line

```
smart-password-manager --help
smart-password-manager --version
```

With no arguments the application opens its main window.

---

## Quick Usage Guide

### Creating Your First Entry

1. Click **+ Add** (or press `Ctrl+N`)
2. Enter a service description (e.g., `GitHub Account`)
3. Enter your secret phrase (minimum 12 characters, never stored)
4. Set password length (12–100 characters, 16–24 recommended)
5. Click **Create** — only the public verification key is stored

### Retrieving a Password

1. Select an entry in the table (click a row, or use `↑` / `↓`)
2. Click **Get** or press `Ctrl+G` / `Enter`
3. Enter your secret phrase
4. The password regenerates — click **Copy to clipboard** to use it

### Editing Metadata

1. Select an entry and click **Edit** (or press `Ctrl+E`)
2. Update the description or length
3. Click **Save**

### Deleting an Entry

1. Select an entry and click **Delete** (or press `Del`)
2. Confirm — only metadata is removed

### Verifying a Secret

1. Select an entry and click **Verify** (or press `Ctrl+R`)
2. Enter your secret phrase
3. The application reports whether the secret matches the stored public key — without generating or displaying the password

### Searching

Type in the **Search** box at the top. Entries are filtered by description or public key as you type. Click **Clear** to reset.

---

## Data Storage

Only public keys, descriptions and lengths are stored, as JSON. The file location follows the standard user data directory convention for your operating system.

The file location is determined by `smartpasslib::SmartPasswordManager`, the same as in the Python, JavaScript, Kotlin, Go and C# implementations:

| Platform | Path                                                          |
|----------|---------------------------------------------------------------|
| Linux    | `~/.config/smart_password_manager/passwords.json`             |
| Windows  | `%USERPROFILE%\.config\smart_password_manager\passwords.json` |

Format:

```json
{
  "a3f8c1e0b9d2f4a6c8e0b1d3f5a7c9e1b2d4f6a8c0e2b4d6f8a0c2e4b6d8f0a1": {
    "public_key": "a3f8c1e0b9d2f4a6c8e0b1d3f5a7c9e1b2d4f6a8c0e2b4d6f8a0c2e4b6d8f0a1",
    "description": "GitHub Account",
    "length": 24
  }
}
```

**What is NOT stored:**

- Your secret phrase
- The actual password
- Any reversible password data

**What IS stored:**

- Public verification key (a hash derived from the secret)
- Service description
- Password length

The file is created automatically on first save. If it does not exist at startup, the application begins with an empty list.

### Data Compatibility

- Uses the same `~/.config/smart_password_manager/passwords.json` format as the Python, JavaScript, Kotlin, Go and C# implementations
- The same `smartpasslib` algorithm is used everywhere, so metadata can be synced through any channel without conversion
- Consistent cryptographic operations across all ecosystem tools

---

## Security Model

- **Proof of knowledge**: Public keys verify secrets without exposing them
- **Decentralized trust**: No third party needed — you control your secrets completely
- **Deterministic security**: Same secret + same length = same password, always
- **Metadata separation**: Non-sensitive data stored separately from verification
- **Local processing**: No data leaves your computer
- **No recovery backdoors**: Lost secret phrase = permanently lost access, by design

### Decentralized Nature

**There is no "forgot password" button.** This is by design:

- No central server can reset your passwords
- No support team can recover your access
- Your secret phrase is the ONLY key

**This is the price of true decentralization** — you are completely in control.

---

## Security Warnings

### Secret Phrase Security

**Your secret phrase is the cryptographic master key**

1. **Permanent data loss**: Lost secret phrase = irreversible loss of all derived passwords
2. **No recovery mechanisms**: No password recovery, no secret reset, no administrative override
3. **Deterministic generation**: Identical input (secret + length) = identical output (password)
4. **Single point of failure**: Secret phrase is the sole authentication factor for all passwords
5. **Secure storage required**: Digital storage of secret phrases is prohibited

**Critical**: Test password regeneration with non-essential accounts before production use.

### Secret Phrase Requirements

- **Minimum 12 characters** (enforced by the application)
- Case-sensitive — `MySecret` and `mysecret` are different
- Use a mix of: uppercase, lowercase, numbers, symbols, emoji, or Cyrillic
- **Never store digitally**
- **NEVER use your password description as your secret phrase**

### Strong Secret Examples

```
✅ "MyStrongSecretPhrase2026!"   — mixed case + numbers + symbols
✅ "P@ssw0rd!LongSecret"         — special chars + numbers + length
✅ "КотБегемот2026НаДиете"       — Cyrillic + numbers
✅ "Correct-Horse-Battery-42"    — memorable phrase with separator
```

### Weak Secret Examples (avoid)

```
❌ "short"                       — too short, rejected
❌ "GitHub Account"              — using description as secret (weak!)
❌ "password"                    — dictionary word, too short
❌ "1234567890"                  — only digits, too short
❌ "qwerty123"                   — keyboard pattern
❌ Same as description           — never use the same value as description
```

### Password Length Requirements

- **Smart passwords**: 12–100 characters

---

## Technical Foundation

Powered by [**smartpasslib**](https://github.com/smartlegionlab/smartpasslib-rs) — the core library for deterministic password generation.

**Key derivation:**

| Key Type    | Iterations      | Purpose                                                |
|-------------|-----------------|--------------------------------------------------------|
| Private Key | 15–30 (dynamic) | Password generation (never stored, never transmitted)  |
| Public Key  | 45–60 (dynamic) | Verification (stored locally)                          |

**Character set (Google-compatible):**

```
!@#$%^&*()_+-=[]{};:,.<>?/ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789abcdefghijklmnopqrstuvwxyz
```

**Validation rules:**

- Secret phrase: minimum 12 characters
- Password length: 12–100 characters

**Decentralized architecture:**

- No central authority required
- Metadata can be synced via any channel
- Your security depends only on your secret phrase
- Works offline — no internet connection required

---

## Cross-Platform Implementations

The same deterministic algorithm is available in multiple languages. Smart Password Manager produces identical passwords to:

| Language   | Repository                                                                   |
|------------|------------------------------------------------------------------------------|
| Python     | [smartpasslib](https://github.com/smartlegionlab/smartpasslib)               |
| JavaScript | [smartpasslib-js](https://github.com/smartlegionlab/smartpasslib-js)         |
| Kotlin     | [smartpasslib-kotlin](https://github.com/smartlegionlab/smartpasslib-kotlin) |
| Go         | [smartpasslib-go](https://github.com/smartlegionlab/smartpasslib-go)         |
| C#         | [smartpasslib-csharp](https://github.com/smartlegionlab/smartpasslib-csharp) |
| **Rust**   | [smartpasslib-rs](https://github.com/smartlegionlab/smartpasslib-rs)         |

**Data transfer:** Metadata can be moved between platforms through the shared `passwords.json` format.

---

## Research Paradigms & Publications

- **[Pointer-Based Security Paradigm](https://doi.org/10.5281/zenodo.17204738)** — Architectural Shift from Data Protection to Data Non-Existence
- **[Local Data Regeneration Paradigm](https://doi.org/10.5281/zenodo.17264327)** — Ontological Shift from Data Transmission to Synchronous State Discovery

---

## Build from Source

Requirements:

- Rust 1.88 or newer (edition 2024)
- Cargo
- On Linux: X11 / Wayland / OpenGL development packages

### Linux (Debian / Ubuntu)

```bash
sudo apt-get install -y \
  libx11-dev \
  libxcursor-dev \
  libxrandr-dev \
  libxi-dev \
  libgl1-mesa-dev \
  libwayland-dev \
  libxkbcommon-dev
```

### Linux (Arch)

```bash
sudo pacman -S base-devel libx11 libxcursor libxrandr libxi mesa wayland libxkbcommon
```

### Windows

No additional packages needed.

### Build

```bash
git clone https://github.com/smartlegionlab/smart-password-manager-gui-rs.git
cd smart-password-manager-gui-rs
cargo build --release
./target/release/smart-password-manager
```

For development:

```bash
cargo run
```

---

## Installation

### Option 1: `cargo install`

The simplest way, if you have Rust installed. Builds the binary for your system and installs it to `~/.cargo/bin/`.

```bash
cargo install smart-password-manager
```

Then run:

```bash
smart-password-manager
```

On Linux you still need the runtime graphics libraries (X11 / Wayland / OpenGL) — they are present on any desktop distribution.

### Option 2: Prebuilt binaries

Download the binary for your platform from the GitHub Releases page:

```
https://github.com/smartlegionlab/smart-password-manager-gui-rs/releases
```

| Platform       | Filename                                              |
|----------------|-------------------------------------------------------|
| Linux x86_64   | `smart-password-manager-linux-x86_64-<version>`       |
| Windows x86_64 | `smart-password-manager-windows-x86_64-<version>.exe` |

**Linux:**

```bash
chmod +x smart-password-manager-linux-x86_64-<version>
./smart-password-manager-linux-x86_64-<version>
```

**Windows:** double-click the `.exe` file.

### Option 3: One-command install (Linux)

```bash
curl -fsSL https://raw.githubusercontent.com/smartlegionlab/smart-password-manager-gui-rs/master/install.sh | bash
```

Installs the binary to `~/.local/bin/` and registers the application in your desktop menu.

### Uninstall

If you installed via `cargo install`:

```bash
cargo uninstall smart-password-manager
```

If you installed via `install.sh`:

```bash
curl -fsSL https://raw.githubusercontent.com/smartlegionlab/smart-password-manager-gui-rs/master/uninstall.sh | bash
```

Your password metadata in `~/.config/smart_password_manager/passwords.json` is never touched by uninstallers. Delete it manually if you want to remove it:

```bash
rm -rf ~/.config/smart_password_manager
```

---

## Ecosystem

**Core Libraries:**

- **[smartpasslib](https://github.com/smartlegionlab/smartpasslib)** — Python
- **[smartpasslib-js](https://github.com/smartlegionlab/smartpasslib-js)** — JavaScript
- **[smartpasslib-kotlin](https://github.com/smartlegionlab/smartpasslib-kotlin)** — Kotlin
- **[smartpasslib-go](https://github.com/smartlegionlab/smartpasslib-go)** — Go
- **[smartpasslib-csharp](https://github.com/smartlegionlab/smartpasslib-csharp)** — C#
- **[smartpasslib-rs](https://github.com/smartlegionlab/smartpasslib-rs)** — Rust

**CLI Applications:**

- **[CLI Smart Password Manager (Python)](https://github.com/smartlegionlab/clipassman)**
- **[CLI Smart Password Generator (Python)](https://github.com/smartlegionlab/clipassgen)**
- **[CLI Smart Password Manager (C#)](https://github.com/smartlegionlab/SmartPasswordManagerCsharpCli)**
- **[CLI Smart Password Generator (C#)](https://github.com/smartlegionlab/SmartPasswordGeneratorCsharpCli)**
- **[CLI Smart Password Manager (Rust)](https://github.com/smartlegionlab/smart-password-manager-cli-rs)**

**Desktop Applications:**

- **[Desktop Smart Password Manager (Python)](https://github.com/smartlegionlab/smart-password-manager-desktop)**
- **[Desktop Smart Password Manager (C#)](https://github.com/smartlegionlab/SmartPasswordManagerCsharpDesktop)**
- **Desktop Smart Password Manager (Rust, GUI)** — this project
- **[Desktop Smart Password Manager (Rust, TUI)](https://github.com/smartlegionlab/smart-password-manager-tui-rs)**

**Other:**

- **[Web Smart Password Manager](https://github.com/smartlegionlab/smart-password-manager-web)**
- **[Android Smart Password Manager](https://github.com/smartlegionlab/smart-password-manager-android)**

---

## Project Layout

```
smart-password-manager-gui-rs/
├── .github/
│   └── workflows/
│       ├── publish.yml
│       └── release.yml
├── src/
│   ├── main.rs        Entry point, window setup, CLI parsing
│   ├── app.rs         Application state, dialog states, business logic
│   ├── theme.rs       Color palette
│   ├── style.rs       egui visuals, button and frame factories
│   ├── clipboard.rs   System clipboard + OSC 52 fallback
│   └── ui/
│       ├── mod.rs          Render dispatcher
│       ├── main_window.rs  Menu, header, toolbar, search, table, info, status
│       ├── dialogs.rs      Add, Get, Edit, Verify, Delete, About, Help, Shortcuts
│       └── message.rs      Info and Error popups
├── .gitignore
├── Cargo.toml
├── Cargo.lock
├── LICENSE
├── README.md
├── install.sh
└── uninstall.sh
```

---

## License

**[BSD 3-Clause License](LICENSE)**

Copyright (©) 2026, [Alexander Suvorov](https://github.com/smartlegionlab)

---

## Author

**Alexander Suvorov**

- GitHub: [smartlegionlab](https://github.com/smartlegionlab)
- Website: [smartlegionlab.github.io](https://smartlegionlab.github.io)

---

## Support

- **GUI Manager Issues**: [GitHub Issues](https://github.com/smartlegionlab/smart-password-manager-gui-rs/issues)
- **Core Library Issues**: [smartpasslib Issues](https://github.com/smartlegionlab/smartpasslib-rs/issues)
- **Documentation**: Inline Help (`F1`), Shortcuts (`Ctrl+/`) and this README

**Note**: Always test password regeneration with non-essential accounts first. Implementation security depends on proper usage.

---

## Links

- Repository: https://github.com/smartlegionlab/smart-password-manager-gui-rs
- Website: https://smartlegionlab.github.io
- Core library: https://github.com/smartlegionlab/smartpasslib-rs

---

## ⚠️ Disclaimer

**By using this software, you agree to the full disclaimer terms.**

**Summary:** Software provided "AS IS" without warranty. You assume all risks.

**Full legal disclaimer:** See [DISCLAIMER.md](https://github.com/smartlegionlab/smart-password-manager-gui-rs/blob/master/DISCLAIMER.md)

