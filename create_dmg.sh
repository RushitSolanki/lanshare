#!/bin/bash

# Script to create a DMG file for LanShare
# This script should be run from the project root directory

set -e

# Configuration
APP_NAME="lanshare"
VERSION="0.1.0"
ARCH="aarch64"
APP_PATH="src-tauri/target/release/bundle/macos/${APP_NAME}.app"
DMG_NAME="${APP_NAME}_${VERSION}_${ARCH}.dmg"
DMG_PATH="src-tauri/target/release/bundle/dmg/${DMG_NAME}"

# Check if the app exists
if [ ! -d "$APP_PATH" ]; then
    echo "Error: App not found at $APP_PATH"
    echo "Please run 'cargo tauri build' first"
    exit 1
fi

# Create the DMG directory if it doesn't exist
mkdir -p "$(dirname "$DMG_PATH")"

# Create the DMG using create-dmg
echo "Creating DMG file: $DMG_PATH"
create-dmg \
    --volname "${APP_NAME}_${VERSION}" \
    --window-pos 200 120 \
    --window-size 600 400 \
    --icon-size 100 \
    --icon "${APP_NAME}.app" 175 120 \
    --hide-extension "${APP_NAME}.app" \
    --app-drop-link 425 120 \
    --no-internet-enable \
    "$DMG_PATH" \
    "$APP_PATH"

echo "DMG created successfully: $DMG_PATH" 