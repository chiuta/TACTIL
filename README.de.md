# TACTIL — Taktile Ausgabeschicht: Braille-Formate (BRF, Unicode), Displays, Prägedrucker, Touch-Vibration; nutzt louis-rs für die Übersetzung.

[English](README.md) · [Română](README.ro.md) · [Français](README.fr.md) · [Italiano](README.it.md) · [Español](README.es.md) · [Português](README.pt.md) · **Deutsch** · [Magyar](README.hu.md)

Version: 0.1.0-alpha.1 · Letzte Aktualisierung: 2026-11-14

🌐 https://tactil.chiuta.ro

## Status

Frühes Grundgerüst (Pre-Alpha): Das Workspace baut, und die Tests laufen in der CI durch. Jede geplante Funktion ist eine offene Aufgabe in `docs/issues/`.

## Geplanter Umfang

- Sechs- und Achtpunkt-Braillezellen, jedes BRF-Zeichen seiner Punktmenge zugeordnet und in einer Tabelle dokumentiert.
- Seitenlayout: Zellen pro Zeile, Zeilen pro Seite und ein Zeilenumbruch, der nie eine Zellgruppe trennt.
- BRF-Schreiber mit Seitenumbrüchen (Form Feed) und einstellbaren Zeilenenden sowie ein PEF-Schreiber (Portable Embosser Format).
- Optionale Backends: Text zu Zellen über `louis-rs` und Ausgabe auf Braillezeilen über BRLTTY/BrlAPI.
- Zeitmodell für Vibrationen auf Touchscreens (der ATINGERE-Kern) und taktile Grafiken aus Rasterbildern.

Nichts davon ist bisher umgesetzt: Jeder Punkt ist eine offene Aufgabe in `docs/issues/`.

## Bauen und testen

```bash
cargo test --workspace
cargo run -p tactil-cli -- brf ABC
```

## Mitmachen

Lies `CONTRIBUTING.md`, wähle in `docs/issues/` eine Aufgabe mit dem Label `good first issue` und öffne einen Pull Request. Kein Beitrag ist zu klein: eine Tabellenzeile, ein Test oder eine Übersetzung zählt.

## Unabhängige Implementierung

Geschrieben nach öffentlichen Spezifikationen und Lehrbüchern; es wurde kein fremder Quellcode kopiert (siehe `CLEANROOM.md`).

## Lizenz

MIT — Copyright (c) 2026 Alexandru-Ionuț Chiuță (Alexio).

## Unterstützung

Trade-free: kein Tracking, keine Werbung. Wenn es dir nützt, kannst du den Autor unterstützen: https://buymeacoffee.com/echo.of.the.strings
