# Engineering Decisions

## ADR-001 — Rust-first core
**Estado:** aprovado para a baseline inicial.

O núcleo inicial usa Rust, sem `unsafe`, visando previsibilidade, segurança de memória e baixo overhead. Integrações C/C++ futuras exigirão fronteira explícita e revisão.

## ADR-002 — PCM como domínio DSP
**Estado:** aprovado.

O DSP não opera diretamente sobre conceitos de protocolo. Codec adapters convertem de/para PCM e o DSP permanece independente.

## ADR-003 — HVE e HVC separados
**Estado:** aprovado.

HVE é o motor de processamento/transcoding. HVC é pesquisa de codec próprio e não deve ser misturado com adapters de interoperabilidade legada.

## ADR-004 — Proprietário somente fora do núcleo
**Estado:** obrigatório.

Codec proprietário, quando necessário e legalmente autorizado, deve existir atrás de backend externo. Não é incorporado ao HVE core/HVC.

## ADR-005 — XLX por processo separado
**Estado:** aprovado para primeira integração.

Não começar reescrevendo XLXD. Usar processo separado, observe-only primeiro, com rollback.

## ADR-006 — PU2PNY opcional
**Estado:** aprovado.

Falha/ausência do Helix não pode impedir funcionamento básico do PU2PNY-OS.

## ADR-007 — Sem licença final automática
**Estado:** pendente.

O repositório é público, mas uma licença definitiva do código ainda não foi escolhida. A decisão deve ser deliberada antes de release pública de produção/contribuições externas.
