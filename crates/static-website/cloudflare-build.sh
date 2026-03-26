#!/usr/bin/env bash
set -euo pipefail

if ! command -v cargo >/dev/null 2>&1; then
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
  . "$HOME/.cargo/env"
fi

if [[ ! -x ./tailwindcss-extra-linux-x64 ]]; then
  curl -OL https://github.com/dobicinaitis/tailwind-cli-extra/releases/latest/download/tailwindcss-extra-linux-x64
  chmod +x ./tailwindcss-extra-linux-x64
fi

TIMESTAMP=$(date +%s)
export TAILWIND_STYLESHEET="/tailwind-${TIMESTAMP}.css"

cargo fetch --locked
../../scripts/tailwind-crates --input ./input.css --output "./dist/tailwind-${TIMESTAMP}.css" --tailwind-bin ./tailwindcss-extra-linux-x64
cargo run
