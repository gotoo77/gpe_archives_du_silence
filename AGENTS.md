# Agent instructions - Les Archives du Silence

## Scope
- Inspect current repository state, relevant GPE APIs, errors, tests and CI before editing.
- Make the smallest justified change. No speculative engine abstraction, no unrelated GPE refactor.
- Keep `main` stable: work in focused branches and submit PRs with validation evidence.

## Mandatory Rust validation
For **every Rust change** (including tests and examples), run from repository root:

```bash
cargo fmt --all -- --check
cargo check --locked --all-targets
cargo test --locked --all-targets
cargo clippy --locked --all-targets --all-features -- -D warnings
git diff --check
```

Use a committed `Cargo.lock` for reproducible application builds. Before the
first lockfile has been created, it must be generated with `cargo generate-lockfile`.
This is a temporary bootstrap state, **not** an excuse to claim locked reproducibility.

**No bypasses**: no `--no-verify`, disabling warnings, removing tests,
excluding targets/features or weakening CI merely to achieve a green result.

If a command cannot run, report **BLOCKED** with cause and evidence, and do
not claim successful validation or merge unfinished work.

## Spoiler-safe beta testing
The project owner plans to be the first blind beta tester. Do not disclose
solutions, glyph meanings, puzzle sequences or hidden states in README, PR title,
issue titles, or normal status reports. Code and internal design docs inevitably
contain spoilers: mark these clearly; only share them with explicit spoiler consent.
The README may contain gameplay controls and build instructions.

## Final report
Briefly state what changed, commands executed, results, exact blocked checks,
and whether the work is merged or only in a PR.
