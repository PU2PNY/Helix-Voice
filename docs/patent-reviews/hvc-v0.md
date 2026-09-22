# Patent Review — HVC v0 / next-generation HVC

Status FTO técnico: **UNCERTAIN**
Data: 2026-09-22
Territórios prioritários de pesquisa: Brasil, Estados Unidos, Europa/PCT

## 1. Problema técnico

Representar fala digital inteligível em baixa taxa com:
- baixa latência;
- baixo CPU/RAM;
- robustez a perda;
- bitstream e implementação próprios;
- nenhuma dependência interna de AMBE/AMBE+2.

## 2. Pesquisa executada

Fontes oficiais/metodologia:
- WIPO FTO/public-domain guidance;
- WIPO PATENTSCOPE;
- EPO Espacenet;
- USPTO Patent Public Search;
- INPI Brasil, diretrizes para invenções implementadas em computador.

Leads de famílias encontrados em pesquisa preliminar:
- US8359197B2 — Half-rate vocoder — Digital Voice Systems, Inc.;
- CA2461704A1/C — pitch, voicing and/or gain bits — Digital Voice Systems, Inc.;
- US11270714B2 / WO2021142198 — Speech Coding Using Time-Varying Interpolation — Digital Voice Systems, Inc.;
- US12462814B2 / WO2025076379 — Bit error correction in digital speech — Digital Voice Systems, Inc.;
- famílias históricas de speech model / analysis / synthesis / quantization e interoperable vocoder.

Os resultados acima são **leads de pesquisa**, não parecer jurídico. Status/família/jurisdição devem ser confirmados em bases oficiais antes de qualquer decisão comercial.

## 3. Risco identificado no HVC v0

O HVC v0 atual transmite explicitamente:
- voiced flag;
- pitch lag;
- pitch confidence;
- energy;
- parâmetros espectrais.

Essa combinação é tecnicamente próxima de temas presentes em famílias históricas de vocoder pesquisadas.

Isso **não estabelece infração**, porém viola nossa nova tolerância conservadora para risco de patente como candidato final.

Conclusão:
- HVC v0 permanece válido como baseline de pesquisa/benchmark;
- HVC v0 **não será promovido a produção**;
- não congelar seu formato como contrato de longo prazo;
- próxima geração deve usar representação substancialmente diferente.

## 4. Mapa do que evitar no próximo HVC

Não usar como núcleo do bitstream:
- pitch lag explícito;
- voiced/unvoiced flag explícito;
- combinação explícita pitch + voicing + gain como estrutura principal;
- magnitude harmônica / modelo multi-band excitation semelhante a MBE;
- AMBE/AMBE+2 frame layout, tables, codebooks, bit allocation ou parameter ordering;
- time-varying interpolation copiada de solução de terceiros;
- bit-error protection layout copiado de família de terceiros;
- qualquer tabela/constante derivada de patente/firmware/software proprietário.

Não usar patente como receita de implementação. Claims servem para criar restrições/avoidance map.

## 5. Direção de design independente

Próxima fase deve comparar candidatos que operam sobre **waveform/transform residual**, não sobre representação AMBE-like.

Critérios obrigatórios:
- bitstream novo/versionado;
- nenhuma compatibilidade bit-a-bit com AMBE;
- transform/quantization/framing próprios;
- ausência de campos explícitos pitch/voicing;
- código escrito do zero;
- parâmetros obtidos por experimentos próprios sobre corpus legal;
- sem codebook/tabela copiados.

Candidatos de pesquisa:
1. transform-domain waveform coder com transform próprio/sparse lifting;
2. subband residual coder com quantização própria;
3. stochastic spectral-envelope coder sem pitch explícito.

Cada candidato exige patent review específico antes de promoção.

## 6. Eficiência alvo

Medir sempre:
- realtime factor;
- CPU;
- RSS;
- tamanho do artefato;
- bitrate;
- clipping;
- STOI/eSTOI;
- distância espectral;
- perda de pacote;
- energia no hardware-alvo quando disponível.

Meta: melhorar qualidade **sem perder** o baseline conhecido de leveza/estabilidade.

## 7. Gate

- [x] proveniência da pesquisa registrada
- [x] risco preliminar identificado
- [x] HVC v0 congelado como pesquisa
- [x] design-around de alto nível definido
- [ ] busca oficial completa por família/claims/status
- [ ] revisão por território de comercialização
- [ ] candidato v1 implementado e medido
- [ ] benchmark comparativo v0 vs v1
- [ ] revisão jurídica especializada antes de declaração comercial de FTO

## 8. Conclusão técnica

**UNCERTAIN**

Não promover HVC v0 a produção. Prosseguir com design clean-slate substancialmente diferente e repetir FTO técnico por candidato.

Esta conclusão é de engenharia e não substitui parecer jurídico/FTO profissional.
