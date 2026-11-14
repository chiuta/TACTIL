# TACTIL — Tactile output layer: braille formats (BRF, Unicode), displays, embossers, touch-screen vibration; uses louis-rs for translation.

**English** · [Română](README.ro.md) · [Français](README.fr.md) · [Italiano](README.it.md) · [Español](README.es.md) · [Português](README.pt.md) · [Deutsch](README.de.md) · [Magyar](README.hu.md)

Version: 0.1.0-alpha.1 · Last updated: 2026-11-14

🌐 https://tactil.chiuta.ro

## Status

Early skeleton (pre-alpha): the workspace builds and the tests pass in CI. Every planned feature is an open task in `docs/issues/`.

## Planned scope

- Six- and eight-dot braille cells, with every BRF character mapped to its dot set and documented in a table.
- Page layout: cells per line, lines per page and word wrap that never splits a cell group.
- BRF writer with form-feed page breaks and configurable line endings, plus a PEF (Portable Embosser Format) writer.
- Optional back-ends: text to cells through `louis-rs`, and braille-display output through BRLTTY/BrlAPI.
- Touch-screen vibration timing model (the ATINGERE core) and tactile graphics from raster images.

None of this is implemented yet: each item is an open task in `docs/issues/`.

## Build and test

```bash
cargo test --workspace
cargo run -p tactil-cli -- brf ABC
```

## Contribute

Read `CONTRIBUTING.md`, pick a task labelled `good first issue` from `docs/issues/` and open a pull request. No contribution is too small: a table row, a test or a translation counts.

## Independent implementation

Written from public specifications and textbooks; no third-party source code is copied (see `CLEANROOM.md`).

## License

MIT — Copyright (c) 2026 Alexandru-Ionuț Chiuță (Alexio).

## Support

Trade-free: no tracking, no ads. If it is useful, you can support the author: https://buymeacoffee.com/echo.of.the.strings
