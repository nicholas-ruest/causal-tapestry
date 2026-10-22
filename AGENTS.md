# Causal Tapestry contributor contract

This repository implements the 2026-10-21 Dream Machine build. Design commit must precede implementation commits.

- Keep one writer per worktree; only the integration owner edits Cargo.toml or Cargo.lock.
- Rust files and Markdown files stay below 500 lines.
- Validate all trace, path, policy, and bundle inputs.
- Never commit secrets or .env files.
- Core-memory is build guidance, never a runtime dependency.
- A crate counts only with implemented behavior, a typed API, a real caller or executable, and tests.
- Upstream claims require an executed boundary and raw evidence.
- Publication requires the pinned structural checker plus independent semantic review.

