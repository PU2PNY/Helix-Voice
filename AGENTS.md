# Instructions for AI coding agents

Before modifying this repository, read and follow in order:
1. PROJECT_START_HERE.md
2. PROJECT_MASTER_SPEC.md
3. PROJECT_RELEASE_STATUS.md
4. PROJECT_TEST_MATRIX.md
5. PROJECT_CHANGELOG.md
6. CLEAN_ROOM.md
7. PATENT_GUARDRAILS.md
8. PROJECT_KNOWN_GOOD.md
9. referenced architecture/integration documents.

GitHub is the persistent technical memory. Do not reconstruct the project from assumptions or chat history.

Rules:
- identify the actual branch/HEAD before editing;
- inspect existing implementation before replacing it;
- preserve approved requirements and verified PASS baselines;
- never claim SW evidence as ENV/HW/PROD;
- never claim compatibility or performance without evidence;
- proprietary codec implementation must not enter Helix core/HVC;
- patent/FTO risk must be researched before implementing a materially similar technical function;
- when patent risk exists, produce a documented design-around and implement independently from first principles;
- do not claim freedom-to-operate, non-infringement, lower power, lower latency or smaller footprint without evidence;
- external dependencies require provenance/license review;
- permanent decisions must update requirements/tests/status/changelog;
- use small changes with rollback;
- do not hide failed tests.

Project-specific source of truth: `PROJECT_START_HERE.md`.

## Autonomous execution rule

When the user provides a sufficiently specified objective, execute it end-to-end with minimum human intervention.

- do not stop at the first build, test, integration or deployment failure;
- diagnose the failure, apply the smallest safe correction, and rerun the relevant validation;
- when a technical uncertainty can be resolved by documentation, source inspection, reproducible experiment or test, resolve it before asking the user;
- preserve working behavior, approved requirements, rollback points and evidence levels while fixing problems;
- do not hide, bypass or relabel failed tests as success;
- do not claim "ready", "complete", "compatible", "better" or "production" until the required evidence exists;
- ask the user only when progress is blocked by information/action that cannot be obtained or safely inferred, such as unavailable credentials, inaccessible hardware, an irreversible business/product decision, or authorization that only the user can grant;
- complete as much as possible in the current execution/session; never imply background work or future completion that is not actually running.


Patent register gate:
- read PROJECT_PATENT_REGISTER.md before implementing/releasing a relevant feature;
- every new relevant feature starts UNREVIEWED and production-blocked;
- update the register after patent review;
- never treat REVIEWED-LOWER-RISK as legal clearance.
## Habilidades de agentes — ativacao condicional

Antes de desenvolver, identificar a natureza da tarefa e consultar os SKILL.md das ferramentas instaladas que realmente se aplicam:

- **Modern Web Guidance** (`modern-web-guidance`): consultar primeiro em tarefas de HTML, CSS, JavaScript no navegador, formularios, acessibilidade, Web Vitals e compatibilidade Baseline. Usar `search` e depois `retrieve` do ID encontrado; nao ativar para backend, radio, CI, banco de dados ou scripts de infraestrutura.
- **Taste Skill** (`design-taste-frontend`): ativar em criacao/redesign/auditoria visual de sites, landing pages e paginas comerciais, juntamente com Modern Web Guidance se necessario. Nao aplicar seu kit automaticamente em paineis operacionais existentes; usar previews/branches e checagens em navegador com Node 22+ e Chrome/Chromium.
- **Outras skills instaladas**: verificar descricoes e escolher somente quando correspondam a tarefa; por exemplo, Playwright em testes funcionais, ferramentas de design em auditorias de UI. Nao executar indiscriminadamente todos os scripts, nao assumir que ferramentas estao instaladas em outro computador.
- Prioridade: especificacao e requisitos de seguranca do projeto, dados verificados, compatibilidade, acessibilidade e intencao do usuario prevalecem sobre templates. Nao inventar valores, numeros, contatos ou alterar marcas sem pedido. Nao instalar novas dependencias ou publicar em producao apenas por ler uma skill.

Fontes: https://github.com/GoogleChrome/modern-web-guidance e https://github.com/Leonxlnx/taste-skill.
