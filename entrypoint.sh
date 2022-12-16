#!/usr/bin/env bash
set -exuo pipefail

INPUT_OUTPUTS="$(echo "$INPUT_OUTPUTS" | jq -r @json)"
INPUT_KEYS="$(echo "$INPUT_KEYS" |  tr '\n' ' ' | xargs)"

if [[ -z "$INPUT_BIN_PATH" ]]; then
  ## TODO: use "curl -sf https://[github releases - latest]] | PREFIX=. sh"
  exit 1;
fi

echo "Generating output using $INPUT_BIN_PATH..."

$INPUT_BIN_PATH --keys="$INPUT_KEYS" --outputs="$INPUT_OUTPUTS" \
  --directory="$INPUT_DIRECTORY" --extension="$INPUT_EXTENSION" && exit_status=$? || exit_status=$?

rm -f "$INPUT_BIN_PATH"

if [[ $exit_status -ne 0 ]]; then
  echo "::error::Error generating output files from JSON"
  exit 1;
fi
