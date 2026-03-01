#!/usr/bin/env bash
set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
DEST="$SCRIPT_DIR/samples"
TMP=$(mktemp -d)

trap 'rm -rf "$TMP"' EXIT

echo "Fetching samples from github-linguist/linguist..."

git clone --depth=1 --filter=blob:none --sparse \
    https://github.com/github-linguist/linguist.git "$TMP/linguist"

cd "$TMP/linguist"
git sparse-checkout set samples

rm -rf "$DEST"
mv "$TMP/linguist/samples" "$DEST"

echo "Done — samples written to $DEST"
