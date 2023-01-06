#!/usr/bin/env bash
set -euo pipefail

echo "Parsing inputs..."
INPUT_OUTPUTS="$(echo "$INPUT_OUTPUTS" | jq -r @json)"
INPUT_KEYS="$(echo "$INPUT_KEYS" |  tr '\n' ' ' | xargs)"
EXTRA_ARGS=""

if [[ "$INPUT_SKIP_MISSING_KEYS" == "true" ]]; then
  EXTRA_ARGS="$EXTRA_ARGS --skip-missing-keys"
fi

echo "Generating output using $INPUT_BIN_PATH..."

# shellcheck disable=SC2086
json2file $EXTRA_ARGS --keys="$INPUT_KEYS" --outputs="$INPUT_OUTPUTS" --directory="$INPUT_DIRECTORY" --extension="$INPUT_EXTENSION" && exit_status=$? || exit_status=$?

if [[ $exit_status -ne 0 ]]; then
  echo "::error::Error generating output files from JSON"
  exit 1;
fi
