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

cargo fetch --locked
generated_input="$(bash ./generate-tailwind-input.sh)"
./tailwindcss-extra-linux-x64 -i "${generated_input}" -o ./dist/tailwind.css
cargo run
