#!/bin/bash
# ──────────────────────────────────────────────────────────────────
#  export-wallpaper.sh
#  Renders index.html → PNG → sets as macOS desktop wallpaper
#  Usage:  ./export-wallpaper.sh [width] [height]
#  Default resolution: 3456×2234 (MacBook Pro 16" native)
# ──────────────────────────────────────────────────────────────────

DIR="$(cd "$(dirname "$0")" && pwd)"
OUTPUT="$DIR/wallpaper-export.png"
CHROME="/Applications/Google Chrome.app/Contents/MacOS/Google Chrome"

WIDTH="${1:-3456}"
HEIGHT="${2:-2234}"

echo "→ Rendering at ${WIDTH}×${HEIGHT}…"

"$CHROME" \
  --headless=new \
  --no-sandbox \
  --disable-gpu \
  --disable-extensions \
  --window-size="${WIDTH},${HEIGHT}" \
  --force-device-scale-factor=1 \
  --screenshot="$OUTPUT" \
  "file://${DIR}/index.html" 2>/dev/null

if [ ! -f "$OUTPUT" ]; then
  echo "✗ Screenshot failed. Is Google Chrome installed?"
  exit 1
fi

echo "→ Setting as desktop wallpaper…"

osascript <<APPLESCRIPT
  tell application "System Events"
    set picture of every desktop to "$OUTPUT"
  end tell
APPLESCRIPT

echo "✓ Done — wallpaper updated: $OUTPUT"
