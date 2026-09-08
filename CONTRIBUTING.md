# Contributing

Thanks for contributing! A few notes make the process smooth for everyone.

## Does your change belong here or upstream?

This crate vendors its language data from
[github-linguist](https://github.com/github-linguist/linguist) — the
`definitions/*.yml` files are machine-written snapshots, refreshed weekly by
an automated sync, and hand edits to them will be overwritten.

- **A language is missing, has the wrong extensions, colors, or heuristics** →
  contribute to [github-linguist](https://github.com/github-linguist/linguist)
  directly. Your fix lands here automatically within a week of being merged
  upstream.
- **Detection logic, API, CLI, performance, or a Rust bug** → this is the
  right repo, read on.
- That includes heuristic regexes `fancy-regex` can't compile: upstream
  checks pattern portability across regex engines in its CI, so the fix
  belongs there too (see [MAINTAINING.md](MAINTAINING.md)).

## Getting set up

```bash
git clone https://github.com/drshade/linguist && cd linguist
bash tests/pull-samples.sh   # fetch test fixtures — required!
cargo test
```

**Don't skip `pull-samples.sh`.** The main detection suite quietly self-skips
when `tests/samples/` is absent, so `cargo test` on a fresh clone passes
without actually testing detection.

The CLI binary is feature-gated: `cargo run --features cli -- <files>`.

## Opening a pull request

CI will check all of this, but you'll iterate faster running it locally:

```bash
cargo fmt --all --check
cargo clippy --all-targets -- -D warnings
cargo clippy --all-targets --features cli -- -D warnings
cargo test && cargo test --features cli
```

Please also:

- **Expect to go through a PR** — `main` doesn't accept direct pushes, for
  the maintainer either; merged PRs are also what the release notes are
  generated from.
- **Don't bump version numbers or add tags** — releases are handled by the
  maintainer (versions are bumped automatically by the sync workflow, and
  merging a version bump to main publishes it automatically).
- Keep the PR focused on one change, and say what it does and why in the
  description.
- Add or extend tests for behaviour changes (`tests/` has suites per
  detection method).

If you work with a coding agent, point it at [AGENTS.md](AGENTS.md) — it
covers the same traps in agent-directed form.
