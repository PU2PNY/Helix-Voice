# PROJECT_RELEASE_STATUS — Helix Voice

Atualizado: 2026-09-22

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
- **Integração real XLX026:** PENDENTE
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
