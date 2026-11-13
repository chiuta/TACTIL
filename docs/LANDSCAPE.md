# Related projects and where TACTIL fits

*Checked with a limited web search on 13 November 2026. Facts change quickly: re-check before announcing or depending on anything below.*

- **louis-rs** — the liblouis project's from-scratch Rust re-implementation, on crates.io (v0.3.0 announced on liblouis.io): <https://liblouis.io/>. At an earlier stage its README reported passing 68% of the liblouis test suite — check the current figure.
- **liblouis** (C) — the reference translator, used by Bookshare and many assistive products.
- **Rust bindings:** `brlapi` (BRLTTY display API; safe liblouis wrappers), `braille-ascii` (small BRF crate).
- **Gap targeted:** the *output side* — page layout, PEF, display/embosser drivers, touch-screen vibration patterns, tactile graphics — in one MIT-licensed crate family. We did not find it in a limited search.
