# TACTIL — primele zece sarcini

[English](ISSUES.md) · **Română** · [Français](ISSUES.fr.md) · [Italiano](ISSUES.it.md) · [Español](ISSUES.es.md) · [Português](ISSUES.pt.md) · [Deutsch](ISSUES.de.md) · [Magyar](ISSUES.hu.md)

Creează-le pe GitHub cu `scripts/create-issues.sh OWNER/REPO`. Comentează la un issue ca să-l revendici.

1. **Extinde testele: fiecare caracter BRF ↔ set de puncte, scris ca tabel în documentație** — `good first issue`, `tests`, `docs`
   Generează `docs/BRF-TABLE.md` din cod și compară cu un tabel publicat (citează-l).

2. **`Cell` pentru braille cu opt puncte (braille de calculator) alături de tipul cu șase puncte** — `good first issue`, `area:core`
   Punctele 7–8 corespund U+2840..U+28FF; păstrează neschimbat API-ul cu șase puncte.

3. **Aranjarea paginii: celule pe rând și rânduri pe pagină; împărțire în cuvinte care nu desparte niciodată un grup de celule** — `help wanted`, `area:layout`
   Parametri pentru dimensiunile uzuale de embosere; teste cu fișiere de referință.

4. **Scrie fișiere BRF cu salturi de pagină (form feed) și terminatori de rând configurabili** — `good first issue`, `area:formats`
   Opțiune CRLF/LF; parcurs dus-întors cu cititorul BRF; documentează presupunerile de codare.

5. **Backend opțional `louis-rs`: text → celule cu un tabel ales** — `help wanted`, `area:translate`
   În spatele unei funcționalități cargo; calea tabelului configurabilă; fără rețea; test cu un tabel englez și unul românesc.

6. **Scriitor PEF (Portable Embosser Format)** — `help wanted`, `area:formats`
   Urmează specificația publică PEF; validează ieșirea față de schema ei în CI.

7. **Model de temporizare a vibrațiilor pe ecran tactil (nucleul ATINGERE)** — `help wanted`, `area:touch`
   Celulă → secvență de impulsuri (durată, pornit/oprit) pentru un deget care scanează; funcție pură + teste; apelul de vibrație al platformei nu intră în sarcină.

8. **Ieșire pe afișaj braille prin BRLTTY/BrlAPI (funcționalitate opțională)** — `help wanted`, `area:displays`
   Doar după o notă de proiectare; are nevoie de testeri cu hardware real — spune dacă ai unul.

9. **Grafică tactilă: imagine raster → hartă de puncte în relief cu prag și dithering** — `help wanted`, `area:graphics`
   Dimensiuni de grilă pentru emboserele uzuale; ieșire deterministă; imagini de probă generate în teste.

10. **Ghid de contribuție pentru tabelele de traducere: trimite-le în amonte la liblouis, nu aici** — `good first issue`, `docs`
   Scrie `docs/UPSTREAM.md` care explică cum propui un tabel și un test proiectului liblouis și cum le consumă TACTIL.
