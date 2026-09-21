# Instructions for AI coding agents

Before modifying this repository, read and follow in order:
1. PROJECT_START_HERE.md
2. PROJECT_MASTER_SPEC.md
3. PROJECT_RELEASE_STATUS.md
4. PROJECT_TEST_MATRIX.md
5. PROJECT_CHANGELOG.md
6. CLEAN_ROOM.md
7. referenced architecture/integration documents.

GitHub is the persistent technical memory. Do not reconstruct the project from assumptions or chat history.

Rules:
- identify the actual branch/HEAD before editing;
- inspect existing implementation before replacing it;
- preserve approved requirements and verified PASS baselines;
- never claim SW evidence as ENV/HW/PROD;
- never claim compatibility or performance without evidence;
- proprietary codec implementation must not enter Helix core/HVC;
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
