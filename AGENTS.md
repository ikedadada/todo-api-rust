# Repository Guidelines

## Project Structure & Modules
- Root: `backend/` is the Rust crate.
- Entry point: `backend/src/main.rs` (Axum server setup and routing).
- Library modules: `backend/src/lib.rs` re-exports internal modules.
- Presentation layer (HTTP handlers): `backend/src/presentation/` (`health_handler.rs`, `hello_handler.rs`, `wait_handler.rs`).
- Errors: `backend/src/errors.rs` (app-level error responses).
- Tooling: `backend/rust-toolchain.toml`, `backend/mise.toml`, `backend/Makefile`, notes in `backend/memo/`.
- Tests: add unit tests next to modules; integration tests in `backend/tests/`.

## Build, Test, and Development
- Build: `cd backend && cargo build` — compile the crate.
- Run: `cargo run` — starts server on `127.0.0.1:3000`.
- Test: `cargo test` — runs unit/integration tests.
- Lint: `cargo clippy --all-targets --all-features -D warnings` — enforce lints.
- Format: `cargo fmt` or `cargo fmt -- --check` — format/verify formatting.
- Init tools (optional): `make init` — installs `cargo-edit`.

## Coding Style & Naming
- Formatting: rustfmt (Rust 1.89 toolchain pinned). Use 4-space indentation.
- Linting: Clippy; fix or allow with clear justification.
- Naming: `snake_case` for functions/modules, `PascalCase` for types/traits, `SCREAMING_SNAKE_CASE` for consts.
- Modules: keep handlers small and focused; prefer `presentation::<feature>_handler.rs` per route group.

## Testing Guidelines
- Framework: built-in Rust test harness.
- Unit tests: co-locate using `#[cfg(test)] mod tests { ... }` in each file.
- Integration tests: files under `backend/tests/` (e.g., `backend/tests/health.rs`).
- Conventions: test names describe behavior (e.g., `returns_200_on_health`); aim for meaningful coverage of handlers and error paths.

## Commit & Pull Requests
- Commits: imperative mood, concise scope (e.g., "add wait handler timeout handling"). Reference issues (`#123`) when applicable.
- PRs: include summary, motivation, notable changes, and how to verify (commands or sample `curl`). Keep diffs tight to the feature. Screenshots/log snippets helpful for responses.

## Architecture Notes
- Web: Axum `Router` with nested routes; 10s timeout via Tower; graceful shutdown on Ctrl-C/SIGTERM.
- Errors: unify responses via `AppError` implementing `IntoResponse`.
- JSON: serialize/deserialize with Serde.
