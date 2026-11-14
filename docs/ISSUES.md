# TACTIL — first ten tasks

**English** · [Română](ISSUES.ro.md) · [Français](ISSUES.fr.md) · [Italiano](ISSUES.it.md) · [Español](ISSUES.es.md) · [Português](ISSUES.pt.md) · [Deutsch](ISSUES.de.md) · [Magyar](ISSUES.hu.md)

Create them on GitHub with `scripts/create-issues.sh OWNER/REPO`. Comment on an issue to claim it.

1. **Extend tests: every BRF character ↔ dot set, written out as a table in the docs** — `good first issue`, `tests`, `docs`
   Generate `docs/BRF-TABLE.md` from code and compare with a published table (cite it).

2. **`Cell` for eight-dot braille (computer braille) alongside the six-dot type** — `good first issue`, `area:core`
   Dots 7–8 map to U+2840..U+28FF; keep the six-dot API unchanged.

3. **Page layout: cells per line and lines per page; word wrap that never splits a cell group** — `help wanted`, `area:layout`
   Parameters for common embosser sizes; golden-file tests.

4. **Write BRF files with page breaks (form feed) and line endings configurable** — `good first issue`, `area:formats`
   CRLF/LF option; round-trip with the BRF reader; document encoding assumptions.

5. **Optional `louis-rs` backend: text → cells with a chosen table** — `help wanted`, `area:translate`
   Behind a cargo feature; table path configurable; no network; test with one English and one Romanian table.

6. **PEF (Portable Embosser Format) writer** — `help wanted`, `area:formats`
   Follow the public PEF specification; validate output against its schema in CI.

7. **Touch-screen vibration timing model (ATINGERE core)** — `help wanted`, `area:touch`
   Cell → sequence of (duration, on/off) pulses for a scanning finger; pure function + tests; the platform vibration call is out of scope.

8. **Braille display output through BRLTTY/BrlAPI (optional feature)** — `help wanted`, `area:displays`
   Only after a design note; needs real hardware testers — say if you have one.

9. **Tactile graphics: raster image → raised-dot map with thresholding and dithering** — `help wanted`, `area:graphics`
   Grid sizes for common embossers; deterministic output; sample images generated in the tests.

10. **Contribution guide for translation tables: send them upstream to liblouis, not here** — `good first issue`, `docs`
   Write `docs/UPSTREAM.md` explaining how to propose a table and test to the liblouis project, and how TACTIL consumes it.
