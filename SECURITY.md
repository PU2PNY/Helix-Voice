# Security Policy

Helix Voice is currently research software and is not yet production-ready.

## Security model

The intended production architecture uses:

- a dedicated unprivileged service account;
- minimal Linux capabilities;
- systemd hardening;
- bounded input buffers and queues;
- strict packet length validation;
- no shell execution from packet data;
- no web administrative shell;
- configuration outside the binary;
- local-only control interfaces by default.

## Reporting

Do not publish exploitable security details before maintainers have had an opportunity to investigate.

When reporting an issue, include:

- affected commit/version;
- architecture;
- reproduction steps;
- expected behavior;
- actual behavior;
- impact;
- logs with secrets removed.

## Secrets

Never commit:

- passwords;
- API keys;
- private certificates;
- production HMAC keys;
- private XLX administration credentials;
- personally identifying test datasets.

## Fuzzing targets

Protocol decoders, packet parsers and configuration parsers are expected fuzzing targets before production release.
