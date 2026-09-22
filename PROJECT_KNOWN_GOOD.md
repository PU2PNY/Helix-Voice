# PROJECT_KNOWN_GOOD — Helix Voice

Atualizado: 2026-09-22

Este arquivo registra o **baseline conhecido como bom**. Qualquer agente, IA ou desenvolvedor deve lê-lo antes de alterar código. Um item só entra aqui quando existe evidência correspondente.

## Regra de preservação

1. item PASS não pode ser removido, enfraquecido ou quebrado sem justificativa explícita;
2. mudança que afete um item abaixo exige teste de regressão;
3. SW, ENV, HW e PROD continuam sendo evidências distintas;
4. avaliação qualitativa não substitui o status PASS/FAIL;
5. quando um teste revelar deficiência real, registrar e corrigir em vez de maquiar o resultado.

## Escala qualitativa solicitada

- **ÓTIMO** — atende com folga ao objetivo medido naquele teste;
- **BOM** — atende ao objetivo básico, mas ainda há margem relevante;
- **RUIM** — não atende à meta final e precisa de melhoria;
- **PÉSSIMO** — falha grave/bloqueadora ou comportamento inseguro.

A escala é contextual e deve vir acompanhada do número/teste que a sustenta.

## Known-good atual

| Área | Evidência | Resultado | Avaliação |
|---|---|---|---|
| Rust formatting | GitHub Actions | PASS | ÓTIMO |
| Clippy com warnings como erro | GitHub Actions | PASS | ÓTIMO |
| Workspace tests | GitHub Actions | PASS | ÓTIMO |
| HVC v0 encode/decode | SW | PASS | BOM como protótipo |
| HVC packet CRC/validação | SW | PASS, corrupção de 1 bit rejeitada nas 96 posições | ÓTIMO |
| HVC silêncio | SW | vetor canônico bit-exato | ÓTIMO |
| HVC output bounds | SW + ENV | sem NaN/Inf e headroom 0,98 | ÓTIMO |
| Clipping em fala testada | ENV/WartyWallaby | 0 amostras clipadas | ÓTIMO |
| Resampler 16 kHz → 8 kHz | SW | anti-alias 2:1 com testes de banda | BOM |
| WAV laboratory | SW + ENV | round-trip funcional | ÓTIMO |
| Binários estáticos | CI + ENV | SHA-256 confere na VPS | ÓTIMO |
| helix-daemon self-test | CI + ENV | PASS, rede/XLX desativados | ÓTIMO |
| Desempenho HVC na WartyWallaby | ENV | centenas de vezes tempo real | ÓTIMO |
| PLC | SW + ENV | bounded/fade-to-silence, zero clipping em perdas simuladas | BOM |
| XLXD control framing | SW + ENV | parser/encoder + captura real AMBEDPINGXLX999 | ÓTIMO para a fronteira observada |
| XLX default safety | SW + ENV | Disabled/fail-closed | ÓTIMO |
| Fala humana LibriSpeech | ENV | 20 arquivos, zero clipping | ÓTIMO como cobertura de teste |
| Patent governance | DOC | PATENT_GUARDRAILS + per-feature review mandatory | ÓTIMO como controle de processo |
| Inteligibilidade HVC v0 | ENV, STOI | média 0,601; mínimo 0,355; máximo 0,747 | RUIM para a meta final |
| eSTOI HVC v0 | ENV | média 0,499 | RUIM para a meta final |
| Distância espectral HVC v0 | ENV | média ~12,66 dB no lote humano | RUIM para a meta final |

## Evidências principais

- CI atual do baseline: run 35626402621, commit 7cd5a9be4c2468e9c529646151b734ef09c137f4, conclusão success.
- Artefato estático deste run foi validado na WartyWallaby por SHA-256 e self-tests.
- Corpus humano: Mini LibriSpeech/LibriSpeech, material aberto para teste.
- Avaliação objetiva observada em 20 arquivos: STOI médio 0.6014755333, eSTOI médio 0.4988678182.
- Perdas simuladas avaliadas em fala humana: aproximadamente 1%, 5%, 10% e 20%; zero clipping em todos os lotes testados.
- Captura passiva XLXD de laboratório confirmou UDP/10100 e payload AMBEDPINGXLX999 aproximadamente a cada 5 s.

## Não preservar como “qualidade aprovada”

O bitstream HVC v0 pode permanecer compatível durante a fase de pesquisa somente quando a mudança não exige melhorar sua representação. A meta de qualidade tem precedência sobre congelar um formato experimental ruim.

Não tratar como aprovado:
- naturalidade final do HVC;
- superioridade sobre AMBE, Codec2, Opus ou qualquer outro codec;
- integração de áudio real no XLX026;
- Raspberry Pi/RF;
- produção;
- ABX humano;
- implementação HVC independente interoperável.

## Baselines/rollback

- pre-governance: baseline/pre-governance-2026-09-21
- pre-HVC-v0: baseline/pre-hvc-v0-2026-09-21
- pre-audio-lab: baseline/pre-audio-lab-2026-09-21
- pre-quality-improvement: baseline/pre-quality-improvement-2026-09-22

Ao iniciar trabalho novo, leia este arquivo junto com PROJECT_RELEASE_STATUS.md e PROJECT_TEST_MATRIX.md.


## Regra IP adicionada em 2026-09-22

HVC v0 permanece conhecido-bom apenas como benchmark técnico. Ele não é known-good para FTO comercial. O review docs/patent-reviews/hvc-v0.md está UNCERTAIN e bloqueia sua promoção a produção.

Nenhuma otimização futura pode quebrar PATENT_GUARDRAILS.md.
