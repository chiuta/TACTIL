# TACTIL — Tapintható kimeneti réteg: braille formátumok (BRF, Unicode), kijelzők, nyomtatók, érintőképernyős rezgés; a fordításhoz a louis-rs-t használja.

[English](README.md) · [Română](README.ro.md) · [Français](README.fr.md) · [Italiano](README.it.md) · [Español](README.es.md) · [Português](README.pt.md) · [Deutsch](README.de.md) · **Magyar**

Verzió: 0.1.0-alpha.1 · Utolsó frissítés: 2026-11-14

🌐 https://tactil.chiuta.ro

## Állapot

Korai váz (pre-alfa): a workspace lefordul, a tesztek a CI-ben lefutnak. Minden tervezett funkció nyitott feladat a `docs/issues/` mappában.

## Tervezett terjedelem

- Hat- és nyolcpontos braille-cellák, minden BRF-karakter a saját ponthalmazához rendelve és táblázatban dokumentálva.
- Oldalelrendezés: cella soronként, sor oldalanként és szóközi törés, amely sosem vág szét cellacsoportot.
- BRF-író oldaltöréssel (form feed) és beállítható sorvégekkel, valamint PEF-író (Portable Embosser Format).
- Opcionális háttérmodulok: szöveg cellákká a `louis-rs`-sel, és kimenet braille-sorra BRLTTY/BrlAPI-n át.
- Érintőképernyős rezgésidőzítési modell (az ATINGERE magja) és tapintható grafika raszterképekből.

Ezek közül még semmi sincs megvalósítva: minden pont nyitott feladat a `docs/issues/` mappában.

## Fordítás és tesztelés

```bash
cargo test --workspace
cargo run -p tactil-cli -- brf ABC
```

## Közreműködés

Olvasd el a `CONTRIBUTING.md` fájlt, válassz egy `good first issue` címkéjű feladatot a `docs/issues/` mappából, és nyiss pull requestet. Nincs túl kicsi hozzájárulás: egy táblázatsor, egy teszt vagy egy fordítás is számít.

## Független megvalósítás

Nyilvános specifikációk és tankönyvek alapján íródott; harmadik fél forráskódját nem másoltuk (lásd `CLEANROOM.md`).

## Licenc

MIT — Copyright (c) 2026 Alexandru-Ionuț Chiuță (Alexio).

## Támogatás

Trade-free: nincs követés, nincs reklám. Ha hasznos számodra, támogathatod a szerzőt: https://buymeacoffee.com/echo.of.the.strings
