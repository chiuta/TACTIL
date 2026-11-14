# TACTIL — az első tíz feladat

[English](ISSUES.md) · [Română](ISSUES.ro.md) · [Français](ISSUES.fr.md) · [Italiano](ISSUES.it.md) · [Español](ISSUES.es.md) · [Português](ISSUES.pt.md) · [Deutsch](ISSUES.de.md) · **Magyar**

Hozd létre őket a GitHubon a `scripts/create-issues.sh OWNER/REPO` paranccsal. Szólj hozzá egy issue-hoz, hogy lefoglald.

1. **A tesztek bővítése: minden BRF-karakter ↔ ponthalmaz, táblázatként kiírva a dokumentációban** — `good first issue`, `tests`, `docs`
   Generáld a `docs/BRF-TABLE.md`-t a kódból, és hasonlítsd össze egy közzétett táblázattal (hivatkozz rá).

2. **`Cell` nyolcpontos braillehez (számítógépes braille) a hatpontos típus mellett** — `good first issue`, `area:core`
   A 7–8. pont az U+2840..U+28FF tartománynak felel meg; a hatpontos API maradjon változatlan.

3. **Oldalelrendezés: cella soronként és sor oldalanként; szótördelés, amely soha nem vág szét cellacsoportot** — `help wanted`, `area:layout`
   Paraméterek a gyakori domborítónyomtató-méretekhez; golden-file tesztek.

4. **BRF-fájlok írása oldaltöréssel (form feed) és beállítható sorvégekkel** — `good first issue`, `area:formats`
   CRLF/LF opció; oda-vissza út a BRF-olvasóval; dokumentáld a kódolási feltevéseket.

5. **Opcionális `louis-rs` backend: szöveg → cellák egy választott táblával** — `help wanted`, `area:translate`
   Cargo feature mögött; a tábla útvonala beállítható; nincs hálózat; teszt egy angol és egy román táblával.

6. **PEF (Portable Embosser Format) író** — `help wanted`, `area:formats`
   Kövesd a nyilvános PEF-specifikációt; a kimenetet a CI-ben validáld a sémája ellen.

7. **Érintőképernyős rezgésidőzítési modell (ATINGERE mag)** — `help wanted`, `area:touch`
   Cella → (időtartam, be/ki) impulzusok sorozata egy pásztázó ujjhoz; tiszta függvény + tesztek; a platform rezgéshívása nem része a feladatnak.

8. **Braille-sor kimenet BRLTTY/BrlAPI-n át (opcionális funkció)** — `help wanted`, `area:displays`
   Csak tervezési jegyzet után; valódi hardverrel rendelkező tesztelők kellenek — szólj, ha van egyik.

9. **Tapintható grafika: raszterkép → domború pontkép küszöbölt és ditherelt módon** — `help wanted`, `area:graphics`
   Rácsméretek a gyakori domborítónyomtatókhoz; determinisztikus kimenet; a mintaképek a tesztekben generálódnak.

10. **Hozzájárulási útmutató fordítótáblákhoz: upstream küldd őket a liblouis-nak, ne ide** — `good first issue`, `docs`
   Írd meg a `docs/UPSTREAM.md`-t, amely elmagyarázza, hogyan javasolj táblát és tesztet a liblouis projektnek, és hogyan használja azt a TACTIL.
