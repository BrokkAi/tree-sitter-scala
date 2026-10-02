# Brokk Scala grammar provenance

The grammar snapshot starts at upstream `tree-sitter/tree-sitter-scala` tag `v0.26.2`,
commit `b931fcc338390925eb893d70ad070033f5856ccf`. Upstream history and the MIT
license are preserved. The fork's initial upstream master commit,
`db390f312a54b04b13790e1767bfac32665c17ac`, is also retained in the maintained
branch's ancestry. Its unreleased grammar changes are not activated by this
snapshot export; they remain candidates for a separately validated update.

The grammar payload was copied byte for byte from Bifrost source commit
`4919a047adf43786b0e60c3595be64576eb884d9`, using only files explicitly
classified public. `VENDORED_SOURCE.sha256` records the copied bytes. The
original crates.io `tree-sitter-scala` 0.26.2 archive checksum is
`24e0ab4505990bfe30051761d40a7bf4033ce5a81c9eda9e20e987a5cdc84826`.

The local grammar correction admits `export` as an identifier in a field
selector, so Scala 2 calls such as `Export.export(project, file)` retain a
structured call expression. Scala 3 export declarations keep their original
rule. Generated parser, node types, scanner and headers are checked in.

The separately named Rust package is `brokk-tree-sitter-scala` 0.26.3. Its
native language entry point remains `tree_sitter_scala`. Consumers should use
one Scala grammar per binary. Rust tests exercise Bifrost's Tree-sitter 0.25.10
runtime family.

Regenerate intentionally with the original export's generator:

```sh
npx --yes tree-sitter-cli@0.26.3 generate --abi 15
```

The snapshot hashes document the initial export. Deliberate future grammar
changes should update this provenance file rather than claim byte identity
with the original export.
