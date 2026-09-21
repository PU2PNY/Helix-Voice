# PROJECT_RELEASE_STATUS — Helix Voice

Atualizado: 2026-09-21

## Estado atual

- **Estágio:** DEV / pesquisa
- **Versão de workspace:** `0.0.1`
- **Branch de trabalho identificada:** `main`
- **Baseline de código auditada:** `62ee9b83ee72ff7ba721413f9f3bdd56f620c0c5`
- **Rollback da baseline:** `baseline/pre-governance-2026-09-21`
- **Releases GitHub:** nenhuma encontrada na auditoria
- **Issues abertas:** nenhuma encontrada
- **PRs:** não verificado por endpoint dedicado; nenhuma apareceu na busca de issues/PRs acessível
- **Tags:** não verificado pelo conector disponível
- **Produção:** não
- **Integração real XLX026:** PENDENTE
- **Integração real PU2PNY-OS:** PENDENTE
- **HVC:** pesquisa futura, ainda sem bitstream

## Concluído com evidência SW

- workspace Rust criado;
- `helix-core`;
- frame PCM fixo;
- validação básica de frame;
- tipagem de codec;
- `helix-dsp`;
- medidor RMS/pico;
- protótipo de AdaptiveGain;
- protótipo de SoftLimiter;
- `helix-xlx` com modo padrão `Disabled`;
- `helix-daemon` inicial;
- CI com rustfmt, clippy e cargo test;
- CI final da baseline passou no run `35601721908`;
- documentação clean-room, arquitetura, segurança e integrações;
- governança persistente criada.

## Parcial / protótipo

- AGC: algoritmo inicial existe, mas qualidade perceptual, constantes, tempo de ataque/release e comportamento em fala real ainda não foram validados.
- Limiter: propriedade de não ultrapassar full scale tem teste; distorção/qualidade ainda não.
- XLX adapter: apenas estrutura/configuração; não há transporte XLXD implementado.
- Daemon: inicialização/estado básico; não é daemon de áudio funcional.
- Audio quality framework: documentação existe; métricas reais ainda não coletadas.

## Pendente

- DC blocker / HPF;
- compressor;
- limiter aprimorado;
- resampler;
- jitter buffer;
- PLC;
- perfis por stream;
- WAV lab;
- benchmark CPU/RSS/latência;
- fuzzing;
- adapter XLXD observe-only;
- interface para backend externo licenciado;
- soak 24 h;
- A/B/ABX;
- integração de laboratório no XLX026;
- serviço systemd no PU2PNY-OS;
- testes em Raspberry Pi;
- testes RF reais;
- análise final de licenças de Codec2/M17/Opus/RNNoise/RADE;
- escolha da licença do próprio Helix;
- especificação HVC.

## Bloqueadores de produção

1. ausência de transcoding real;
2. ausência de testes ENV/HW/PROD;
3. ausência de métricas de desempenho;
4. ausência de testes auditivos;
5. ausência de interface real XLXD;
6. licença final do projeto ainda não definida;
7. HVC ainda não especificado.

## Próxima sequência recomendada

1. estabilizar Phase 1 DSP laboratory;
2. criar testes sintéticos mais rigorosos do AGC/limiter;
3. adicionar benchmark e relatório de nível/clipping;
4. implementar ferramenta WAV offline;
5. somente então iniciar XLX observe-only;
6. não tocar em áudio de produção até cumprir gates XLX.
