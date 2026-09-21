# Helix Voice repository instructions

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

Do not generate or introduce proprietary AMBE/AMBE+2 implementation code. Follow `CLEAN_ROOM.md`.
