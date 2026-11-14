# TACTIL — primeiras dez tarefas

[English](ISSUES.md) · [Română](ISSUES.ro.md) · [Français](ISSUES.fr.md) · [Italiano](ISSUES.it.md) · [Español](ISSUES.es.md) · **Português** · [Deutsch](ISSUES.de.md) · [Magyar](ISSUES.hu.md)

Crie-as no GitHub com `scripts/create-issues.sh OWNER/REPO`. Comente numa issue para a reivindicar.

1. **Amplie os testes: cada caractere BRF ↔ conjunto de pontos, escrito como tabela na documentação** — `good first issue`, `tests`, `docs`
   Gere `docs/BRF-TABLE.md` a partir do código e compare com uma tabela publicada (cite-a).

2. **`Cell` para braille de oito pontos (braille de computador) ao lado do tipo de seis pontos** — `good first issue`, `area:core`
   Os pontos 7–8 correspondem a U+2840..U+28FF; mantenha inalterada a API de seis pontos.

3. **Layout de página: células por linha e linhas por página; quebra de palavras que nunca parta um grupo de células** — `help wanted`, `area:layout`
   Parâmetros para tamanhos comuns de impressoras braille; testes com arquivos de referência.

4. **Escreva arquivos BRF com quebras de página (form feed) e fins de linha configuráveis** — `good first issue`, `area:formats`
   Opção CRLF/LF; ida e volta com o leitor BRF; documente as suposições de codificação.

5. **Backend opcional `louis-rs`: texto → células com uma tabela escolhida** — `help wanted`, `area:translate`
   Atrás de uma funcionalidade do cargo; caminho da tabela configurável; sem rede; teste com uma tabela inglesa e uma romena.

6. **Escritor PEF (Portable Embosser Format)** — `help wanted`, `area:formats`
   Siga a especificação pública do PEF; valide a saída contra o seu esquema no CI.

7. **Modelo de temporização de vibração para tela sensível ao toque (núcleo ATINGERE)** — `help wanted`, `area:touch`
   Célula → sequência de pulsos (duração, ligado/desligado) para um dedo que varre; função pura + testes; a chamada de vibração da plataforma fica fora do escopo.

8. **Saída para linha braille via BRLTTY/BrlAPI (funcionalidade opcional)** — `help wanted`, `area:displays`
   Só depois de uma nota de projeto; precisa de testadores com hardware real — diga se tem um.

9. **Gráficos táteis: imagem raster → mapa de pontos em relevo com limiar e dithering** — `help wanted`, `area:graphics`
   Tamanhos de grade para impressoras braille comuns; saída determinística; imagens de exemplo geradas nos testes.

10. **Guia de contribuição para tabelas de tradução: envie-as à montante para o liblouis, não aqui** — `good first issue`, `docs`
   Escreva `docs/UPSTREAM.md` explicando como propor uma tabela e um teste ao projeto liblouis e como o TACTIL a consome.
