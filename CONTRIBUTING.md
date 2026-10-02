# Contributing to Brokk's Scala grammar

`master` is Brokk's maintained branch. Preserve upstream attribution and record
intentional divergence in [BROKK_PATCH.md](BROKK_PATCH.md). Add a small corpus
case or Rust regression for changed parser behavior.

## Setup and validation

Install Node 24, a C compiler and stable Rust with rustfmt and clippy.

```sh
npm ci --ignore-scripts
npm rebuild tree-sitter-cli
npx tree-sitter test --rebuild
cargo test --locked
cargo clippy --locked --all-targets -- -D warnings
cargo fmt --check
cargo package --locked
```

The CLI is pinned to 0.26.3. The checked-in parser is ABI 15 and is exercised
against the Tree-sitter 0.25.10 runtime family used by Bifrost. The upstream
corpus contains explicitly pending cases; keep that incompleteness visible.

## Regeneration

```sh
npx tree-sitter generate --abi 15
npx tree-sitter test --rebuild
```

Commit deliberate grammar changes together with their generated parser,
grammar JSON and node-type artifacts. The initial exported bytes are recorded
in `VENDORED_SOURCE.sha256`; record future changes in `BROKK_PATCH.md`.
There is no scheduled workflow that silently regenerates this fork.

## Playground

With Emscripten installed or Docker available:

```sh
npx tree-sitter build --wasm
npx tree-sitter playground --export site
python3 script/brand-playground.py site/index.html
```

The Pages workflow performs this build and deploys the result on each push to
`master`. Do not commit `site/` or local compiler artifacts.

## Releases

The standalone Rust package is `brokk-tree-sitter-scala`. Release preparation
must validate the exact source and package before publication, and update the
Rust and grammar metadata deliberately. The npm package is private; other
language bindings retain upstream identities and are not Brokk registry
releases. This bootstrap does not configure automatic registry publication.
