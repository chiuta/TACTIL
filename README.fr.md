# TACTIL — Couche de sortie tactile : formats braille (BRF, Unicode), afficheurs, embosseuses, vibrations tactiles ; s'appuie sur louis-rs pour la traduction.

[English](README.md) · [Română](README.ro.md) · **Français** · [Italiano](README.it.md) · [Español](README.es.md) · [Português](README.pt.md) · [Deutsch](README.de.md) · [Magyar](README.hu.md)

Version : 0.1.0-alpha.1 · Dernière mise à jour : 2026-11-14

🌐 https://tactil.chiuta.ro

## État

Squelette précoce (pré-alpha) : le workspace compile et les tests passent en CI. Chaque fonctionnalité prévue est une tâche ouverte dans `docs/issues/`.

## Périmètre prévu

- Cellules braille à six et huit points, chaque caractère BRF associé à son ensemble de points et documenté dans un tableau.
- Mise en page : cellules par ligne, lignes par page et retour à la ligne qui ne coupe jamais un groupe de cellules.
- Écriture BRF avec sauts de page (form feed) et fins de ligne configurables, plus une écriture PEF (Portable Embosser Format).
- Moteurs optionnels : texte vers cellules via `louis-rs` et sortie sur afficheur braille via BRLTTY/BrlAPI.
- Modèle de temporisation des vibrations d’écran tactile (le cœur ATINGERE) et graphiques tactiles à partir d’images raster.

Rien de tout cela n’est encore implémenté : chaque point est une tâche ouverte dans `docs/issues/`.

## Compiler et tester

```bash
cargo test --workspace
cargo run -p tactil-cli -- brf ABC
```

## Contribuer

Lisez `CONTRIBUTING.md`, choisissez une tâche étiquetée `good first issue` dans `docs/issues/` et ouvrez une pull request. Aucune contribution n'est trop petite : une ligne de table, un test ou une traduction comptent.

## Implémentation indépendante

Écrite à partir de spécifications publiques et de manuels ; aucun code source tiers n'a été copié (voir `CLEANROOM.md`).

## Licence

MIT — Copyright (c) 2026 Alexandru-Ionuț Chiuță (Alexio).

## Soutien

Trade-free : ni pistage ni publicité. Si cela vous est utile, vous pouvez soutenir l'auteur : https://buymeacoffee.com/echo.of.the.strings
