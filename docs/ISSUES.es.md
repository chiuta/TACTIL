# TACTIL — primeras diez tareas

[English](ISSUES.md) · [Română](ISSUES.ro.md) · [Français](ISSUES.fr.md) · [Italiano](ISSUES.it.md) · **Español** · [Português](ISSUES.pt.md) · [Deutsch](ISSUES.de.md) · [Magyar](ISSUES.hu.md)

Créalas en GitHub con `scripts/create-issues.sh OWNER/REPO`. Comenta en una issue para reclamarla.

1. **Amplía las pruebas: cada carácter BRF ↔ conjunto de puntos, escrito como tabla en la documentación** — `good first issue`, `tests`, `docs`
   Genera `docs/BRF-TABLE.md` desde el código y compáralo con una tabla publicada (cítala).

2. **`Cell` para braille de ocho puntos (braille informático) junto al tipo de seis puntos** — `good first issue`, `area:core`
   Los puntos 7–8 corresponden a U+2840..U+28FF; deja sin cambios la API de seis puntos.

3. **Maquetación de página: celdas por línea y líneas por página; ajuste de palabras que nunca parta un grupo de celdas** — `help wanted`, `area:layout`
   Parámetros para tamaños comunes de embosadoras; pruebas con archivos de referencia.

4. **Escribe archivos BRF con saltos de página (form feed) y finales de línea configurables** — `good first issue`, `area:formats`
   Opción CRLF/LF; ida y vuelta con el lector BRF; documenta las suposiciones de codificación.

5. **Backend opcional `louis-rs`: texto → celdas con una tabla elegida** — `help wanted`, `area:translate`
   Tras una funcionalidad de cargo; ruta de la tabla configurable; sin red; prueba con una tabla inglesa y una rumana.

6. **Escritor PEF (Portable Embosser Format)** — `help wanted`, `area:formats`
   Sigue la especificación pública de PEF; valida la salida contra su esquema en CI.

7. **Modelo de temporización de vibración para pantalla táctil (núcleo ATINGERE)** — `help wanted`, `area:touch`
   Celda → secuencia de pulsos (duración, encendido/apagado) para un dedo que explora; función pura + pruebas; la llamada de vibración de la plataforma queda fuera del alcance.

8. **Salida a línea braille mediante BRLTTY/BrlAPI (funcionalidad opcional)** — `help wanted`, `area:displays`
   Solo tras una nota de diseño; hacen falta probadores con hardware real — di si tienes uno.

9. **Gráficos táctiles: imagen raster → mapa de puntos en relieve con umbral y tramado** — `help wanted`, `area:graphics`
   Tamaños de cuadrícula para embosadoras comunes; salida determinista; imágenes de muestra generadas en las pruebas.

10. **Guía de contribución para tablas de traducción: envíalas a liblouis, no aquí** — `good first issue`, `docs`
   Escribe `docs/UPSTREAM.md` explicando cómo proponer una tabla y una prueba al proyecto liblouis y cómo la consume TACTIL.
