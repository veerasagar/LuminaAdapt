#!/usr/bin/env bash
set -euo pipefail

# 1. Build the Rust project (optional if you just want cargo run to rebuild)
cargo build

# 2. Run the Rust server in the background
cargo run &
SERVER_PID=$!

# 3. Give the server a moment to start up
sleep 2

# 4. URL where your index.html is served
URL="index.html"

# 5. Open the URL in the default browser
if command -v xdg-open >/dev/null 2>&1; then
  xdg-open "$URL"
elif command -v open >/dev/null 2>&1; then
  open "$URL"
else
  echo "Please open your browser and visit: $URL"
fi

# 6. When the script is interrupted (Ctrl+C), kill the Rust server
trap 'echo "Shutting down server…"; kill $SERVER_PID 2>/dev/null' INT TERM

# 7. Wait for the server process to exit
wait $SERVER_PID
