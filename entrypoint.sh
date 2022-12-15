#!/usr/bin/env bash
set -euo pipefail

INPUT_OUTPUTS="$(echo "$INPUT_OUTPUTS" | jq -r @json)"

if [[ -z "$INPUT_BIN_PATH" ]]; then
  ## TODO: use "curl -sf https://[github releases - latest]] | PREFIX=. sh"
  exit 1;
fi

echo "::debug::Generating output using $INPUT_BIN_PATH..."

IFS=$'\n' read -ra KEYS <<<"$INPUT_KEYS"

$INPUT_BIN_PATH --keys="${KEYS[*]}" --outputs="$INPUT_OUTPUTS" \
  --directory="$INPUT_DIRECTORY" --extension="$INPUT_EXTENSION" && exit_status=$? || exit_status=$?

rm -f "$INPUT_BIN_PATH"

if [[ $exit_status -ne 0 ]]; then
  echo "::error::Error generating output files from JSON"
  exit 1;
fi
