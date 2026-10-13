#!/bin/bash
set -e

APP_NAME="Smart Password Manager"
APP_ID="smart-password-manager"
BIN_NAME="smart-password-manager"
REPO_URL="https://github.com/smartlegionlab/smart-password-manager-gui-rs.git"
BIN_DIR="$HOME/.local/bin"
APPS_DIR="$HOME/.local/share/applications"
MENU_ENTRY="$APPS_DIR/$APP_ID.desktop"

GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
BLUE='\033[0;34m'
BOLD='\033[1m'
NC='\033[0m'

info()  { echo -e "${GREEN}==>${NC} $*"; }
warn()  { echo -e "${YELLOW}==>${NC} $*"; }
error() { echo -e "${RED}==>${NC} $*" >&2; }
step()  { echo -e "${BOLD}${BLUE}[*]${NC} $*"; }

banner() {
    echo ""
    echo -e "${BOLD}${GREEN}==============================================${NC}"
    echo -e "${BOLD}${GREEN}  Smart Password Manager - Installer${NC}"
    echo -e "${BOLD}${GREEN}==============================================${NC}"
    echo ""
}

banner

echo "This installer will:"
echo "  1. Download the application source code"
echo "  2. Build it with cargo (requires Rust toolchain)"
echo "  3. Install the binary to:  $BIN_DIR/$BIN_NAME"
echo "  4. Register the app in your application menu"
echo ""
echo "Your password metadata will be stored in:"
echo "  ~/.config/smart_password_manager/passwords.json"
echo ""

step "Checking cargo..."
if ! command -v cargo >/dev/null 2>&1; then
    error "cargo is required but not installed."
    error "Install Rust from https://rustup.rs and try again."
    exit 1
fi
info "Found: $(cargo --version)"

step "Checking git..."
if ! command -v git >/dev/null 2>&1; then
    error "git is required but not installed."
    exit 1
fi
info "git found."

step "Cloning repository..."
TMP_CLONE="$(mktemp -d)"
git clone --depth 1 "$REPO_URL" "$TMP_CLONE" >/dev/null 2>&1
info "Repository cloned."

step "Building release binary (this may take several minutes)..."
pushd "$TMP_CLONE" >/dev/null
cargo build --release >/dev/null 2>&1
popd >/dev/null
info "Build finished."

step "Installing binary..."
mkdir -p "$BIN_DIR"
install -m 755 "$TMP_CLONE/target/release/$BIN_NAME" "$BIN_DIR/$BIN_NAME"
info "Binary installed: $BIN_DIR/$BIN_NAME"

step "Registering application in the menu..."
mkdir -p "$APPS_DIR"

DESKTOP_CONTENT="[Desktop Entry]
Version=1.0
Type=Application
Name=$APP_NAME
Comment=GUI for smart password management
Exec=$BIN_DIR/$BIN_NAME
Icon=utilities-terminal
Terminal=false
Categories=Utility;Security;
StartupNotify=true
Keywords=password;manager;security;gui;smartpasslib;
"

echo "$DESKTOP_CONTENT" > "$MENU_ENTRY"
chmod +x "$MENU_ENTRY"
info "Menu entry created: $MENU_ENTRY"

rm -rf "$TMP_CLONE"
info "Temporary clone removed."

if command -v update-desktop-database >/dev/null 2>&1; then
    update-desktop-database "$APPS_DIR" >/dev/null 2>&1 || true
fi

echo ""
echo -e "${BOLD}${GREEN}  Installation complete${NC}"
echo ""
echo "  Binary:      $BIN_DIR/$BIN_NAME"
echo "  Menu entry:  $MENU_ENTRY"
echo "  Metadata:    ~/.config/smart_password_manager/passwords.json"
echo ""
echo "  Launch from terminal:"
echo "    $BIN_NAME"
echo ""
echo "  Or from the application menu: \"$APP_NAME\""
echo ""
echo "  Uninstall:"
echo "    curl -fsSL https://raw.githubusercontent.com/smartlegionlab/smart-password-manager-gui-rs/master/uninstall.sh | bash"
echo ""
