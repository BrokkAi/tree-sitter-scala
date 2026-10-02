# Brokk's Scala Grammar for Tree-sitter

[![CI](https://img.shields.io/github/actions/workflow/status/BrokkAi/tree-sitter-scala/ci.yml?branch=master&logo=github&label=CI)](https://github.com/BrokkAi/tree-sitter-scala/actions/workflows/ci.yml)
[![Playground](https://img.shields.io/github/actions/workflow/status/BrokkAi/tree-sitter-scala/pages.yml?branch=master&logo=github&label=Playground)](https://brokkai.github.io/tree-sitter-scala/)

This is the **Brokk-owned and independently maintained fork** of
[`tree-sitter/tree-sitter-scala`](https://github.com/tree-sitter/tree-sitter-scala),
a [Tree-sitter](https://tree-sitter.github.io/tree-sitter/) grammar for Scala 2
and Scala 3. Brokk maintains it for its code-intelligence tooling.

Try the grammar in [Brokk's web playground](https://brokkai.github.io/tree-sitter-scala/).

The `master` branch carries the grammar previously vendored by Bifrost,
including the Scala 2 `Export.export(project, file)` selector correction.
Upstream history, the MIT license, bindings, queries and corpus tests are
preserved. See [BROKK_PATCH.md](BROKK_PATCH.md) for exact provenance.

## Rust package

The released Rust package is
[`brokk-tree-sitter-scala` 0.26.3](https://crates.io/crates/brokk-tree-sitter-scala/0.26.3).
Bifrost uses this registry package, including the contextual selector correction:

```toml
brokk-tree-sitter-scala = "=0.26.3"
```

Rust builds use the checked-in parser and need no Node or Tree-sitter CLI.
The native entry point remains `tree_sitter_scala`; use one Scala grammar per
binary. Other bindings retain their upstream identities. The npm manifest is
private, and there is no automatic package publication workflow.

## Development

See [CONTRIBUTING.md](CONTRIBUTING.md) for setup and regeneration instructions.

```sh
cargo test --locked
cargo clippy --locked --all-targets -- -D warnings
cargo fmt --check
npm ci --ignore-scripts
npm rebuild tree-sitter-cli
npx tree-sitter test --rebuild
```

The CI checks the grammar corpus, highlighting and tag assertions, Rust
bindings and parser regressions, formatting, linting and standalone packaging
on Linux, Windows and macOS. The Pages workflow builds the checked-in parser
as WebAssembly and deploys its playground.

## Language references

- [Scala 2 language specification](https://www.scala-lang.org/files/archive/spec/2.13/)
- [Scala 2 syntax summary](https://www.scala-lang.org/files/archive/spec/2.13/13-syntax-summary.html)
- [Scala 3 syntax summary](https://docs.scala-lang.org/scala3/reference/syntax.html)
