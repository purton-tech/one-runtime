#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
INPUT_FILE="${SCRIPT_DIR}/input.css"
OUTPUT_FILE="${SCRIPT_DIR}/dist/generated-input.css"

mkdir -p "${SCRIPT_DIR}/dist"

lock_version="$(
  sed -n '/^name = "ssg_whiz"$/,/^$/p' "${SCRIPT_DIR}/../../Cargo.lock" \
    | awk -F'"' '/^version = / { print $2; exit }'
)"

if [[ -z "${lock_version}" ]]; then
  echo "Could not determine ssg_whiz version from Cargo.lock" >&2
  exit 1
fi

search_roots=()
if [[ -n "${CARGO_HOME:-}" ]]; then
  search_roots+=("${CARGO_HOME}")
fi
if [[ -n "${HOME:-}" ]]; then
  search_roots+=("${HOME}/.cargo")
fi
search_roots+=("/usr/local/cargo")

ssg_whiz_path=""
for root in "${search_roots[@]}"; do
  registry_src="${root}/registry/src"
  if [[ ! -d "${registry_src}" ]]; then
    continue
  fi

  ssg_whiz_path="$(
    find "${registry_src}" -maxdepth 2 -type d -name "ssg_whiz-${lock_version}" 2>/dev/null | head -n 1
  )"

  if [[ -n "${ssg_whiz_path}" ]]; then
    break
  fi
done

if [[ -z "${ssg_whiz_path}" ]]; then
  echo "Could not find ssg_whiz-${lock_version} in any Cargo registry cache." >&2
  echo "Checked roots: ${search_roots[*]}" >&2
  echo "Run 'cargo fetch --locked' first." >&2
  exit 1
fi

cat > "${OUTPUT_FILE}" <<EOF
@source "${ssg_whiz_path}/**/*.{rs,html}";

EOF

cat "${INPUT_FILE}" >> "${OUTPUT_FILE}"

echo "${OUTPUT_FILE}"
