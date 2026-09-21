# Operations

Helix Voice is currently **research software**, not a production service.

## Current executable

`helix-daemon` is only an initial research executable. It does not perform production transcoding.

## Build validation

Canonical software gate:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

GitHub Actions repeats these checks on pushes to `main` and pull requests.

## Deployment rule

Do not install Helix into production XLX026 or enable it by default in PU2PNY-OS until the relevant entries in `PROJECT_TEST_MATRIX.md` have ENV/HW evidence and the gates in the integration documents pass.

## Logging

Future daemon logging must:
- avoid per-sample logs;
- avoid secrets;
- expose version/build where practical;
- record last meaningful error;
- remain bounded/rotated.

## Health

Future health reporting should distinguish:
- process alive;
- DSP ready;
- adapter ready;
- external backend ready;
- degraded;
- error.

A process being alive is not proof that transcoding works.

## Configuration

Configuration format is not finalized. When introduced it must be versioned, validated, documented and included in recovery procedures.

## Rollback

See `docs/RECOVERY.md`.
