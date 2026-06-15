# Maintaining

How to keep this crate in sync with upstream [github-linguist](https://github.com/github-linguist/linguist).

## Upstream data sources

This crate vendors two independent sets of files from upstream, both tracking `main` HEAD:

| What | Location | Pulled by |
|------|----------|-----------|
| Language/heuristic/vendor definitions | `definitions/*.yml` | `definitions/pull-new.sh` |
| Detection test fixtures | `tests/samples/` | `tests/pull-samples.sh` |

**They must be pulled together.** Both scripts fetch the current upstream `main`, so the definitions and the samples are a matched pair. Updating one without the other causes spurious `tests/samples` failures, because the samples test asserts each file under `tests/samples/<Language>/` detects as `<Language>` — and upstream changes language names, sample categorisation, and definitions in the same commits.

## Updating

```bash
cd definitions && bash pull-new.sh && cd ..
bash tests/pull-samples.sh
cargo test
```

Then commit the resulting diffs: the `definitions/*.yml` and any `tests/samples.rs` change (see below). Note `tests/samples/` is gitignored — it is re-fetched locally and never committed, so each developer runs `tests/pull-samples.sh` themselves.

### If `pull-new.sh`'s patch step fails

`heuristics.yml` is produced by applying `definitions/heuristics.patch` to the raw upstream `heuristics_original.yml`. The patch rewrites the one Ruby/Oniguruma regex feature `fancy-regex` cannot handle (the `\g<version>` subroutine call in the Adblock Filter List pattern). If `patch` fails, upstream changed that pattern — update `heuristics.patch` to match the new upstream text, then re-run.

## Expected `tests/samples` failures after an upstream pull

When the suite fails after re-pulling **both** sources, it is almost always one of these upstream changes — not a bug in the Rust code. Detection is usually correct; the *expectation* (derived from the sample's folder name) is what drifted.

- **Extension recategorised** — e.g. `.service` moved from `desktop` to `INI`. Re-pulling samples fixes it (upstream moves the sample file to the new language's folder).
- **Language renamed** — e.g. `robots.txt` → `Robots Exclusion Rules` (same `language_id`). Re-pulling samples fixes it (upstream renames the folder).
- **New language with a filesystem-unfriendly name** — a language whose name contains characters like `*` declares an `fs_name` in `languages.yml`, and its samples live under that `fs_name`. The samples test can't read `fs_name`, so add a folder→name entry to `canonical_language_name()` in `tests/samples.rs`. Examples already present: `Fstar` → `F*`, `ProC` → `Pro*C`.

A genuine bug looks different: a sample whose folder name is unchanged upstream starts detecting as the wrong language. Investigate those against the definition and heuristic logic.

## Releasing

After syncing and getting a green `cargo test`, bump the version before publishing. The workspace has two independently-versioned crates:

| Crate | File | Notes |
|-------|------|-------|
| `linguist` | `Cargo.toml` (root `[package]`) | the published library |
| `linguist-types` | `linguist-types/Cargo.toml` | serde types; depended on by the root via `linguist-types = { path = "linguist-types", version = "0.1" }` |

- A refreshed upstream pull changes detection behaviour but not the public API, so it is normally a **patch** bump of `linguist`. Bump `linguist-types` only if its types changed.
- The root's dependency uses a `"0.1"` range, so a patch/minor bump of `linguist-types` within `0.1.x` needs no change to the dep spec. If `linguist-types` crosses to `0.2.x`, update that `version = "0.1"` line too — and publish `linguist-types` to crates.io **before** `linguist`.
- Run `cargo test` once more after bumping so `Cargo.lock` reflects the new versions, then commit.
