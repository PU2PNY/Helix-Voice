# Helix Voice

**Helix Voice Engine (HVE)** é um projeto independente para processamento de voz digital, DSP e transcodificação em tempo real, criado para aplicações de rádio digital, gateways, refletores e sistemas embarcados.

O objetivo é construir um motor **modular, rápido, leve, auditável e desenvolvido do zero**, capaz de oferecer níveis de áudio consistentes, baixa latência e uma arquitetura moderna para integração com sistemas como **XLX/XLXD** e **PU2PNY-OS**.

> **Status:** pesquisa e desenvolvimento inicial.  
> O projeto ainda não deve ser considerado um codec ou transcoder pronto para produção.

---

## Declaração de independência / Clean-room

**Helix Voice Engine (HVE)** e qualquer futuro **Helix Voice Codec (HVC)** são projetos independentes, desenvolvidos com arquitetura, código, DSP, interfaces e algoritmos próprios.

Este projeto:

- **não é AMBE, AMBE+, AMBE+2 ou uma implementação desses produtos;**
- não contém código-fonte proprietário da Digital Voice Systems, Inc. (DVSI);
- não copia tabelas, firmware, modelos, parâmetros internos, algoritmos proprietários ou implementações fechadas da DVSI;
- não utiliza engenharia reversa de software ou hardware proprietário como base para reproduzir uma implementação AMBE;
- não pretende clonar o comportamento interno de um vocoder proprietário;
- não é afiliado, patrocinado, aprovado ou endossado pela DVSI.

O desenvolvimento segue uma abordagem **clean-room**: requisitos de interoperabilidade são tratados separadamente da implementação interna do Helix, e o núcleo é desenvolvido somente a partir de conhecimento público, literatura técnica, especificações abertas e componentes de terceiros com licenças verificadas.

Quando um protocolo legado exigir um codec proprietário, a interoperabilidade deverá ocorrer por **adaptadores externos, hardware autorizado ou componentes devidamente licenciados**, mantendo o núcleo Helix independente da implementação proprietária.

**AMBE®, AMBE+™ e AMBE+2™ são marcas/tecnologias associadas à Digital Voice Systems, Inc. (DVSI).** As referências eventualmente existentes neste repositório destinam-se apenas à identificação de formatos, equipamentos ou requisitos de interoperabilidade de terceiros.

---

## Independent Clean-Room Statement

**Helix Voice Engine (HVE)** and any future **Helix Voice Codec (HVC)** are independently developed projects with their own architecture, source code, DSP pipeline, interfaces and algorithms.

Helix does **not** contain or reproduce proprietary DVSI source code, firmware, internal tables, private parameters, trade secrets or closed AMBE implementations.

Any interoperability with legacy systems requiring proprietary codecs must be provided through an **external authorized/licensed adapter or hardware interface**. Such adapters are not part of the Helix codec design itself.

The goal of Helix is **not to clone an existing vocoder**. The goal is to create a new open architecture for digital voice processing and transcoding.

---

## Objetivos

O Helix está sendo projetado para:

- processamento de áudio em tempo real;
- transcodificação modular entre codecs;
- normalização automática de nível entre protocolos;
- AGC adaptativo sem pumping agressivo;
- compressor de dinâmica;
- soft limiter contra clipping;
- filtros de voz configuráveis;
- resampling de alta qualidade;
- jitter buffer;
- packet-loss concealment quando tecnicamente aplicável;
- métricas de áudio e latência;
- baixo consumo de CPU e memória;
- operação estável 24/7;
- execução em Linux x86_64 e ARM;
- integração futura com XLX026 e PU2PNY-OS.

---

## Arquitetura proposta

O motor trabalha preferencialmente em um domínio de áudio PCM interno e mantém codecs e protocolos separados do DSP.

```text
                    ┌──────────────────────────┐
                    │      Protocol Input      │
                    │ D-STAR / DMR / YSF / ... │
                    └────────────┬─────────────┘
                                 │
                                 ▼
                    ┌──────────────────────────┐
                    │      Codec Adapter       │
                    └────────────┬─────────────┘
                                 │
                                 ▼
                         Canonical PCM
                                 │
                 ┌───────────────┼───────────────┐
                 ▼               ▼               ▼
              Filter            AGC          Denoise
                 │               │               │
                 └───────────────┼───────────────┘
                                 ▼
                          Compressor
                                 │
                                 ▼
                           Soft Limiter
                                 │
                                 ▼
                           Resampler
                                 │
                                 ▼
                    ┌──────────────────────────┐
                    │   Destination Encoder    │
                    └────────────┬─────────────┘
                                 │
                                 ▼
                    ┌──────────────────────────┐
                    │     Protocol Output      │
                    └──────────────────────────┘
```

O DSP não deve depender de D-STAR, DMR, YSF ou qualquer outro protocolo específico.

---

## Componentes previstos

```text
helix-voice/
├── helix-core
├── helix-dsp
├── helix-codecs
├── helix-protocols
├── helix-xlx
├── helix-daemon
├── helix-bench
└── helix-testlab
```

### helix-core

Responsável por:

- streams;
- frames;
- buffers;
- temporização;
- filas;
- concorrência;
- estado do motor.

### helix-dsp

Pipeline próprio de processamento:

- DC blocker / high-pass;
- medição de nível de fala;
- AGC;
- compressor;
- limiter;
- filtros;
- equalização;
- resampling;
- métricas.

### helix-codecs

Arquitetura de plugins/adapters para codecs permitidos e devidamente licenciados.

Um codec externo **não se torna parte do núcleo Helix apenas porque existe um adapter para ele**.

### helix-protocols

Adaptadores de transporte para protocolos de rádio digital.

O protocolo e o codec são tratados como camadas distintas.

### helix-xlx

Camada destinada à integração controlada com XLX/XLXD.

A primeira estratégia de integração deverá evitar modificações desnecessárias no núcleo do refletor e usar interfaces externas sempre que possível.

---

## Normalização de áudio

Uma das metas principais do Helix é reduzir diferenças perceptíveis de volume entre protocolos.

Em vez de depender apenas de ganhos fixos por codec, o Helix deverá medir o sinal e aplicar processamento dinâmico controlado.

Exemplo conceitual:

```text
decode
  ↓
PCM
  ↓
voice-level detector
  ↓
adaptive gain control
  ↓
dynamic-range compressor
  ↓
soft limiter
  ↓
protocol/output profile
  ↓
encode
```

O projeto deve evitar:

- clipping;
- pumping;
- aumento excessivo de ruído;
- alterações bruscas de volume;
- compressão excessiva;
- latência desnecessária.

---

## Linguagem e desempenho

O núcleo está planejado prioritariamente em **Rust**.

Razões técnicas:

- memory safety;
- zero-cost abstractions;
- ausência de garbage collector obrigatório;
- boa interoperabilidade com C/C++;
- suporte a ARM e x86_64;
- concorrência segura;
- possibilidade de SIMD;
- adequado para serviços Linux de longa duração.

A meta é evitar alocações de memória no caminho crítico de áudio sempre que possível e utilizar buffers pré-alocados.

---

## Codec Helix

Um futuro **Helix Voice Codec (HVC)** poderá ser pesquisado separadamente do motor de transcodificação.

Esse eventual codec deverá:

- ser desenvolvido independentemente;
- possuir especificação própria;
- utilizar bitstream próprio;
- ter parâmetros e algoritmos próprios;
- ser mensurável por testes objetivos;
- não depender internamente de AMBE;
- não ser apresentado como substituto compatível bit-a-bit de um codec proprietário.

Isso significa que equipamentos legados não passam automaticamente a entender HVC.

A compatibilidade com rádios existentes dependerá das capacidades reais de cada protocolo, rádio, gateway e codec utilizado.

---

## Interoperabilidade legada

Compatibilidade de protocolo **não significa** necessariamente compatibilidade de codec.

Exemplo:

```text
Legacy radio
    ↓
Legacy codec
    ↓
authorized/licensed decoder
    ↓
PCM
    ↓
Helix DSP
    ↓
PCM
    ↓
authorized/licensed encoder
    ↓
Legacy destination
```

O componente proprietário/licenciado permanece fora do núcleo Helix.

---

## Referências abertas permitidas

Projetos externos podem ser utilizados como documentação, referência acadêmica ou dependência somente quando sua licença e forma de utilização forem verificadas.

Referências relevantes incluem:

- [Codec2](https://github.com/drowe67/codec2)
- [FreeDV](https://freedv.org/)
- [M17 Project](https://m17project.org/)
- [libm17](https://github.com/M17-Project/libm17)
- [Opus](https://opus-codec.org/)
- [Xiph Opus](https://github.com/xiph/opus)
- [RNNoise](https://github.com/xiph/rnnoise)
- [Rust](https://www.rust-lang.org/)
- [XLXD](https://github.com/LX3JL/xlxd)

A presença de um link nesta seção **não significa que código daquele projeto será incorporado ao Helix**.

Cada dependência deverá passar por revisão de:

1. licença;
2. compatibilidade de distribuição;
3. manutenção;
4. segurança;
5. custo computacional;
6. necessidade real.

---

## Regra de desenvolvimento clean-room

Contribuições ao núcleo Helix não devem incluir:

- código proprietário obtido sem autorização;
- dumps de firmware;
- tabelas privadas;
- documentação confidencial;
- código sem licença clara;
- código copiado de implementações fechadas;
- material obtido por engenharia reversa de produto proprietário com o objetivo de reproduzir seu funcionamento interno.

Ao estudar interoperabilidade, documente **o requisito observável**, não a implementação proprietária.

Exemplo correto:

```text
O sistema precisa receber frames de 20 ms.
```

Exemplo inadequado:

```text
Copiar o algoritmo interno usado pelo produto proprietário para gerar aquele frame.
```

---

## Testes planejados

O desenvolvimento deverá medir, sempre que aplicável:

- latência de encode/decode;
- jitter;
- utilização de CPU;
- memória RSS;
- clipping;
- RMS;
- peak level;
- loudness;
- relação sinal/ruído;
- intelligibility;
- STOI / ESTOI;
- testes auditivos ABX;
- testes prolongados de estabilidade;
- perda e reordenação de pacotes.

Métricas sujeitas a licenciamento, como determinados métodos perceptuais comerciais, somente deverão ser usadas quando houver autorização/licença apropriada.

---

## Integração prevista

### XLX026

O Helix deverá inicialmente funcionar como serviço isolado.

```text
XLXD
  ↕
Helix adapter
  ↕
Helix Voice Engine
```

O objetivo é preservar o funcionamento atual do refletor e permitir rollback simples.

### PU2PNY-OS

Integração prevista:

```text
PU2PNY-OS
   │
   ├── MMDVM / gateways
   │
   └── helix-voice.service
```

O serviço deverá permanecer opcional e não impedir o boot ou a operação básica do hotspot se estiver ausente ou apresentar falha.

---

## Segurança e confiabilidade

O projeto deverá seguir:

- princípio do menor privilégio;
- serviço dedicado;
- hardening via systemd;
- configuração separada do binário;
- logs limitados;
- watchdog;
- health check;
- rollback;
- nenhuma execução como root sem necessidade comprovada;
- testes antes de atualização em produção.

---

## Política de fontes e propriedade intelectual

O Helix pretende manter rastreabilidade de decisões técnicas.

Quando uma fonte externa influenciar diretamente uma decisão de engenharia relevante, ela deverá ser registrada na documentação técnica do projeto.

Este README é uma declaração técnica do processo de desenvolvimento, **não um parecer jurídico**.

Antes de qualquer distribuição comercial envolvendo codecs, patentes, marcas ou componentes proprietários, recomenda-se revisão jurídica especializada.

### Referência do fabricante

Informações sobre produtos e tecnologia AMBE devem ser consultadas diretamente na fonte oficial da DVSI:

- [Digital Voice Systems, Inc.](https://www.dvsinc.com/)

---

## Roadmap inicial

- [ ] especificação HVE v0;
- [ ] estrutura Rust workspace;
- [ ] formato PCM interno;
- [ ] frame scheduler;
- [ ] ring buffers;
- [ ] medidor de nível;
- [ ] AGC;
- [ ] compressor;
- [ ] limiter;
- [ ] resampler;
- [ ] benchmark;
- [ ] testes automatizados;
- [ ] API de plugins;
- [ ] adapter Codec2/M17;
- [ ] adapter XLXD experimental;
- [ ] integração de laboratório com XLX026;
- [ ] integração experimental com PU2PNY-OS;
- [ ] pesquisa separada do Helix Voice Codec.

---

## Princípio do projeto

> **Interoperabilidade não exige copiar uma implementação.**

Helix pretende tratar sistemas legados como interfaces externas e construir um novo núcleo de áudio digital com tecnologia própria, mensurável, documentada e evolutiva.
