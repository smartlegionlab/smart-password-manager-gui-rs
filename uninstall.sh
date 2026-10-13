#!/bin/bash
set -e

APP_ID="smart-password-manager"
BIN_NAME="smart-password-manager"
BIN_DIR="$HOME/.local/bin"
APPS_DIR="$HOME/.local/share/applications"
MENU_ENTRY="$APPS_DIR/$APP_ID.desktop"
DATA_DIR="$HOME/.config/smart_password_manager"

GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
BOLD='\033[1m'
NC='\033[0m'

info()  { echo -e "${GREEN}==>${NC} $*"; }
warn()  { echo -e "${YELLOW}==>${NC} $*"; }

banner() {
    echo ""
    echo -e "${BOLD}${RED}==============================================${NC}"
    echo -e "${BOLD}${RED}  Smart Password Manager - Uninstaller${NC}"
    echo -e "${BOLD}${RED}==============================================${NC}"
    echo ""
}

banner

echo "Will remove:"
echo "  - Binary:      $BIN_DIR/$BIN_NAME"
echo "  - Menu entry:  $MENU_ENTRY"
echo ""
echo "Your password metadata is NOT touched."
echo "  $DATA_DIR/passwords.json"
echo ""

if [ -f "$BIN_DIR/$BIN_NAME" ]; then
    rm -f "$BIN_DIR/$BIN_NAME"
    info "Removed: $BIN_DIR/$BIN_NAME"
else
    warn "Not found: $BIN_DIR/$BIN_NAME"
fi

if [ -f "$MENU_ENTRY" ]; then
    rm -f "$MENU_ENTRY"
    info "Removed: $MENU_ENTRY"
else
    warn "Not found: $MENU_ENTRY"
fi

if command -v update-desktop-database >/dev/null 2>&1; then
    update-desktop-database "$APPS_DIR" >/dev/null 2>&1 || true
fi

echo ""
echo -e "${BOLD}${GREEN}  Uninstall complete${NC}"
echo ""
echo "  Kept: $DATA_DIR/passwords.json"
echo "  To delete your metadata too:"
echo "    rm -rf $DATA_DIR"
echo ""
