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
