# Golden Core

Golden Core is the shared Rust runtime and Svelte workbench for [Chataigne2](https://github.com/Golden-Geek/Chataigne2) and [MapGyver2](https://github.com/Golden-Geek/MapGyver2). Applications pin this repository as a Git submodule at `golden/`.

- `crates/golden_core` contains the public application API, engine, persistence, protocol and hosts. See [application contracts](docs/architecture/application-contracts.md) and [host boundaries](docs/architecture/hosts.md).
- `crates/` also owns the reusable graph, asset, audio, video and timing primitives.
- `packages/` contains the generic workbench, graph canvas, audio controls and video preview.
- `vendor/` contains patched dependencies. Consumer Cargo roots apply the required patches from the same submodule revision.

Use the Rust version in `tools/bootstrap/rust-version` and the Node version in `.nvmrc`. See the CI workflow and each crate's documentation for platform libraries and SDKs. From the repository root:

```sh
npm ci
npm run check
npm test
cargo test --locked --workspace -- --test-threads=1
```

Run `npm run codegen` when shared Rust DTOs change. Commit the generated TypeScript with its Rust source. See [the wire contracts](docs/architecture/ui-protocol.md).

This source was extracted from Chataigne2 commit `3b7a4bbcbfc211903a52d43f31d3fe5e17e73ac3`. See the [repository license](LICENSE), [retained Golden Core notice](licenses/golden-core-AGPL-3.0.txt), and notices alongside vendored sources.
