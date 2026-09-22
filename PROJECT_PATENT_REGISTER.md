# PROJECT_PATENT_REGISTER — Helix Voice

Atualizado: 2026-09-22

Este registro é o gate central de propriedade intelectual do projeto.

## Regra

Um componente pode estar tecnicamente PASS e continuar **bloqueado para produção/comercialização** por patente/FTO.

Status:
- **UNREVIEWED** — pesquisa específica ainda não concluída; produção bloqueada;
- **UNCERTAIN** — pesquisa encontrou risco/overlap ou incerteza material; produção bloqueada;
- **BLOCKED** — conflito técnico relevante não resolvido; não implementar/promover;
- **REVIEWED-LOWER-RISK** — review técnico não encontrou overlap material evidente no escopo pesquisado, mas isso não é parecer jurídico nem garantia de FTO;
- **LEGAL-CLEARED** — somente após revisão jurídica qualificada no território/uso definido.

Por padrão, qualquer feature nova começa em **UNREVIEWED**.

## Registro atual

| Componente | Função | Review | Status técnico | Produção |
|---|---|---|---|---|
| HVC v0 | codec paramétrico experimental | docs/patent-reviews/hvc-v0.md | UNCERTAIN | BLOQUEADA |
| HVC next | próxima arquitetura clean-slate | HVC_NEXT.md + reviews futuros | UNREVIEWED | BLOQUEADA |
| AdaptiveGain | controle de nível por RMS | docs/patent-reviews/dsp-level-control.md | UNCERTAIN | BLOQUEADA |
| SoftLimiter | limitação de pico sem look-ahead | docs/patent-reviews/dsp-level-control.md | UNCERTAIN | BLOQUEADA |
| Decimator2 | 16 kHz → 8 kHz windowed-sinc FIR | docs/patent-reviews/decimator2.md | REVIEWED-LOWER-RISK | revisão jurídica ainda necessária antes de claim comercial |
| HVC PLC | concealment por repetição atenuada bounded | docs/patent-reviews/hvc-plc.md | UNCERTAIN | BLOQUEADA |
| CRC-8/frame validation | integridade de pacote | review de utilidade pendente | UNREVIEWED | BLOQUEADA |
| XLXD control adapter | interoperabilidade pública | review protocolo/IP pendente | UNREVIEWED | BLOQUEADA |
| helix-daemon | processo/health/fail-closed | sem risco específico identificado até agora | UNREVIEWED | BLOQUEADA |
| PcmFrame/core | representação interna PCM | sem risco específico identificado até agora | UNREVIEWED | BLOQUEADA |

## Gate de merge/release

Para feature com relevância funcional:
1. requisito registrado;
2. source/provenance registrada;
3. busca patentária proporcional ao risco;
4. review em docs/patent-reviews/;
5. design-around quando necessário;
6. testes/benchmarks;
7. status atualizado aqui;
8. produção somente com nível de confiança apropriado e, quando houver uso comercial relevante, revisão jurídica qualificada.

## Regra conservadora

- UNREVIEWED, UNCERTAIN e BLOCKED nunca significam "infringe"; significam **não temos evidência suficiente para liberar**.
- REVIEWED-LOWER-RISK não significa legal clearance.
- Busca negativa não prova FTO.
- Patente expirada/abandonada pode continuar útil como estado da técnica, mas status deve ser confirmado.
- Software open source pode trazer direitos/licenças de copyright sem necessariamente resolver toda questão patentária.

## Prioridade de reviews

1. HVC next candidates;
2. PLC/FEC;
3. resampler;
4. XLXD/protocol adapter;
5. AGC/limiter — review técnico já iniciado;
6. demais componentes antes de qualquer release de produção.
