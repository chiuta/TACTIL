# TACTIL — Camada de saída tátil: formatos braille (BRF, Unicode), linhas braille, impressoras, vibração em ecrã tátil; usa louis-rs para tradução.

[English](README.md) · [Română](README.ro.md) · [Français](README.fr.md) · [Italiano](README.it.md) · [Español](README.es.md) · **Português** · [Deutsch](README.de.md) · [Magyar](README.hu.md)

Versão: 0.1.0-alpha.1 · Última atualização: 2026-11-14

🌐 https://tactil.chiuta.ro

## Estado

Esqueleto inicial (pré-alfa): o workspace compila e os testes passam no CI. Cada funcionalidade prevista é uma tarefa aberta em `docs/issues/`.

## Âmbito previsto

- Células braille de seis e oito pontos, com cada carácter BRF associado ao seu conjunto de pontos e documentado numa tabela.
- Paginação: células por linha, linhas por página e quebra de linha que nunca parte um grupo de células.
- Escritor BRF com quebras de página (form feed) e fins de linha configuráveis, mais um escritor PEF (Portable Embosser Format).
- Motores opcionais: texto para células através de `louis-rs` e saída para linha braille através de BRLTTY/BrlAPI.
- Modelo de temporização das vibrações em ecrã tátil (o núcleo ATINGERE) e gráficos táteis a partir de imagens raster.

Nada disto está ainda implementado: cada ponto é uma tarefa aberta em `docs/issues/`.

## Compilar e testar

```bash
cargo test --workspace
cargo run -p tactil-cli -- brf ABC
```

## Contribuir

Leia `CONTRIBUTING.md`, escolha uma tarefa com a etiqueta `good first issue` em `docs/issues/` e abra um pull request. Nenhuma contribuição é demasiado pequena: uma linha de tabela, um teste ou uma tradução contam.

## Implementação independente

Escrita a partir de especificações públicas e manuais; não foi copiado código-fonte de terceiros (ver `CLEANROOM.md`).

## Licença

MIT — Copyright (c) 2026 Alexandru-Ionuț Chiuță (Alexio).

## Apoio

Trade-free: sem rastreamento nem publicidade. Se lhe for útil, pode apoiar o autor: https://buymeacoffee.com/echo.of.the.strings
