# TACTIL — Livello di output tattile: formati braille (BRF, Unicode), display, stampanti braille, vibrazione su touch; usa louis-rs per la traduzione.

[English](README.md) · [Română](README.ro.md) · [Français](README.fr.md) · **Italiano** · [Español](README.es.md) · [Português](README.pt.md) · [Deutsch](README.de.md) · [Magyar](README.hu.md)

Versione: 0.1.0-alpha.1 · Ultimo aggiornamento: 2026-11-14

🌐 https://tactil.chiuta.ro

## Stato

Scheletro iniziale (pre-alfa): il workspace compila e i test passano in CI. Ogni funzione prevista è un'attività aperta in `docs/issues/`.

## Ambito previsto

- Celle braille a sei e otto punti, con ogni carattere BRF associato al suo insieme di punti e documentato in una tabella.
- Impaginazione: celle per riga, righe per pagina e a capo che non spezza mai un gruppo di celle.
- Scrittore BRF con salti pagina (form feed) e terminatori di riga configurabili, più uno scrittore PEF (Portable Embosser Format).
- Backend facoltativi: testo in celle tramite `louis-rs` e uscita su display braille tramite BRLTTY/BrlAPI.
- Modello di temporizzazione delle vibrazioni su touch screen (il nucleo ATINGERE) e grafica tattile da immagini raster.

Nulla di tutto ciò è ancora implementato: ogni voce è un compito aperto in `docs/issues/`.

## Compilare e testare

```bash
cargo test --workspace
cargo run -p tactil-cli -- brf ABC
```

## Contribuire

Leggi `CONTRIBUTING.md`, scegli un'attività con etichetta `good first issue` in `docs/issues/` e apri una pull request. Nessun contributo è troppo piccolo: una riga di tabella, un test o una traduzione contano.

## Implementazione indipendente

Scritta a partire da specifiche pubbliche e manuali; nessun codice sorgente di terzi è stato copiato (vedi `CLEANROOM.md`).

## Licenza

MIT — Copyright (c) 2026 Alexandru-Ionuț Chiuță (Alexio).

## Sostegno

Trade-free: niente tracciamento, niente pubblicità. Se ti è utile, puoi sostenere l'autore: https://buymeacoffee.com/echo.of.the.strings
