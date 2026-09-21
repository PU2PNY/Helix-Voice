# PROJECT_MASTER_SPEC — Helix Voice

Status: **fonte canônica de requisitos**
Projeto: **Helix Voice Engine (HVE)** e pesquisa experimental **Helix Voice Codec (HVC)**

## 1. Objetivo

Construir um motor independente de voz digital, DSP e transcodificação em tempo real, moderno, modular, leve e auditável, com foco inicial em:

- processamento PCM;
- consistência de nível de áudio;
- baixa latência mensurável;
- integração segura e reversível com XLX/XLXD;
- integração opcional com PU2PNY-OS;
- arquitetura de adapters para codecs/protocolos;
- pesquisa e implementação experimental de um codec próprio HVC, separada do caminho de interoperabilidade legado.

## 2. Limite de propriedade intelectual

### IP-001 — Clean-room obrigatório
O Helix deve ser desenvolvido independentemente segundo `CLEAN_ROOM.md`.

### IP-002 — Sem implementação proprietária no núcleo
`helix-core`, `helix-dsp` e HVC não podem incorporar código-fonte proprietário, firmware, tabelas privadas, parâmetros internos, segredos comerciais ou implementação fechada de terceiros.

### IP-003 — Interoperabilidade legada isolada
Quando um sistema legado exigir codec proprietário, o Helix poderá somente usar um backend externo devidamente autorizado/licenciado, atrás de interface explícita.

### IP-004 — Proveniência
Toda dependência ou fonte externa que influencie materialmente uma implementação deve ter proveniência/licença registrada.

## 3. Arquitetura

### ARCH-001 — Separação de camadas
Transporte/protocolo, codec e DSP devem permanecer separados.

### ARCH-002 — PCM canônico
O DSP trabalha sobre representação PCM canônica interna, sem depender diretamente de D-STAR, DMR, YSF ou outro protocolo.

### ARCH-003 — Fronteira de processo para XLX
A primeira integração XLX deve executar em processo separado do XLXD para reduzir blast radius e permitir rollback.

### ARCH-004 — Serviço opcional no PU2PNY-OS
A integração com PU2PNY-OS deve ser opcional e não pode impedir boot, rede, RF, wizard ou painel básico se Helix falhar/estiver ausente.

### ARCH-005 — Buffers limitados
Filas e buffers devem ser limitados; não são permitidas filas de crescimento irrestrito.

### ARCH-006 — Hot path previsível
Evitar alocação heap, I/O bloqueante e logging por amostra no caminho crítico de áudio.

## 4. Núcleo de áudio

### CORE-001 — PcmFrame
O frame canônico inicial deve suportar mono `f32`, 8 kHz a 48 kHz, com capacidade fixa suficiente para 20 ms a 48 kHz.

### CORE-002 — Validação de frame
Frames vazios, grandes demais ou com sample rate fora do intervalo permitido devem ser rejeitados.

### CORE-003 — Timestamp
O frame deve carregar timestamp em unidades de amostra.

### CORE-004 — Tipagem de codec
A API deve distinguir pelo menos:
- codec aberto;
- backend externo licenciado;
- codec não suportado.

## 5. DSP

### DSP-001 — Medição
O motor deve medir RMS e pico.

### DSP-002 — AGC por stream
O AGC deve manter estado por stream, não globalmente entre usuários.

### DSP-003 — Comportamento do AGC
O AGC deve:
- elevar fala baixa gradualmente;
- atenuar nível excessivo mais rápido do que aumenta;
- não aumentar silêncio indefinidamente;
- limitar ganho máximo e mínimo;
- evitar clipping;
- reduzir pumping e breathing audíveis.

### DSP-004 — Limiter
O limiter deve impedir saída além de full scale.

### DSP-005 — Normalização entre caminhos
Ganhos fixos por protocolo não serão o mecanismo principal de normalização. Perfis específicos só podem existir quando justificados por medições reais.

### DSP-006 — Pipeline futuro
Avaliar e, quando aprovado por testes, adicionar:
- DC blocker / high-pass;
- compressor;
- limiter de menor distorção;
- resampler;
- denoise opcional;
- equalização;
- jitter buffer;
- PLC.

### DSP-007 — Qualidade
Uma alteração DSP não pode ser declarada melhor apenas por ser mais alta. Deve considerar nível, distorção, inteligibilidade, ruído, fadiga e latência.

## 6. Desempenho

### PERF-001 — Métricas obrigatórias
Cada caminho suportado deve medir quando aplicável:
- CPU;
- RSS;
- latência de processamento;
- latência de buffering;
- latência de codec;
- jitter;
- clipping;
- RMS;
- pico;
- perda de pacotes.

### PERF-002 — Sem alegação não medida
Termos como "baixa latência", "leve" ou "qualidade superior" não são critérios de aceitação sem dados.

### PERF-003 — ARM
Qualquer habilitação por padrão em Raspberry Pi exige medição no hardware-alvo. ARMv6/Pi Zero original exige validação separada.

## 7. XLX / XLXD

### XLX-001 — Estado padrão seguro
A integração XLX deve iniciar desabilitada.

### XLX-002 — Observe-only primeiro
Primeira etapa de laboratório deve observar/conectar sem transformar áudio de produção.

### XLX-003 — Não remover backend funcional
Backend/hardware/licença existente não pode ser removido antes de backup e validação da alternativa.

### XLX-004 — Backend proprietário externo
Qualquer capacidade proprietária fica fora do núcleo e atrás de backend externo autorizado/licenciado.

### XLX-005 — Gate de produção
Antes de produção:
- soak de 24 h;
- sem crescimento de memória fora do orçamento acordado;
- sem queda do XLXD;
- rollback determinístico testado;
- sem alteração fora do caminho em teste;
- sem regressão de clipping;
- sem regressão material de latência;
- consistência de nível igual ou melhor medida;
- falha do Helix não corrompe configuração XLXD.

## 8. PU2PNY-OS

### PNY-001 — Serviço opcional
Serviço proposto: `helix-voice.service`, opcional.

### PNY-002 — Menor privilégio
Executar com usuário sem privilégio, hardening systemd e IPC local sempre que possível.

### PNY-003 — Estados explícitos
UI só pode mostrar estados confirmados:
- unavailable;
- disabled;
- starting;
- ready;
- degraded;
- error.

### PNY-004 — Não inferir suporte
O painel não deve indicar codec/protocolo como operacional sem backend real detectado e teste correspondente.

### PNY-005 — Gate físico
Integração RF exige teste em hardware real antes de qualquer status de compatibilidade.

## 9. Segurança

### SEC-001 — Unsafe proibido inicialmente
O workspace Rust atual proíbe `unsafe_code`.

### SEC-002 — Entrada não confiável
Pacotes e configuração devem ser validados como entrada não confiável.

### SEC-003 — Segredos
Nunca versionar senha, token, API key, chave privada, certificado privado ou credencial de produção.

### SEC-004 — Superfície local
Interfaces de controle devem ser locais por padrão.

### SEC-005 — Fuzzing
Parsers de protocolos/configuração devem receber fuzzing antes de produção.

## 10. Dependências

### DEP-001 — Dependência mínima
Não adicionar bibliotecas para funções triviais sem justificativa.

### DEP-002 — Revisão prévia
Toda dependência deve ter:
- upstream;
- versão/commit;
- licença;
- manutenção;
- segurança;
- custo de CPU/memória quando relevante;
- finalidade registrada em `THIRD_PARTY.md`.

## 11. Testes e evidência

### TEST-001 — Níveis de evidência
Usar:
- `DOC` — documentação/código inspecionado;
- `SW` — teste automatizado/local;
- `ENV` — ambiente reproduzível/staging/VPS;
- `HW` — hardware real;
- `PROD` — produção.

### TEST-002 — Sem promoção por inferência
`CI PASS != PROD PASS`.
`SW PASS != HW PASS`.

### TEST-003 — Matriz canônica
Resultados devem ser registrados em `PROJECT_TEST_MATRIX.md`.

### TEST-004 — Qualidade auditiva
Quando o pipeline atingir áudio real, incluir A/B ou ABX level-matched com material de teste de origem legalmente adequada.

## 12. HVC

### HVC-001 — Linha de pesquisa separada
HVC é separado do motor de interoperabilidade.

### HVC-002 — Bitstream experimental versionado
O HVC v0 experimental deve possuir especificação publicada no próprio repositório, versão explícita, encoder, decoder e vetores de teste sintéticos próprios/redistribuíveis.

Existência de encoder/decoder da implementação principal não basta para declarar interoperabilidade. Compatibilidade HVC só pode ser alegada após uma segunda implementação independente interpretar os mesmos vetores/bitstream.

### HVC-003 — Design próprio
HVC deve possuir estrutura, framing, parâmetros e implementação próprios e não pode ser apresentado como clone bit-a-bit de codec proprietário.

### HVC-004 — Operating point v0
O HVC v0 de pesquisa usa:
- PCM mono a 8 kHz;
- frames de 20 ms / 160 amostras;
- pacote fixo de 12 bytes;
- taxa de payload de 4.800 bit/s;
- CRC-8;
- análise de energia, pitch/voicing e oito coeficientes de reflexão quantizados.

Mudança incompatível no bitstream exige nova versão.

### HVC-005 — Gate de qualidade
HVC v0 é protótipo SW. Não pode ser descrito como melhor que AMBE, Codec2, Opus ou qualquer outro codec sem:
- corpus legalmente utilizável;
- comparação com níveis casados;
- métricas objetivas adequadas;
- testes auditivos A/B ou ABX;
- CPU/latência/RSS medidos;
- teste em ambiente/hardware alvo quando aplicável.

## 13. Release

### REL-001 — Estado inicial
A série atual é `0.0.x`, estágio DEV/research.

### REL-002 — Critério de promoção
Build/CI verde não promove release automaticamente. Deve existir cadeia:
requisito → implementação → teste → resultado → regressão → pendências.

### REL-003 — Produção
Nenhuma integração deve ser chamada PROD antes de evidência `PROD`.

## 14. Critérios globais de aceitação

Uma funcionalidade só pode ser marcada concluída quando:

1. requisito possui ID;
2. implementação está identificada;
3. teste correspondente existe;
4. resultado e nível de evidência estão registrados;
5. regressões relevantes foram verificadas;
6. status e changelog foram atualizados;
7. qualquer dependência nova possui licença/proveniência registrada.
