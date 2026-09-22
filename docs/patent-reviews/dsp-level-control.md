# Patent Review — DSP level control (AGC + limiter)

Status FTO técnico: **UNCERTAIN**
Data: 2026-09-22
Escopo: helix-dsp / AdaptiveGain / SoftLimiter

## 1. Problema técnico

Manter fala em nível utilizável e limitar picos sem clipping, com:
- estado pequeno por stream;
- baixo CPU/memória;
- sem look-ahead/buffer adicional;
- sem multibanda;
- sem algoritmos proprietários.

## 2. Pesquisa preliminar

A pesquisa pública encontrou diversas famílias relacionadas a AGC/limiting, incluindo:
- AGC com attack/release adaptáveis;
- limiters com look-ahead;
- multiband compressors;
- peak limiting com histórico/smoothing de ganhos;
- AGC associado a identificação de locutor;
- speech leveling baseado em limiter/gain control.

Esses documentos são usados apenas como **mapa do que evitar**, não como receita.

Leads:
- EP2418770A1 — Automatic gain control;
- EP0922328A1/B1 — Multi-band audio compressor with look-ahead clipper;
- US9979369B2 — Audio peak limiting;
- US8185387B1 — Automatic gain control with audio-source/talker association;
- WO2019016199A1 — Speech signal leveling.

Status/jurisdição/claims devem ser confirmados em bases oficiais antes de qualquer promoção comercial.

## 3. Mapa do que evitar

Para manter distância técnica conservadora, o Helix DSP atual não deve incorporar sem novo review:
- look-ahead delay para peak limiting;
- multiband compressor/limiter;
- ajuste dinâmico de attack/release a partir de classificação do sinal;
- histórico de razões de ganho usado para release;
- identificação de locutor/fonte para selecionar estado/ganho;
- topologias específicas descritas em famílias pesquisadas;
- parâmetros/copias de curvas provenientes de terceiros.

## 4. Design Helix atual

### AdaptiveGain
O algoritmo atual:
- mede RMS do frame;
- calcula target_rms / rms;
- limita ganho em faixa fixa;
- usa duas constantes fixas distintas conforme o ganho precisa subir/descer;
- congela estado abaixo do limiar de silêncio;
- não usa look-ahead;
- não usa classificação de locutor;
- não usa multibanda;
- não mantém histórico além de um único ganho escalar.

### SoftLimiter
O algoritmo atual:
- deixa amostras abaixo de um knee fixo **exatamente 1:1**;
- acima do knee aplica uma curva exponencial monotônica própria;
- não usa buffer/look-ahead;
- não usa detector de envelope separado;
- não usa histórico de ganho;
- não usa attack/release;
- não é multibanda.

A curva implementada é uma escolha matemática própria para este projeto e deve ser avaliada por testes/benchmark, não comparada por implementação com terceiros.

## 5. Eficiência

Propriedades atuais:
- O(n) por amostra/frame;
- estado AGC: poucos escalares;
- limiter: stateless;
- sem heap no hot path;
- sem delay/look-ahead;
- sem tabelas/codebooks.

## 6. Gate

- [x] pesquisa preliminar registrada
- [x] mapa de técnicas específicas a evitar
- [x] implementação atual documentada
- [x] testes de unity-below-knee adicionados
- [x] testes de convergência AGC adicionados
- [ ] pesquisa oficial completa de claims/status por território
- [ ] THD/escuta/ABX do limiter
- [ ] fala real com AGC
- [ ] revisão jurídica antes de claim comercial de FTO

## 7. Conclusão

**UNCERTAIN**, porém com arquitetura deliberadamente simples e sem as topologias específicas identificadas na busca preliminar.

Não adicionar look-ahead, multibanda, adaptação inteligente de constantes ou histórico complexo sem novo patent review.
