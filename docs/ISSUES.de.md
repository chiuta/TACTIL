# TACTIL — die ersten zehn Aufgaben

[English](ISSUES.md) · [Română](ISSUES.ro.md) · [Français](ISSUES.fr.md) · [Italiano](ISSUES.it.md) · [Español](ISSUES.es.md) · [Português](ISSUES.pt.md) · **Deutsch** · [Magyar](ISSUES.hu.md)

Lege sie auf GitHub mit `scripts/create-issues.sh OWNER/REPO` an. Kommentiere ein Issue, um es zu übernehmen.

1. **Tests erweitern: jedes BRF-Zeichen ↔ Punktmenge, als Tabelle in der Dokumentation ausgeschrieben** — `good first issue`, `tests`, `docs`
   `docs/BRF-TABLE.md` aus dem Code erzeugen und mit einer veröffentlichten Tabelle vergleichen (zitieren).

2. **`Cell` für Achtpunkt-Braille (Computerbraille) neben dem Sechspunkt-Typ** — `good first issue`, `area:core`
   Punkte 7–8 entsprechen U+2840..U+28FF; die Sechspunkt-API unverändert lassen.

3. **Seitenlayout: Zellen pro Zeile und Zeilen pro Seite; Umbruch, der nie eine Zellgruppe trennt** — `help wanted`, `area:layout`
   Parameter für gängige Prägedruckergrößen; Golden-File-Tests.

4. **BRF-Dateien mit Seitenumbrüchen (Form Feed) und konfigurierbaren Zeilenenden schreiben** — `good first issue`, `area:formats`
   CRLF/LF-Option; Rundlauf mit dem BRF-Leser; Annahmen zur Kodierung dokumentieren.

5. **Optionales `louis-rs`-Backend: Text → Zellen mit einer gewählten Tabelle** — `help wanted`, `area:translate`
   Hinter einem Cargo-Feature; Tabellenpfad konfigurierbar; kein Netzwerk; Test mit einer englischen und einer rumänischen Tabelle.

6. **PEF-Writer (Portable Embosser Format)** — `help wanted`, `area:formats`
   Der öffentlichen PEF-Spezifikation folgen; die Ausgabe in der CI gegen ihr Schema validieren.

7. **Zeitmodell für Touchscreen-Vibration (ATINGERE-Kern)** — `help wanted`, `area:touch`
   Zelle → Folge von (Dauer, an/aus)-Impulsen für einen abtastenden Finger; reine Funktion + Tests; der Vibrationsaufruf der Plattform ist nicht Teil der Aufgabe.

8. **Braillezeilen-Ausgabe über BRLTTY/BrlAPI (optionales Feature)** — `help wanted`, `area:displays`
   Erst nach einer Designnotiz; braucht Tester mit echter Hardware — sag Bescheid, wenn du eine hast.

9. **Taktile Grafik: Rasterbild → Relief-Punktkarte mit Schwellwert und Dithering** — `help wanted`, `area:graphics`
   Rastergrößen für gängige Prägedrucker; deterministische Ausgabe; Beispielbilder werden in den Tests erzeugt.

10. **Beitragsleitfaden für Übersetzungstabellen: sie upstream an liblouis schicken, nicht hierher** — `good first issue`, `docs`
   `docs/UPSTREAM.md` schreiben, das erklärt, wie man dem liblouis-Projekt eine Tabelle und einen Test vorschlägt und wie TACTIL sie nutzt.
