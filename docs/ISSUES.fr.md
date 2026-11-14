# TACTIL — dix premières tâches

[English](ISSUES.md) · [Română](ISSUES.ro.md) · **Français** · [Italiano](ISSUES.it.md) · [Español](ISSUES.es.md) · [Português](ISSUES.pt.md) · [Deutsch](ISSUES.de.md) · [Magyar](ISSUES.hu.md)

Créez-les sur GitHub avec `scripts/create-issues.sh OWNER/REPO`. Commentez un ticket pour le réclamer.

1. **Étendre les tests : chaque caractère BRF ↔ ensemble de points, écrit en tableau dans la documentation** — `good first issue`, `tests`, `docs`
   Générer `docs/BRF-TABLE.md` à partir du code et comparer avec un tableau publié (le citer).

2. **`Cell` pour le braille huit points (braille informatique) à côté du type six points** — `good first issue`, `area:core`
   Les points 7–8 correspondent à U+2840..U+28FF ; laisser l’API six points inchangée.

3. **Mise en page : cellules par ligne et lignes par page ; césure qui ne sépare jamais un groupe de cellules** — `help wanted`, `area:layout`
   Paramètres pour les tailles courantes d’embosseuses ; tests avec fichiers de référence.

4. **Écrire des fichiers BRF avec sauts de page (form feed) et fins de ligne configurables** — `good first issue`, `area:formats`
   Option CRLF/LF ; aller-retour avec le lecteur BRF ; documenter les hypothèses d’encodage.

5. **Backend optionnel `louis-rs` : texte → cellules avec une table choisie** — `help wanted`, `area:translate`
   Derrière une fonctionnalité cargo ; chemin de table configurable ; pas de réseau ; test avec une table anglaise et une table roumaine.

6. **Écrivain PEF (Portable Embosser Format)** — `help wanted`, `area:formats`
   Suivre la spécification publique PEF ; valider la sortie contre son schéma en CI.

7. **Modèle de timing de vibration pour écran tactile (cœur ATINGERE)** — `help wanted`, `area:touch`
   Cellule → séquence d’impulsions (durée, marche/arrêt) pour un doigt qui balaie ; fonction pure + tests ; l’appel de vibration de la plateforme est hors périmètre.

8. **Sortie sur afficheur braille via BRLTTY/BrlAPI (fonctionnalité optionnelle)** — `help wanted`, `area:displays`
   Seulement après une note de conception ; besoin de testeurs avec du matériel réel — dites si vous en avez un.

9. **Graphiques tactiles : image raster → carte de points en relief avec seuillage et tramage** — `help wanted`, `area:graphics`
   Tailles de grille pour les embosseuses courantes ; sortie déterministe ; images d’exemple générées dans les tests.

10. **Guide de contribution pour les tables de traduction : les envoyer en amont à liblouis, pas ici** — `good first issue`, `docs`
   Écrire `docs/UPSTREAM.md` expliquant comment proposer une table et un test au projet liblouis, et comment TACTIL la consomme.
