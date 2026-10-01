# PROJECT_RELEASE_STATUS — Helix Voice

Atualizado: 2026-09-30

## Estado atual

- **Estágio:** DEV / pesquisa
- **Versão de workspace:** 0.0.1
- **Branch:** main
- **Baseline HVC/ENV medido:** 7cd5a9be4c2468e9c529646151b734ef09c137f4
- **Baseline DSP/robustez validado:** d22e84157c60bd361cbb15360d58134515ce0274
- **Baseline conhecido-bom consolidado:** 9f09b55c48341a017d4d361236bfcc0fc998e559
- **Rollback conhecido-bom:** baseline/known-good-2026-09-22
- **Rollback antes da nova fase de qualidade:** baseline/pre-quality-improvement-2026-09-22
- **Produção:** não
- **PCM bridge XLX:** integrado em `main` como código local/default-disabled no commit `e80969d58d0ecf0f4bd55bbc7fae85311c0176d2`; integração de áudio XLX026 em produção continua PENDENTE
- **Integração real PU2PNY-OS:** PENDENTE
- **HVC:** v0 experimental funcional em SW/ENV; qualidade objetiva ainda abaixo da meta final
- **Patent/FTO:** governança obrigatória ativa; HVC v0 classificado tecnicamente como UNCERTAIN para produção/comercialização

## Known-good / preservar

- workspace Rust com CI;
- helix-core e validação de frames;
- helix-dsp básico;
- AGC com convergência sintética e silêncio sem drift testados;
- SoftLimiter neutro 1:1 abaixo do knee 0,82 e monotônico acima do knee;
- resampler anti-alias 16 kHz → 8 kHz;
- helix-hvc v0 encoder/decoder;
- bitstream próprio de 12 bytes / 4.800 bit/s;
- CRC-8 e rejeição de corrupção;
- vetor canônico de silêncio;
- decoder stateful;
- headroom de saída 0,98;
- PLC bounded com fade-to-silence;
- WAV laboratory;
- benchmark sintético;
- avaliação de fala humana;
- helix-daemon com --health e --self-test;
- XLX fail-closed/Disabled por padrão;
- parser/encoder do protocolo público de controle XLXD;
- binários estáticos Linux x86_64-musl;
- validação SHA-256 na WartyWallaby.

Detalhes obrigatórios: ver PROJECT_KNOWN_GOOD.md.

## Evidência CI atual

Baseline HVC/ENV: run **35626402621**, commit **7cd5a9be4c2468e9c529646151b734ef09c137f4**.

Correções DSP/robustez: run **35730271351**, commit **d22e84157c60bd361cbb15360d58134515ce0274**.

Consolidação canônica: run **35731034369**, commit **9f09b55c48341a017d4d361236bfcc0fc998e559**, success completo

Resultado:
- rustfmt: PASS;
- clippy -D warnings: PASS;
- cargo test workspace: PASS;
- helix-lab self-test: PASS;
- helix-daemon self-test: PASS;
- artefato estático: gerado e validado.

Avaliação: **ÓTIMO** para integridade SW/CI.

## Evidência ENV — WartyWallaby

### Integridade de artefatos
- helix-lab SHA-256: confere;
- helix-daemon SHA-256: confere;
- self-tests: PASS;
- daemon health: ready;
- network: disabled;
- XLX: disabled.

Avaliação: **ÓTIMO**.

### Desempenho
Em execuções do HVC na VPS de aproximadamente 1 GiB:
- centenas de vezes mais rápido que tempo real;
- lotes de fala humana tipicamente entre ~300x e ~680x em execuções registradas;
- sem dependência de runtime dinâmico no artefato musl.

Avaliação: **ÓTIMO** para desempenho de software no host testado.

### Clipping/headroom
Após correção de normalização:
- fala PT-BR sintética: 0 clipping;
- fala EN sintética: 0 clipping;
- 20 arquivos humanos Mini LibriSpeech: 0 clipping;
- pico máximo limitado a aproximadamente 0,98.

Avaliação: **ÓTIMO**.

### Fala humana / qualidade objetiva
Lote: 20 arquivos Mini LibriSpeech.

Resultados:
- STOI médio: 0.6014755333;
- STOI mediano: 0.6114377513;
- STOI mínimo: 0.3545976839;
- STOI máximo: 0.7468197758;
- eSTOI médio: 0.4988678182;
- distância espectral média observada: ~12,66 dB.

Avaliação: **RUIM para a meta final de áudio impecável**. Isso é agora o principal alvo técnico.

### PLC / perdas simuladas em fala humana
Lotes avaliados em aproximadamente 1%, 5%, 10% e 20% de perda:
- zero clipping;
- saída finita/bounded;
- degradação não causou instabilidade;
- ~21% de perda observada no lote de 20% continuou processando sem falha.

Avaliação:
- estabilidade: **ÓTIMO**;
- qualidade perceptual sob perda: **BOM/INCONCLUSIVO** até comparação objetiva específica de perda.

### XLXD de laboratório
Captura passiva real na WartyWallaby:
- UDP 10100;
- payload observado: AMBEDPINGXLX999;
- intervalo aproximado: 5 s;
- zero alteração de áudio/configuração do XLXD.

O vetor capturado foi incorporado aos testes do parser XLX.

Avaliação: **ÓTIMO para a fronteira observada**, mas ainda não prova integração de áudio/transcoding.

## Parcial / precisa melhorar

- HVC naturalidade/inteligibilidade;
- modelo de excitação voiced/unvoiced;
- resolução/representação espectral do HVC;
- AGC em fala real;
- limiter: THD/escuta acima do knee ainda pendentes;
- jitter buffer;
- coverage-guided fuzzing (mutation hardening já PASS);
- métricas de latência por estágio;
- interface real para backend externo licenciado;
- processo XLX observe-only completo;
- segunda implementação HVC independente.

## Pendente por exigir evidência adicional

- A/B ou ABX humano level-matched;
- comparação controlada com codecs de referência;
- soak de 24 h em tempo real;
- Raspberry Pi;
- rádio/RF real;
- PU2PNY-OS;
- XLX026 produção;
- licença final do projeto;
- interoperabilidade HVC independente.

## Bloqueadores de produção

1. qualidade HVC ainda abaixo da meta;
2. HVC v0 com patent/FTO técnico UNCERTAIN; não promover como candidato final;
3. ausência de ABX humano;
4. ausência de segunda implementação HVC;
5. ausência de integração de áudio XLX com backend autorizado;
6. ausência de 24 h soak;
7. ausência de evidência HW/RF;
8. licença final ainda não definida.

## Próxima prioridade

1. concluir pesquisa patent/FTO por candidato e escolher arquitetura clean-slate;
2. elevar STOI/eSTOI sem quebrar known-good;
3. repetir os 20 arquivos humanos após cada mudança;
4. somente aceitar mudança de codec se qualidade melhorar e clipping/estabilidade continuarem PASS;
5. medir THD/escuta do limiter e fala real do AGC;
6. avançar para XLX observe-only completo;
7. manter produção bloqueada até os gates IP/HW/PROD.


## Trabalho isolado — XLX local PCM bridge V1
Branch de origem: `feature/xlx-legacy-pcm-bridge-v1-20260930`; PR #1 mesclada em `main` como `e80969d58d0ecf0f4bd55bbc7fae85311c0176d2`.

Implementado em código:
- protocolo PCM16 local `HXP1` v1 com buffers limitados;
- Unix stream request/reply e Unix datagram one-way locais no `helix-daemon`;
- estado DSP por stream e reset explícito;
- observe/bit-exact quando DSP não é solicitado;
- processamento somente quando solicitado;
- XLX continua Disabled por padrão.

O backend AMBE/AMBE+2 permanece externo ao Helix. Nenhuma integração de áudio de produção foi promovida por esta branch. `process` continua bloqueado pelos gates de áudio/ENV/rollback e pelos gates IP aplicáveis aos componentes DSP.


### Evidência ENV — XLX PCM bridge V1 (2026-09-30)

WartyWallaby:
- `helix-daemon --self-test`: PASS;
- contrato PCM HXP1 e observer não bloqueante: PASS;
- bridge externo XLX exercitado com AMBE+2 válido gerado a partir de PCM;
- `shadow` recebeu 40/40 frames sem alterar a saída do transcoder;
- `process` recebeu/processou 40/40 frames numa execução single-stream, produzindo saída distinta do baseline;
- Helix ausente foi tratado pelo adapter externo com fallback bit-idêntico ao baseline;
- dois streams intercalados mantiveram entrega de 60/60 frames; um timeout do Helix foi contido pelo adapter externo sem falha do transcoder;
- request/reply HXP1 direto, 2.000 frames: p50 0,085 ms; p95 0,319 ms; p99 0,817 ms; p99,9 1,750 ms; máximo 3,653 ms; nenhum acima de 5 ms.

Classificação: `ENV PASS` para IPC local, observer, DSP request/reply e isolamento/fallback demonstrado pelo adapter externo. Isso **não** promove `process` para PROD e não altera os bloqueadores de áudio real, soak, rollback, HW nem FTO.


### Evidência posterior do adapter XLX externo — repetibilidade de `process`
O adapter externo vive no repositório `PU2PNY/XLX-Modern-Installer` e foi mesclado como código experimental/default-off pela PR #65 (`main` `15a1612a720bbab4882bf1b56f4584301a2ec16b`). A evidência posterior preserva 40/40 frames por stream e zero falha de codec, mas a quantidade de respostas Helix antes do fallback sticky variou entre execuções: wrapper 4/5 e três repetições adicionais 35/34, 2/28 e 18/14. Portanto:
- continuidade/fail-open: **PASS (ENV)**;
- isolamento/estado por stream: **PASS (ENV)**;
- confiabilidade de `process` sob contenção: **PARCIAL (ENV)**;
- produção: **BLOQUEADA**.

O limite total de 1..5 ms não deve ser ampliado para mascarar essa variabilidade. Scheduling/contenção permanece hipótese, não causa raiz provada. `shadow` e `process` continuam sujeitos a soak, rollback completo, áudio/HW e gates IP aplicáveis antes de qualquer promoção PROD.

Fonte canônica do adapter e evidências: https://github.com/PU2PNY/XLX-Modern-Installer/tree/main/docs/evidence

## 2026-10-01 — XLX cross-mode safe DSP candidate
Após o operador relatar áudio estranho no cross-mode D-Star↔YSF durante um teste real de Helix `process`, o XLX026 foi retornado a `shadow` antes de qualquer nova alteração. A comparação de logs mostrou que falhas de decode D-Star 1→2 também existiam no baseline `shadow`, portanto não foram atribuídas ao DSP sem evidência. O diferencial confirmado do teste `process` era o perfil genérico de nível: `AdaptiveGain::default()` permitia 0,25×..4,0× e limiter knee 0,82.

Foi criado um perfil separado `DspChain::xlx_crossmode()` sem alterar `DspChain::default()`: target RMS 0,10, ganho 0,50×..1,50×, silence RMS 0,006, attack 0,20, release 0,02 e limiter knee 0,95. O algoritmo continua AdaptiveGain + SoftLimiter; o status FTO existente permanece inalterado.

Evidência SW/ENV: helix-dsp 14/14 PASS, helix-daemon 5/5 PASS, clippy -D warnings PASS, daemon self-test PASS. Probe direto mostrou nominal 1,000× estável; trecho baixo ficou limitado a ~1,456× em vez de 4,0×; transição baixo→alto iniciou em ~1,265× em vez de ~1,980×. E2E com o xuvd bounded processou 3.000 frames single e 2×1.500 multi com zero falha de codec, `consecutive_max=1` e nenhum stream desabilitado. SHA do daemon release ENV: `63f51d7bb2e18ad4c151fe571f3a3120e746f3d3858514e0d2f3ddeae1787574`.

A classificação permanece **SW/ENV**. O áudio RF real do novo perfil ainda é PENDENTE; produção permanece em `shadow` e nenhuma melhora auditiva é declarada por inferência.
