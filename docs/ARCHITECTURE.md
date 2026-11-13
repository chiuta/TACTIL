# TACTIL — architecture

**Scope decision:** TACTIL does *not* rewrite the braille translator — `louis-rs` (the liblouis project's own Rust re-implementation) does that.
TACTIL is the layer *after* translation: cells, BRF/Unicode/PEF files, page layout for embossers, display protocols, touch-vibration timing (the core of the ATINGERE app), tactile graphics.
`tactil-core` is `std`-only; `louis-rs` is an optional cargo feature.
