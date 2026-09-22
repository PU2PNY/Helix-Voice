# HVC_NEXT — Clean-slate quality research plan

Status: research only
Data: 2026-09-22

## Regra

O HVC v0 permanece apenas como baseline de benchmark. O próximo HVC não deve evoluir incrementalmente a estrutura pitch/voicing/gain do v0.

## Objetivo

Encontrar uma representação própria que:
- aumente STOI/eSTOI significativamente;
- preserve zero clipping;
- permaneça centenas de vezes mais rápida que tempo real em x86_64 de baixo recurso, quando possível;
- mantenha memória pequena;
- permita implementação ARM eficiente;
- tenha bitstream próprio;
- reduza risco de sobreposição com famílias de vocoder paramétrico pesquisadas.

## Restrições de arquitetura

Proibido no candidato final desta fase:
- voiced flag explícito;
- pitch lag transmitido;
- harmonic magnitude vector;
- MBE-like band voicing map;
- AMBE-compatible ordering/framing/codebooks;
- parâmetros/tabelas obtidos de patente, firmware ou codec fechado.

## Candidatos

### Candidate A — sparse lifting transform
- waveform-domain;
- blocos pequenos;
- transform invertível próprio composto por lifting/add/subtract;
- coeficientes/ordem definidos pelo Helix;
- quantização própria;
- sem pitch detector.

### Candidate B — subband residual
- filterbank próprio/public-domain;
- residual por subband;
- bit allocation derivado de medidas do corpus Helix;
- sem modelo harmônico.

### Candidate C — stochastic spectral envelope
- envelope de baixa dimensão;
- excitação determinística própria sem pitch explícito;
- parâmetros/framing próprios.

## Torneio de engenharia

Cada candidato deve rodar no mesmo corpus Mini LibriSpeech e ser comparado com HVC v0.

Critérios eliminatórios:
- CI vermelho;
- clipping > 0;
- NaN/Inf;
- regressão grave de memória/CPU;
- patente review UNCERTAIN com overlap não resolvido.

Critérios de seleção:
1. STOI/eSTOI;
2. naturalidade/ABX futuro;
3. bitrate;
4. CPU/RSS;
5. latência;
6. robustez a perda;
7. simplicidade e auditabilidade;
8. risco IP residual.

## Promoção

Nenhum candidato vira HVC v1 antes de:
- patent review próprio;
- corpus real;
- métricas reproduzíveis;
- test vectors próprios;
- documentação de bitstream;
- segunda implementação de conformidade planejada.
