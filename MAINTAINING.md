# Maintaining

How to keep this crate in sync with upstream [github-linguist](https://github.com/github-linguist/linguist), and how releases work.

## Upstream data sources

This crate vendors two sets of files from upstream, pinned to a single upstream commit:

| What | Location | Pulled by |
|------|----------|-----------|
| Language/heuristic/vendor definitions | `definitions/*.yml` | `definitions/pull-new.sh` |
| Detection test fixtures | `tests/samples/` | `tests/pull-samples.sh` |

**They are always a matched pair.** `pull-new.sh` snapshots upstream HEAD and records its commit SHA in `definitions/UPSTREAM_COMMIT`; `pull-samples.sh` fetches the samples from exactly that commit. This keeps the samples test deterministic: the samples always correspond to the committed definitions, no matter when they are fetched. (Upstream changes language names, sample categorisation, and definitions in the same commits, so mixing snapshots causes spurious failures.)

`tests/samples/` is gitignored — it is re-fetched locally and never committed; run `tests/pull-samples.sh` after cloning to get the samples matching the committed `UPSTREAM_COMMIT`.

## Updating definitions

Normally automated: the **Sync upstream definitions** workflow (`.github/workflows/sync-upstream.yml`) runs weekly (and on manual dispatch), pulls both sources, runs the tests, and opens a PR if the definitions changed. When tests pass, the PR also includes a patch version bump, so merging it leaves main release-ready — publishing is then just tagging (the PR body contains the exact commands). GitHub does not fire `pull_request` workflows for PRs opened with the built-in `GITHUB_TOKEN`, so the sync workflow explicitly dispatches `ci.yml` on its branch afterwards (`workflow_dispatch` is exempt from that rule); the checks then appear on the PR as usual. If they are ever missing, close/reopen the PR or push to its branch.

Dependabot PRs (dependency bumps) carry no version bump — merge them to main and they ride along in whatever release comes next.

Manually, it is:

```bash
bash definitions/pull-new.sh
bash tests/pull-samples.sh
cargo test
```

Then commit the resulting diffs: `definitions/*.yml`, `definitions/UPSTREAM_COMMIT`, and any `tests/samples.rs` change (see below).

### If a heuristic regex fails to compile after a pull

Upstream heuristics are used verbatim — there is no local patching. Upstream's own CI checks every heuristic pattern for portability across the Ruby, Go and Rust regex engines (`script/check-regex-compatibility` in github-linguist), so a pattern `fancy-regex` cannot compile should be rare. If one slips through, `tests/definitions_compile.rs` fails and names the offending rule (it compiles every heuristic and vendor pattern eagerly — at runtime the library compiles heuristics lazily, so without it a bad pattern would only surface as a `LinguistError::InvalidRegex` for files of that extension). Fix it upstream (as was done for the Adblock, Rez and Smarty patterns) rather than working around it here.

## Expected `tests/samples` failures after an upstream pull

When the suite fails after a pull, it is almost always one of these upstream changes — not a bug in the Rust code. Detection is usually correct; the *expectation* (derived from the sample's folder name) is what drifted.

- **Extension recategorised** — e.g. `.service` moved from `desktop` to `INI`. The paired samples pull fixes it (upstream moves the sample file to the new language's folder).
- **Language renamed** — e.g. `robots.txt` → `Robots Exclusion Rules` (same `language_id`). The paired samples pull fixes it (upstream renames the folder).
- **New language with a filesystem-unfriendly name** — a language whose name contains characters like `*` declares an `fs_name` in `languages.yml`, and its samples live under that `fs_name`. The samples test can't read `fs_name`, so add a folder→name entry to `canonical_language_name()` in `tests/samples.rs`. Examples already present: `Fstar` → `F*`, `ProC` → `Pro*C`.

A genuine bug looks different: a sample whose folder name is unchanged upstream starts detecting as the wrong language. Investigate those against the definition and heuristic logic.

## CI

`.github/workflows/ci.yml` runs on PRs, pushes to main, and `workflow_dispatch` (used by the sync workflow, see above):

- `cargo fmt --check`, clippy (deny warnings) and `cargo test` in **both** feature configurations — default (library only) and `--features cli` (the binary is gated behind the `cli` feature; `cargo install linguist` and `cargo run` need `--features cli`).
- An MSRV job that checks the crate on the toolchain declared as `rust-version` in `Cargo.toml` (read dynamically, so bumping the manifest is the only step). Note the MSRV is set by *syntax the code uses* (currently let-chains → 1.88), not just the edition.

Dependabot (`.github/dependabot.yml`) opens weekly PRs for cargo and GitHub Actions updates.

## Branch protection

`main` is governed by the `protect-main` ruleset (GitHub → Settings → Rules → Rulesets; it lives in repository settings, not in this tree). It requires every change to arrive via a pull request (zero approvals — the maintainer can't approve their own PR), and blocks force-pushes and deletion. There are no bypass actors: the escape hatch is temporarily disabling the ruleset, which is deliberate friction.

Besides keeping history reviewable, the PR requirement is what makes the generated release notes complete: GitHub builds them from merged PRs, so a commit pushed straight to main would never be listed.

Required status checks are **not** enabled yet: they would block the weekly sync PR unless the `workflow_dispatch` trick above reliably produces the CI checks on it. Once that has held up over a few sync runs, add `Test, clippy, fmt (stable)` and `MSRV (rust-version in Cargo.toml)` as required checks to the ruleset.

## Releasing

The workspace has two independently-versioned crates:

| Crate | File | Notes |
|-------|------|-------|
| `linguist` | `Cargo.toml` (root `[package]`) | the published library; released via git tags |
| `linguist-types` | `linguist-types/Cargo.toml` | serde types; depended on by the root via `linguist-types = { path = "linguist-types", version = "0.1" }` |

To release `linguist`:

1. Bump `version` in the root `Cargo.toml` (a definitions refresh changes detection behaviour but not the public API, so it is normally a **patch** bump — and sync PRs already include this bump). Run `cargo test` so `Cargo.lock` reflects the new version, and land it on main via a PR (main rejects direct pushes — see [Branch protection](#branch-protection)).
2. Tag the merge commit and push the tag: `git pull && git tag vX.Y.Z && git push origin --tags`.
3. The **Release** workflow (`.github/workflows/release.yml`) verifies the tag matches `Cargo.toml`, runs the tests, publishes to crates.io, and creates a GitHub Release with generated notes.

The workflow needs a `CARGO_REGISTRY_TOKEN` repository secret (a crates.io API token with publish scope for `linguist`).

`linguist-types` releases stay manual (they are rare): bump its version, `cargo publish -p linguist-types`, and commit. If it crosses to `0.2.x`, update the root's `version = "0.1"` dep spec too — and publish `linguist-types` **before** tagging a `linguist` release that depends on it.
