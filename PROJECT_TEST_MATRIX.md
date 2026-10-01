# PROJECT_TEST_MATRIX — Helix Voice

Legenda:
- **DOC** documentação/código verificado;
- **SW** teste automatizado/local;
- **ENV** staging/VPS reproduzível;
- **HW** hardware real;
- **PROD** produção.

Status: PASS / FAIL / PARCIAL / PENDENTE.

## Baseline automatizada

| ID | Requisito | Teste/evidência | Observado | Evidência | Commit | Estado |
|---|---|---|---|---|---|---|
| T-QA-001 | REL-002 | cargo fmt --all -- --check | success | SW | 7cd5a9b | PASS |
| T-QA-002 | SEC-001 / QA | cargo clippy --workspace --all-targets -- -D warnings | success | SW | 7cd5a9b | PASS |
| T-QA-003 | TEST-003 | cargo test --workspace | success | SW | 7cd5a9b | PASS |
| T-CORE-001 | CORE-001/003 | frame fixo preserva samples/rate/timestamp | success | SW | 7cd5a9b | PASS |
| T-CORE-002 | CORE-002 | empty/too-large/sample-rate inválido | rejeitados | SW | 7cd5a9b | PASS |
| T-DSP-001 | DSP-004 | limiter não ultrapassa full scale | success | SW | 7cd5a9b | PASS |
| T-DSP-002 | DSP-003 | silêncio não provoca runaway do AGC | success | SW | 7cd5a9b | PASS |
| T-DSP-003 | DSP-001/003/004 | cadeia produz saída finita/bounded | success | SW | 7cd5a9b | PASS |
| T-DSP-004 | DSP-003 | AGC converge em fala sintética baixa/alta | gain converge ~4,0 e ~0,26 | SW | d22e841 | PASS |
| T-DSP-005A | DSP-003 | silêncio após ganho acumulado | 500 frames sem drift/runaway | SW | d22e841 | PASS |
| T-DSP-007A | DSP-004/007 | limiter abaixo do knee | saída exatamente 1:1 | SW | d22e841 | PASS |
| T-DSP-007B | DSP-004/007 | limiter acima do knee | monotônico, preserva sinal, <= full scale | SW | d22e841 | PASS |
| T-DSP-008 | DSP-006 | decimator 16k→8k preserva banda de fala | teste de 1 kHz | SW | 7cd5a9b | PASS |
| T-DSP-009 | DSP-006 | decimator suprime alias fora da banda | teste de 6 kHz | SW | 7cd5a9b | PASS |
| T-HVC-001 | HVC-002/004 | encode→packet→decode | success | SW | 7cd5a9b | PASS |
| T-HVC-002 | HVC-002/004 | silêncio com vetor canônico | bit-exato | SW | 7cd5a9b | PASS |
| T-HVC-007 | HVC-002/004 | corrupção de 1 bit nas 96 posições | todas rejeitadas | SW | 7cd5a9b | PASS |
| T-HVC-008 | HVC-005 | stream stateful prolongado | finito/bounded | SW | 7cd5a9b | PASS |
| T-HVC-009 | HVC-005 | headroom decoder | pico <= 0,98 | SW + ENV | 7cd5a9b | PASS |
| T-HVC-010 | HVC-005 | PLC bounded/fade | converge para silêncio | SW | 7cd5a9b | PASS |
| T-LAB-001 | TEST-001 | WAV read/write + roundtrip | success | SW + ENV | 7cd5a9b | PASS |
| T-LAB-002 | PERF-001 | benchmark HVC | centenas x realtime | ENV | 7cd5a9b | PASS |
| T-DAEMON-001 | DEP/REL | --self-test | PASS, rede desativada | SW + ENV | 7cd5a9b | PASS |
| T-DAEMON-002 | DEP/REL | --health | ready, xlx/network disabled | ENV | 7cd5a9b | PASS |
| T-ART-001 | REL | SHA-256 artefatos musl | confere byte-a-byte | ENV | 7cd5a9b | PASS |
| T-XLX-001 | XLX-001 | default Disabled | success | SW + ENV | 7cd5a9b | PASS |
| T-XLX-006 | XLX-002 | parser/encoder controle público | round-trip e validação | SW | 7cd5a9b | PASS |
| T-XLX-007 | XLX-002 | captura passiva UDP/10100 | AMBEDPINGXLX999 observado | ENV | 7cd5a9b | PASS |
| T-SEC-002 | SEC-005 | mutação determinística parser XLX | ~33 mil entradas sem panic | SW | d22e841 | PASS |
| T-SEC-003 | SEC-005 | mutação determinística pacote HVC | 20 mil entradas sem panic | SW | d22e841 | PASS |
| T-GOV-001 | IP-005/006/007 | scripts/check-governance.sh | patent/known-good gates executados antes do Rust CI | SW | d451a6e+ | PASS |

CI do baseline HVC/ENV: GitHub Actions run **35626402621**, conclusão success.

CI das correções DSP/robustez: run **35730271351**, commit **d22e84157c60bd361cbb15360d58134515ce0274**, conclusão success.

## Qualidade objetiva HVC

| ID | Requisito | Teste | Resultado | Evidência | Estado |
|---|---|---|---|---|---|
| T-AUDIO-003 | HVC-005 | 20 arquivos Mini LibriSpeech | 0 clipping | ENV | PASS |
| T-AUDIO-004 | HVC-005 | STOI em 20 arquivos | média 0,601; min 0,355; max 0,747 | ENV | FAIL para meta final |
| T-AUDIO-005 | HVC-005 | eSTOI em 20 arquivos | média 0,499 | ENV | FAIL para meta final |
| T-AUDIO-006 | HVC-005 | distância espectral por 8 bandas | média ~12,66 dB | ENV | FAIL para meta final |
| T-HVC-006 | HVC-005 | PLC com ~1/5/10/20% de perda | estável, zero clipping | SW + ENV | PARCIAL |

A palavra FAIL acima significa: **o protótipo não atingiu a meta final de qualidade**, não que o programa tenha crashado.

## Ainda necessários

| ID | Requisito | Teste necessário | Evidência alvo | Estado |
|---|---|---|---|---|
| T-DSP-005B | DSP-003 | fala/ruído reais em sessão longa | ENV + áudio | PENDENTE |
| T-DSP-006 | DSP-003 | pumping/breathing | SW + escuta | PENDENTE |
| T-DSP-007C | DSP-004/007 | THD/qualidade perceptual acima do knee | SW + áudio | PENDENTE |
| T-PERF-001 | PERF-001 | CPU/RSS com coleta dedicada | ENV/HW | PARCIAL |
| T-PERF-002 | PERF-001/002 | latência por estágio | ENV/HW | PENDENTE |
| T-AUDIO-001 | DSP-005/007 | nível percebido entre caminhos | HW | PENDENTE |
| T-AUDIO-002 | TEST-004 | A/B ou ABX level-matched | HW/humano | PENDENTE |
| T-XLX-002 | XLX-002 | processo observe-only completo sem alterar áudio | ENV | PARCIAL |
| T-XLX-003 | XLX-005 | soak 24 h | ENV | PENDENTE |
| T-XLX-004 | XLX-005 | rollback determinístico | ENV | PENDENTE |
| T-XLX-005 | XLX-005 | falha Helix não afeta XLXD | ENV | PENDENTE |
| T-PNY-001 | PNY-001/004 | boot PU2PNY sem Helix | HW | PENDENTE |
| T-PNY-002 | PNY-003 | estados UI reais | HW | PENDENTE |
| T-PNY-003 | PNY-005 | RF real | HW | PENDENTE |
| T-SEC-001 | SEC-005 | coverage-guided fuzzing parsers | SW | PARCIAL — mutation tests PASS, fuzzer dedicado pendente |
| T-HVC-003 | HVC-002 | segunda implementação independente | SW | PENDENTE |
| T-HVC-011 | HVC-005 | qualidade melhorada com nova representação/excitação | SW + ENV | EM DESENVOLVIMENTO |

## Regra

Nenhum item PENDENTE/PARCIAL pode ser promovido por inferência. Mudanças no HVC só são aceitas se melhorarem a evidência de qualidade sem regredir itens PASS de PROJECT_KNOWN_GOOD.md.

| T-XLX-008 | XLX-006/007/008 | HXP1 local + observer + fallback externo em transcoder de laboratório | off/fallback/shadow bit-idênticos; shadow 40/40; process 40/40; 2 streams 60/60 com fallback seguro de um stream | ENV | PASS |
| T-XLX-009 | PERF-001 | latência HXP1 request/reply, 2.000 frames | p50 0,085 ms; p95 0,319 ms; p99 0,817 ms; p99,9 1,750 ms; max 3,653 ms; 0 >5 ms | ENV | PASS |

| T-XLX-010 | XLX-005/007/008 / PERF-001 | Repetibilidade do adapter externo em `process` multi-stream | Installer ENV: todos os runs entregam 40/40 frames por stream e 0 falhas de codec; respostas Helix antes de fallback variam (4/5, 35/34, 2/28, 18/14). Continuidade PASS; confiabilidade sob contenção PARCIAL; causa raiz ainda não provada | ENV | PARCIAL |
| T-DSP-010 | DSP-003/007/008 | Perfil XLX conservador: nominal próximo de unidade, boost/attenuation bounded, silêncio sem drift, limiter 1:1 abaixo do knee, saída finita | SW: 14/14 helix-dsp PASS; ENV probe: nominal 1,000× estável; baixo 1,011→1,456× vs legado genérico 1,470→4,000×; alto após trecho baixo 1,265→0,500× vs 1,980→0,250×; daemon self-test PASS | PASS (SW/ENV) |
| T-XLX-011 | DSP-008 / XLX-005/008 | PCM bridge usa perfil XLX separado; HVC/default permanece inalterado; latência/CPU e fallback não regressam | ENV: paced HXP1 3.000 frames = 2.994 respostas, 6 timeouts isolados, max consecutive 1. E2E com xuvd bounded: single 3.000 frames = 2.988 Helix/12 fallback, failures=0; multi 2×1.500 = 1.488/1.492 Helix, 12/8 fallback, failures=0, consecutive max 1, nenhum stream desabilitado | PASS (SW/ENV) |
| T-AUDIO-007 | DSP-007/008 / TEST-004 | Cross-mode D-Star↔AMBE+2 em RF real, comparação com shadow conhecido-bom | HW/PROD + OPERATOR | PENDENTE |
