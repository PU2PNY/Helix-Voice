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
