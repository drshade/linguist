# Agent notes for linguist

Rust library (+ optional CLI) for programming language detection, vendoring
definitions from upstream [github-linguist](https://github.com/github-linguist/linguist).
Process docs (syncing, releasing) live in [MAINTAINING.md](MAINTAINING.md).

**Contributing a change?** Read [CONTRIBUTING.md](CONTRIBUTING.md) first —
it determines whether your change even belongs in this repo.

## Building and testing

```bash
bash tests/pull-samples.sh   # REQUIRED once per clone, before cargo test
cargo test                   # library
cargo test --features cli    # library + binary
```

- **The samples suite silently self-skips if `tests/samples/` is missing**
  (it's gitignored). A green `cargo test` on a fresh clone proves nothing
  until you have run `tests/pull-samples.sh` — it fetches the fixtures
  matching the commit pinned in `definitions/UPSTREAM_COMMIT`.
- The binary is feature-gated: `cargo run --features cli -- <files>`.
  Plain `cargo build`/`cargo run` covers the library only.
- CI enforces `cargo fmt --check` and `cargo clippy -D warnings` in both
  feature configurations, plus a build on the MSRV declared as
  `rust-version` in Cargo.toml. Don't use syntax newer than that toolchain.

## Vendored files — do not hand-edit

`definitions/languages.yml`, `heuristics.yml`, `vendor.yml`, and
`UPSTREAM_COMMIT` are verbatim snapshots of a single upstream commit,
refreshed by `definitions/pull-new.sh` (run weekly by the sync workflow).
Hand edits will be silently overwritten by the next sync.

- A heuristic regex `fancy-regex` can't compile is fixed upstream, never
  patched locally (see MAINTAINING.md).
- Definitions and samples must come from the same upstream commit — always
  re-run `tests/pull-samples.sh` after `pull-new.sh`.

## Mutation responsibility stays with the human

Agents are welcome — encouraged — to write and maintain code here: edit
files, run tests, refactor, prepare everything. But actions that mutate
shared or published state belong to the human owning the work, because they
carry the accountability for them:

- `git commit`, `git push`, creating or deleting branches and tags
- merging/accepting PRs (including Dependabot and sync-workflow PRs)
- publishing (`cargo publish`) or anything else that leaves the machine

Work to a finished, verified state in the working tree, then report what's
ready and suggest commit boundaries — and stop there. If you're assisting a
contributor, the same rule applies in their fork: your human reviews, owns
the commits, and opens the PR. Exception: the repository's own automation
(the sync workflow opening its weekly PR) is sanctioned; a human still
merges it.

## Workspace and releases

Two independently versioned crates: `linguist` (root) and `linguist-types/`.
Merging a version bump to main *is* a release: once CI passes, the
Auto-release workflow tags `vX.Y.Z` and the Release workflow publishes to
crates.io and creates the GitHub Release. The weekly sync PR already
includes the patch version bump. Never bump versions or tag on your own
initiative — the maintainer decides releases. `main` rejects direct pushes:
every change, the maintainer's included, lands through a PR. See
MAINTAINING.md.
