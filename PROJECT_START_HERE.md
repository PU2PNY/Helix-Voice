# PROJECT_START_HERE — Helix Voice

Este é o ponto inicial obrigatório para qualquer pessoa ou IA que vá analisar, programar, corrigir, integrar, publicar ou fazer deploy do Helix Voice.

## Projeto

- **Nome:** Helix Voice
- **Repositório:** `PU2PNY/Helix-Voice`
- **Branch padrão e atualmente usada no desenvolvimento:** `main`
- **Baseline anterior à implantação desta governança:** `62ee9b83ee72ff7ba721413f9f3bdd56f620c0c5`
- **Ponto de retorno:** `baseline/pre-governance-2026-09-21`
- **Estágio:** DEV / pesquisa, não produção

## Leia nesta ordem

1. `PROJECT_MASTER_SPEC.md` — requisitos oficiais.
2. `PROJECT_RELEASE_STATUS.md` — estado técnico atual.
3. `PROJECT_TEST_MATRIX.md` — o que foi realmente testado.
4. `PROJECT_CHANGELOG.md` — evolução e decisões.
5. `CLEAN_ROOM.md` — regra obrigatória de desenvolvimento independente.
6. `ARCHITECTURE.md` — arquitetura técnica.
7. `SPEC.md` — especificação HVE v0 de baixo nível.
8. `ROADMAP.md` — sequência planejada.
9. `THIRD_PARTY.md` — dependências/referências externas.
10. `docs/DECISIONS.md`, `docs/XLX-INTEGRATION.md`, `docs/PU2PNY-INTEGRATION.md`, `docs/AUDIO-QUALITY.md`, `docs/RECOVERY.md`.

## Regra para qualquer IA

Antes de propor ou executar mudanças:

1. confirme a branch real;
2. leia os documentos acima;
3. leia a implementação atual afetada;
4. preserve requisitos aprovados e testes PASS;
5. não confunda `SW` com `HW`, `ENV` ou `PROD`;
6. nunca afirme compatibilidade, qualidade ou desempenho sem evidência;
7. não copie nem reimplemente material proprietário;
8. registre mudança permanente em requisitos/testes/status/changelog;
9. use commits pequenos e mantenha rollback;
10. se houver conflito entre conversa e documentação versionada, pare e reconcilie explicitamente.

## Fonte da verdade

A conversa é ambiente de trabalho. O GitHub e estes documentos são a memória técnica persistente.

A implementação real continua sendo autoridade sobre o que o software efetivamente faz. Documentação que contradiga código ou teste deve ser corrigida, não presumida verdadeira.
