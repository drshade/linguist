#!/usr/bin/env bash
set -euo pipefail

# Fetch the upstream github-linguist definitions as a consistent snapshot of a
# single upstream commit. That commit's SHA is recorded in UPSTREAM_COMMIT so
# tests/pull-samples.sh can fetch the matching samples — definitions and
# samples must always come from the same upstream commit (see MAINTAINING.md).

cd "$(dirname "$0")"

UPSTREAM=https://github.com/github-linguist/linguist
SHA=$(git ls-remote "$UPSTREAM.git" HEAD | cut -f1)
echo "Upstream github-linguist HEAD: $SHA"

RAW="https://raw.githubusercontent.com/github-linguist/linguist/$SHA/lib/linguist"
curl -fsSL "$RAW/languages.yml" -o languages.yml
curl -fsSL "$RAW/heuristics.yml" -o heuristics.yml
curl -fsSL "$RAW/vendor.yml" -o vendor.yml

echo "$SHA" > UPSTREAM_COMMIT
