# PROJECT_CHANGELOG — Helix Voice

## 2026-09-21 — Fundação do projeto

### Decisão
O projeto foi definido como **Helix Voice Engine (HVE)**, com **Helix Voice Codec (HVC)** como linha futura de pesquisa separada.

### Arquitetura
- separação entre protocolo, codec e DSP;
- PCM canônico interno;
- Rust escolhido para o núcleo inicial;
- processo externo planejado para integração XLX;
- serviço opcional planejado para PU2PNY-OS.

### Clean-room
Criada política formal proibindo incorporação/reprodução de implementação proprietária no núcleo e exigindo backend externo autorizado/licenciado quando necessário para legado.

### Código
Criados:
- `helix-core`;
- `helix-dsp`;
- `helix-xlx`;
- `helix-daemon`.

Implementados protótipos:
- PcmFrame de capacidade fixa;
- RMS/peak meter;
- AdaptiveGain;
- SoftLimiter;
- modo XLX padrão Disabled.

### CI
Criado workflow Rust CI.

Primeira execução falhou em `cargo fmt --check`.
A falha não foi ocultada; os arquivos foram formatados/corrigidos.

A execução final da baseline, run `35601721908`, no commit `62ee9b83ee72ff7ba721413f9f3bdd56f620c0c5`, passou:
- rustfmt;
- clippy;
- cargo test.

### Governança
Criada memória persistente baseada em GitHub:
- `PROJECT_START_HERE.md`;
- `PROJECT_MASTER_SPEC.md`;
- `PROJECT_RELEASE_STATUS.md`;
- `PROJECT_TEST_MATRIX.md`;
- `PROJECT_CHANGELOG.md`;
- decisões/recuperação/bootstrap de IA.

Criado ponto de retorno:
`baseline/pre-governance-2026-09-21` → `62ee9b83ee72ff7ba721413f9f3bdd56f620c0c5`.

### Estado
Nenhuma integração XLX026 ou PU2PNY-OS real foi declarada concluída. Nenhuma evidência HW/PROD existe nesta baseline.

## 2026-09-21 — Regra operacional de execução autônoma

A governança de agentes passou a exigir execução ponta a ponta com intervenção humana mínima quando o objetivo estiver suficientemente definido.

Regras registradas:
- falhas devem ser diagnosticadas, corrigidas e retestadas em vez de encerrar o trabalho na primeira ocorrência;
- dúvidas técnicas verificáveis devem ser resolvidas por pesquisa, inspeção ou teste;
- funcionalidades já comprovadas, requisitos e rollback devem ser preservados;
- nenhuma falha pode ser ocultada ou convertida em sucesso por declaração;
- intervenção humana fica reservada a bloqueios externos reais;
- agentes não podem prometer trabalho em segundo plano nem declarar conclusão sem evidência.


## 2026-09-21 — HVC v0 experimental implementado

### Código
Criada a crate `helix-hvc` com um primeiro codec de fala Helix completo em nível SW:
- entrada PCM mono 8 kHz;
- frame de 20 ms / 160 amostras;
- análise de RMS, pitch/voicing e envelope LPC/reflection;
- bitstream HVC v0 próprio com 12 bytes por frame;
- payload de 4.800 bit/s;
- CRC-8;
- decoder stateful com excitação periódica/ruidosa e síntese LPC;
- normalização de RMS e limitação a full scale;
- sem dependências externas no hot path.

### Bitstream e conformidade
Criados `HVC_SPEC.md` e `HVC_TEST_VECTORS.md`.
O vetor sintético de silêncio é próprio do projeto e redistribuível.

### CI
O primeiro run após adicionar a crate, `35615433080`, falhou em `cargo fmt --check`.
A diferença foi corrigida sem ocultar a falha.

O run `35615775362`, commit `a6400726fe65545f663f009fe237df404918350b`, passou:
- rustfmt;
- clippy com warnings tratados como erro;
- cargo test do workspace.

### Rollback
Criado antes do HVC:
`baseline/pre-hvc-v0-2026-09-21` → `df48685e2d64223d527d06f5e6d936ed459ee4ed`.

### Limites
Esta etapa prova somente evidência SW. Não existe ainda evidência de qualidade perceptual, comparação com AMBE/Codec2/Opus, interoperabilidade independente, RF, HW ou produção.


## 2026-09-22 — Baseline known-good, provas ENV e início da fase de qualidade

### Governança
Criado PROJECT_KNOWN_GOOD.md para registrar explicitamente o que já foi comprovado e não deve regredir.

PROJECT_START_HERE.md passou a exigir leitura desse baseline antes de mudanças.

Criado rollback:
baseline/pre-quality-improvement-2026-09-22 → 7cd5a9be4c2468e9c529646151b734ef09c137f4.

### HVC / DSP / laboratório
Passaram a fazer parte do baseline:
- resampler anti-alias 16 kHz → 8 kHz;
- WAV lab;
- benchmark;
- headroom 0,98;
- zero clipping nos lotes testados;
- PLC bounded com fade-to-silence;
- loss-roundtrip determinístico;
- binários estáticos musl;
- daemon health/self-test fail-closed.

### Evidência de ambiente
Na WartyWallaby:
- artefatos foram validados por SHA-256;
- helix-lab self-test PASS;
- helix-daemon self-test PASS;
- HVC executou centenas de vezes mais rápido que tempo real;
- fala PT-BR/EN e 20 arquivos humanos foram processados sem clipping.

### Qualidade objetiva
Mini LibriSpeech, 20 arquivos:
- STOI médio: 0.6014755333;
- mediana: 0.6114377513;
- mínimo: 0.3545976839;
- máximo: 0.7468197758;
- eSTOI médio: 0.4988678182;
- distância espectral média: ~12,66 dB.

Conclusão: desempenho/estabilidade estão fortes, porém a inteligibilidade/naturalidade do HVC v0 ainda está abaixo da meta final. A qualidade passa a ser o principal alvo de desenvolvimento.

### PLC
Em fala humana com perdas simuladas próximas de 1%, 5%, 10% e 20%:
- zero clipping;
- sem NaN/Inf;
- sem instabilidade;
- comportamento bounded.

### XLXD
Foi observada passivamente a interface real de controle na VPS de laboratório:
- UDP 10100;
- payload AMBEDPINGXLX999;
- aproximadamente a cada 5 s.

O vetor observado foi incorporado ao teste do parser. Nenhum áudio/configuração do XLXD foi alterado.

### CI
Run 35626402621, commit 7cd5a9be4c2468e9c529646151b734ef09c137f4:
- format PASS;
- clippy PASS;
- workspace tests PASS;
- lab self-test PASS;
- daemon self-test PASS;
- artefato estático gerado.

### Próxima fase
Melhorar o HVC com medição objetiva após cada alteração. Nenhuma mudança será mantida se piorar clipping, robustez, CI ou known-good.


## 2026-09-22 — Governança universal de patentes e design-around

### Regra
Criado PATENT_GUARDRAILS.md e docs/PATENT_REVIEW_TEMPLATE.md.

A regra vale para todo o projeto:
- pesquisar patentes/estado da técnica antes de implementar função com risco;
- separar análise de claims da implementação;
- não usar patente como receita;
- criar design-around próprio;
- medir CPU, memória, latência, tamanho e energia quando possível;
- bloquear produção para status FTO técnico UNCERTAIN/BLOCKED;
- não declarar ausência de infração como fato jurídico.

### HVC
Criado docs/patent-reviews/hvc-v0.md.

A pesquisa preliminar encontrou famílias de vocoder relacionadas a temas como pitch/voicing/gain, half-rate coding, interpolation e error protection.

Como o HVC v0 transmite pitch/voicing/energia explicitamente, ele foi congelado como benchmark de pesquisa e recebeu status FTO técnico UNCERTAIN.

Isso não é uma conclusão de infração. É uma decisão conservadora de engenharia.

Criado HVC_NEXT.md:
- próxima geração clean-slate;
- sem pitch lag explícito;
- sem voiced flag explícito;
- sem harmonic magnitude vector/MBE-like map;
- sem framing/codebooks AMBE-compatible;
- cada candidato exige patent review próprio antes de implementação/promoção.

### Rollback
baseline/pre-patent-governance-2026-09-22 aponta para 5ce870f1e9171e0fcf9ef11c70f6bacd5fbcc292.
