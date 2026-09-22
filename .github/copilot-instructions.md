# Helix Voice repository instructions

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
- external dependencies require provenance/license review;
- permanent decisions must update requirements/tests/status/changelog;
- use small changes with rollback;
- do not hide failed tests.

Do not generate or introduce proprietary AMBE/AMBE+2 implementation code. Follow `CLEAN_ROOM.md`.


Patent rule:
- this applies to the entire repository;
- when a feature touches patented/proprietary technology, research the patent landscape before implementation;
- use claims only to define what must be avoided, never as an implementation recipe;
- design from first principles and public-domain/open sources;
- prefer demonstrably different representation, framing, state, control and data structures where technically reasonable;
- record per-feature review under docs/patent-reviews/;
- FTO technical status UNCERTAIN/BLOCKED means no production/commercial promotion;
- do not claim non-infringement or freedom-to-operate without qualified legal review.
