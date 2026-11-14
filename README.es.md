# TACTIL — Capa de salida táctil: formatos braille (BRF, Unicode), líneas braille, impresoras, vibración en pantalla táctil; usa louis-rs para traducir.

[English](README.md) · [Română](README.ro.md) · [Français](README.fr.md) · [Italiano](README.it.md) · **Español** · [Português](README.pt.md) · [Deutsch](README.de.md) · [Magyar](README.hu.md)

Versión: 0.1.0-alpha.1 · Última actualización: 2026-11-14

🌐 https://tactil.chiuta.ro

## Estado

Esqueleto inicial (pre-alfa): el workspace compila y las pruebas pasan en CI. Cada función prevista es una tarea abierta en `docs/issues/`.

## Alcance previsto

- Celdas braille de seis y ocho puntos, con cada carácter BRF asociado a su conjunto de puntos y documentado en una tabla.
- Maquetación: celdas por línea, líneas por página y salto de línea que nunca parte un grupo de celdas.
- Escritor BRF con saltos de página (form feed) y finales de línea configurables, más un escritor PEF (Portable Embosser Format).
- Motores opcionales: texto a celdas mediante `louis-rs` y salida a línea braille mediante BRLTTY/BrlAPI.
- Modelo de temporización de vibraciones en pantalla táctil (el núcleo ATINGERE) y gráficos táctiles a partir de imágenes ráster.

Nada de esto está implementado todavía: cada punto es una tarea abierta en `docs/issues/`.

## Compilar y probar

```bash
cargo test --workspace
cargo run -p tactil-cli -- brf ABC
```

## Contribuir

Lee `CONTRIBUTING.md`, elige una tarea con la etiqueta `good first issue` en `docs/issues/` y abre un pull request. Ninguna contribución es demasiado pequeña: una fila de tabla, una prueba o una traducción cuentan.

## Implementación independiente

Escrita a partir de especificaciones públicas y manuales; no se ha copiado código fuente de terceros (véase `CLEANROOM.md`).

## Licencia

MIT — Copyright (c) 2026 Alexandru-Ionuț Chiuță (Alexio).

## Apoyo

Trade-free: sin rastreo ni publicidad. Si te resulta útil, puedes apoyar al autor: https://buymeacoffee.com/echo.of.the.strings
