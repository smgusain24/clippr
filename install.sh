#!/bin/bash
set -e

REPO="smgusain24/clippr"
APP_NAME="Clippr"

echo "Installing $APP_NAME..."

# Get latest release download URL for .dmg
DMG_URL=$(curl -sL "https://api.github.com/repos/$REPO/releases/latest" \
  | grep "browser_download_url.*\.dmg" \
  | head -1 \
  | cut -d '"' -f 4)

if [ -z "$DMG_URL" ]; then
  echo "Error: No .dmg found in latest release."
  exit 1
fi

TMP="/tmp/$APP_NAME.dmg"

echo "Downloading from $DMG_URL..."
curl -sL "$DMG_URL" -o "$TMP"

echo "Installing to /Applications..."
hdiutil attach "$TMP" -quiet
cp -R "/Volumes/$APP_NAME/$APP_NAME.app" /Applications/ 2>/dev/null \
  || cp -R /Volumes/$APP_NAME/*.app /Applications/
hdiutil detach "/Volumes/$APP_NAME" -quiet

rm -f "$TMP"

echo "$APP_NAME installed to /Applications. You can launch it from Spotlight."
