#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
INPUT_FILE="${SCRIPT_DIR}/input.css"
OUTPUT_FILE="${SCRIPT_DIR}/dist/generated-input.css"
CARGO_HOME_DIR="${CARGO_HOME:-/usr/local/cargo}"

mkdir -p "${SCRIPT_DIR}/dist"

lock_version="$(
  sed -n '/^name = "ssg_whiz"$/,/^$/p' "${SCRIPT_DIR}/../../Cargo.lock" \
    | awk -F'"' '/^version = / { print $2; exit }'
)"

if [[ -z "${lock_version}" ]]; then
  echo "Could not determine ssg_whiz version from Cargo.lock" >&2
  exit 1
fi

ssg_whiz_path="$(
  find "${CARGO_HOME_DIR}/registry/src" -maxdepth 2 -type d -name "ssg_whiz-${lock_version}" | head -n 1
)"

if [[ -z "${ssg_whiz_path}" ]]; then
  echo "Could not find ssg_whiz-${lock_version} under ${CARGO_HOME_DIR}/registry/src" >&2
  echo "Run a cargo command that fetches dependencies first." >&2
  exit 1
fi

cat > "${OUTPUT_FILE}" <<EOF
@source "${ssg_whiz_path}/**/*.{rs,html}";

EOF

cat "${INPUT_FILE}" >> "${OUTPUT_FILE}"

echo "${OUTPUT_FILE}"
