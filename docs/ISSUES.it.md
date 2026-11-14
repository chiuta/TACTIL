# TACTIL — primi dieci compiti

[English](ISSUES.md) · [Română](ISSUES.ro.md) · [Français](ISSUES.fr.md) · **Italiano** · [Español](ISSUES.es.md) · [Português](ISSUES.pt.md) · [Deutsch](ISSUES.de.md) · [Magyar](ISSUES.hu.md)

Creale su GitHub con `scripts/create-issues.sh OWNER/REPO`. Commenta una issue per prenderla in carico.

1. **Estendi i test: ogni carattere BRF ↔ insieme di punti, scritto come tabella nella documentazione** — `good first issue`, `tests`, `docs`
   Genera `docs/BRF-TABLE.md` dal codice e confronta con una tabella pubblicata (citala).

2. **`Cell` per il braille a otto punti (braille informatico) accanto al tipo a sei punti** — `good first issue`, `area:core`
   I punti 7–8 corrispondono a U+2840..U+28FF; lascia invariata l’API a sei punti.

3. **Impaginazione: celle per riga e righe per pagina; a capo che non spezzi mai un gruppo di celle** — `help wanted`, `area:layout`
   Parametri per le dimensioni comuni di embosser; test con file di riferimento.

4. **Scrivi file BRF con interruzioni di pagina (form feed) e fine riga configurabili** — `good first issue`, `area:formats`
   Opzione CRLF/LF; andata e ritorno con il lettore BRF; documenta le ipotesi di codifica.

5. **Backend opzionale `louis-rs`: testo → celle con una tabella scelta** — `help wanted`, `area:translate`
   Dietro una funzionalità cargo; percorso della tabella configurabile; niente rete; test con una tabella inglese e una rumena.

6. **Writer PEF (Portable Embosser Format)** — `help wanted`, `area:formats`
   Segui la specifica pubblica PEF; valida l’output rispetto al suo schema in CI.

7. **Modello di temporizzazione delle vibrazioni per touchscreen (nucleo ATINGERE)** — `help wanted`, `area:touch`
   Cella → sequenza di impulsi (durata, acceso/spento) per un dito che scorre; funzione pura + test; la chiamata di vibrazione della piattaforma è fuori ambito.

8. **Uscita su display braille tramite BRLTTY/BrlAPI (funzionalità opzionale)** — `help wanted`, `area:displays`
   Solo dopo una nota di progetto; servono tester con hardware reale — di’ se ne hai uno.

9. **Grafica tattile: immagine raster → mappa di punti in rilievo con soglia e dithering** — `help wanted`, `area:graphics`
   Dimensioni di griglia per gli embosser comuni; output deterministico; immagini di esempio generate nei test.

10. **Guida al contributo per le tabelle di traduzione: mandale a monte a liblouis, non qui** — `good first issue`, `docs`
   Scrivi `docs/UPSTREAM.md` spiegando come proporre una tabella e un test al progetto liblouis e come TACTIL la utilizza.
