# Golden Core

- Keep crates and UI packages usable by both products through public APIs. App-specific nodes, policies, and UI belong in the application repositories.
- Keep Rust DTOs and generated TypeScript in one change. Run `npm run codegen` after changing a shared contract and check that a second generation leaves the checkout clean.
- Validate the touched Rust and UI layers together. Use `cargo fmt --all`, locked Cargo checks and tests, and the affected npm workspace checks. Run native hardware tests only after the relevant software checks pass.
- Keep runtime and tests separate. Place Rust unit-test modules and fixtures under `tests/` directories. Keep generated output and vendor notices with their owners.
- The pinned toolchain and platform libraries are documented in [the README](README.md) and [CI](.github/workflows/ci.yml).
