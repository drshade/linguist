#!/usr/bin/env bash
set -euo pipefail

# Fetch the upstream samples matching the definitions snapshot recorded in
# definitions/UPSTREAM_COMMIT, so definitions and samples are always a pair.

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
DEST="$SCRIPT_DIR/samples"
SHA=$(cat "$SCRIPT_DIR/../definitions/UPSTREAM_COMMIT")
TMP=$(mktemp -d)

trap 'rm -rf "$TMP"' EXIT

echo "Fetching samples from github-linguist/linguist @ $SHA..."

git init -q "$TMP/linguist"
cd "$TMP/linguist"
git remote add origin https://github.com/github-linguist/linguist.git
git sparse-checkout set samples
git fetch -q --depth=1 --filter=blob:none origin "$SHA"
git checkout -q FETCH_HEAD

rm -rf "$DEST"
mv "$TMP/linguist/samples" "$DEST"

echo "Done — samples written to $DEST (upstream $SHA)"
