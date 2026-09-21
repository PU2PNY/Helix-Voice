# PROJECT_TEST_MATRIX — Helix Voice

Legenda de evidência:
- **DOC** documentação/código verificado;
- **SW** teste automatizado/local;
- **ENV** staging/VPS reproduzível;
- **HW** hardware real;
- **PROD** produção.

Status permitidos: PASS / FAIL / PARCIAL / PENDENTE.

## Baseline automatizada

| ID | Requisito | Teste | Esperado | Observado | Evidência | Ambiente | Commit | Estado |
|---|---|---|---|---|---|---|---|---|
| T-QA-001 | REL-002 | `cargo fmt --all -- --check` | sem diff | GitHub Actions success | SW | ubuntu-latest | 62ee9b8 | PASS |
| T-QA-002 | SEC-001 / QA | `cargo clippy --workspace --all-targets -- -D warnings` | zero warnings/error | GitHub Actions success | SW | ubuntu-latest | 62ee9b8 | PASS |
| T-QA-003 | TEST-003 | `cargo test --workspace` | todos unitários passam | GitHub Actions success | SW | ubuntu-latest | 62ee9b8 | PASS |
| T-CORE-001 | CORE-001/003 | `frame_is_fixed_capacity_and_preserves_samples` | preserva amostras/rate/timestamp | passou dentro de cargo test | SW | ubuntu-latest | 62ee9b8 | PASS |
| T-DSP-001 | DSP-004 | `limiter_never_exceeds_full_scale` | abs(sample) <= 1.0 | passou | SW | ubuntu-latest | 62ee9b8 | PASS |
| T-DSP-002 | DSP-003 | `agc_does_not_raise_digital_silence` | silêncio permanece zero, gain 1.0 | passou | SW | ubuntu-latest | 62ee9b8 | PASS |
| T-DSP-003 | DSP-001/003/004 | `chain_processes_fixed_frame` | cadeia produz RMS > 0 e pico <= 1 | passou | SW | ubuntu-latest | 62ee9b8 | PASS |
| T-XLX-001 | XLX-001 | `integration_is_disabled_by_default` | modo padrão Disabled | passou | SW | ubuntu-latest | 62ee9b8 | PASS |

Evidência CI: GitHub Actions **Rust CI**, run `35601721908`, conclusão `success`.

## Testes de comportamento ainda necessários

| ID | Requisito | Teste necessário | Evidência alvo | Estado |
|---|---|---|---|---|
| T-CORE-002 | CORE-002 | rejeitar empty/too-large/sample-rate inválido | SW | PENDENTE |
| T-DSP-004 | DSP-003 | convergência do AGC para fala baixa/alta | SW | PENDENTE |
| T-DSP-005 | DSP-003 | ruído/silêncio não provocar runaway gain | SW + áudio | PENDENTE |
| T-DSP-006 | DSP-003 | pumping/breathing em sinais modulados | SW + escuta | PENDENTE |
| T-DSP-007 | DSP-004/007 | THD/qualidade do limiter | SW | PENDENTE |
| T-PERF-001 | PERF-001 | CPU e RSS idle/active | ENV/HW | PENDENTE |
| T-PERF-002 | PERF-001/002 | latência end-to-end por estágio | ENV/HW | PENDENTE |
| T-AUDIO-001 | DSP-005/007 | nível percebido entre caminhos | HW | PENDENTE |
| T-AUDIO-002 | TEST-004 | A/B ou ABX level-matched | HW | PENDENTE |
| T-XLX-002 | XLX-002 | observe-only sem alterar áudio | ENV | PENDENTE |
| T-XLX-003 | XLX-005 | soak 24 h | ENV | PENDENTE |
| T-XLX-004 | XLX-005 | rollback determinístico | ENV | PENDENTE |
| T-XLX-005 | XLX-005 | falha Helix não derruba/corrompe XLXD | ENV | PENDENTE |
| T-PNY-001 | PNY-001/004 | boot PU2PNY sem Helix | HW | PENDENTE |
| T-PNY-002 | PNY-003 | estados UI refletem backend real | HW | PENDENTE |
| T-PNY-003 | PNY-005 | RF real | HW | PENDENTE |
| T-SEC-001 | SEC-005 | fuzzing de parsers | SW | PENDENTE |
| T-HVC-001 | HVC-002 | encode/decode + vetores + implementação independente | SW/HW | PENDENTE |

## Regra

Nenhum item PENDENTE acima pode ser descrito como funcional, compatível ou aprovado apenas porque a arquitetura o prevê.
