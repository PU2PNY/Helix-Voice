# Audio Quality Contract

Audio quality is a release criterion.

## Primary goals

- intelligible speech;
- consistent perceived level across supported paths;
- no avoidable clipping;
- no AGC pumping;
- no strong noise-floor breathing;
- no unnecessary spectral coloration;
- predictable latency.

## Measurements

For every supported path record:

- input RMS and peak;
- output RMS and peak;
- clipping count;
- gain range applied;
- processing latency;
- CPU;
- memory;
- packet loss/jitter when networked.

Where legally and technically appropriate, research metrics may include STOI/ESTOI and licensed perceptual metrics.

## Listening tests

Objective metrics do not replace listening tests.

Use level-matched A/B or ABX tests with multiple speakers and conditions. Test material must have redistribution rights.

## AGC

AGC is per stream.

It must not boost silence or stationary background noise without bound. Gain must move smoothly and be capped.

## Release rule

A new DSP setting is not "better" because it is louder. Loudness, distortion, intelligibility and fatigue must be evaluated together.

## XLX cross-mode profile
O PCM bridge XLX trabalha sobre voz já decodificada de codecs legados e não deve usar o range agressivo do protótipo genérico como padrão operacional. O perfil XLX deve priorizar transparência: limite de ganho aproximado de -6 dB a +3,5 dB, recuperação de ganho lenta, atenuação mais rápida, silêncio protegido e limiter com knee alto. Esse perfil é específico da integração XLX; o `DspChain::default()` e os baselines HVC permanecem separados.

Qualquer afirmação de melhora auditiva continua exigindo escuta A/B/ABX ou validação de operador; métricas SW/ENV não substituem áudio real.
