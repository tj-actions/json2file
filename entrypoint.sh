#!/usr/bin/env bash
set -euo pipefail

if [[ -z "$INPUT_BIN_PATH" ]]; then
  LATEST_VERSION=$(curl -sL https://api.github.com/repos/tj-actions/json2file/releases/latest | jq -r .tag_name)

  # Download the latest version
  WINDOWS_TARGET=x86_64-pc-windows-gnu
  LINUX_TARGET=x86_64-unknown-linux-musl
  MACOS_TARGET=x86_64-apple-darwin
  ARCHIVE=zip
  TEMP_DIR=$(mktemp -d)

  if [[ $(uname -s) == "Linux" ]]; then
    TARGET=$LINUX_TARGET
    ARCHIVE=tar.gz
  elif [[ $(uname -s) == "Darwin" ]]; then
    TARGET=$MACOS_TARGET
  else
    TARGET=$WINDOWS_TARGET
  fi

  if [[ "$ARCHIVE" == "zip" ]]; then
    curl -sL https://github.com/tj-actions/json2file/releases/download/"$LATEST_VERSION"/json2file_"$LATEST_VERSION"_"$TARGET"."$ARCHIVE" -o "$TEMP_DIR"/json2file.zip
    unzip -q $TEMP_DIR/json2file.zip -d $TEMP_DIR
    chmod +x $TEMP_DIR/json2file
  else
    curl -sL https://github.com/tj-actions/json2file/releases/download/"$LATEST_VERSION"/json2file_"$LATEST_VERSION"_"$TARGET"."$ARCHIVE" -o "$TEMP_DIR"/json2file.tar.gz
    tar -xzf $TEMP_DIR/json2file.tar.gz -C $TEMP_DIR
    chmod +x $TEMP_DIR/json2file
  fi

  INPUT_BIN_PATH=$TEMP_DIR/json2file
fi

INPUT_OUTPUTS="$(echo "$INPUT_OUTPUTS" | jq -r @json)"
INPUT_KEYS="$(echo "$INPUT_KEYS" |  tr '\n' ' ' | xargs)"

echo "Generating output using $INPUT_BIN_PATH..."

$INPUT_BIN_PATH --keys="$INPUT_KEYS" --outputs="$INPUT_OUTPUTS" --directory="$INPUT_DIRECTORY" --extension="$INPUT_EXTENSION" && exit_status=$? || exit_status=$?

rm -f "$INPUT_BIN_PATH"

if [[ $exit_status -ne 0 ]]; then
  echo "::error::Error generating output files from JSON"
  exit 1;
fi
