# Contributing to egui_markdown

## Getting Started

1. Clone the repository.
2. Run `cargo check` to verify your setup.
3. Run `cargo run --example simple` to see the library in action.

## Code Style

- Follow the conventions in the root `CLAUDE.md`.
- All inline comments end with a period.
- Doc comments (`///`) come before `#[allow(...)]` attributes.
- No comment banners or section headers.
- No `.to_string()` inside `format!`, `println!`, `tracing::info!`, etc.
- Keep `use` imports sorted.

## Before Submitting

Run the local CI script:

```sh
./check.sh
```

This runs `cargo fmt --check`, `cargo clippy` across all feature configurations, `cargo test`, and `cargo doc`.

## Pull Request Guidelines

- Keep PRs focused on a single change.
- Add tests for new parser or layout behavior.
- Update `CHANGELOG.md` under an `[Unreleased]` section.
- Ensure `./check.sh` passes before requesting review.
