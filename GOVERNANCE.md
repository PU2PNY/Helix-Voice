# Governance

## Project scope

Helix Voice is an independent digital-voice processing and transcoding research project.

The repository distinguishes:

- HVE — Helix Voice Engine;
- HVC — future experimental Helix Voice Codec;
- protocol adapters;
- external codec adapters;
- integration tooling.

## Decision hierarchy

Engineering decisions prioritize:

1. legal/provenance safety;
2. correctness;
3. audio integrity;
4. real-time stability;
5. low resource consumption;
6. interoperability;
7. convenience.

## Compatibility claims

A feature is considered supported only after automated tests and the relevant physical/integration test are documented.

## Production changes

XLX026 and PU2PNY-OS integrations must be tested outside production first and deployed with backup and rollback.

## Breaking changes

Public APIs and wire formats will use explicit versions once the first stable interface is published.
